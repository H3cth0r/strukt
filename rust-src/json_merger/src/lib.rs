use anyhow::Result; // Removed unused `Context`
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator. This is done to produce a smaller .wasm file.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This function is called when the wasm module is instantiated.
// It sets up the panic hook to log errors to the browser console.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  #[cfg(feature = "console_error_panic_hook")]
  console_error_panic_hook::set_once();
  Ok(())
}

// ... (The rest of your code is perfect, no changes needed there) ...
// === DATA STRUCTURES & HELPERS ===
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Status {
  Merge,
  Keep,
  Add,
  Overwrite,
  Delete,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PlanNode {
  _status: Status,
  _value: Value,
  #[serde(skip_serializing_if = "Option::is_none")]
  _old_value: Option<Value>,
}

fn get_item_identity<'a>(item: &'a Value, identity_keys: &[String]) -> Option<&'a str> {
  if !item.is_object() {
      return None;
  }
  for key in identity_keys {
      if let Some(value) = item.get(key) {
          if let Some(s) = value.as_str() {
              return Some(s);
          }
      }
  }
  None
}

// === PLAN GENERATION ===
fn create_plan(obj1: &Value, obj2: &Value, identity_keys: &[String]) -> PlanNode {
  match (obj1, obj2) {
      (Value::Object(map1), Value::Object(map2)) => {
          let mut plan_value = serde_json::Map::new();
          let all_keys: BTreeMap<_, _> = map1
              .keys()
              .chain(map2.keys())
              .map(|k| (k.clone(), ()))
              .collect();

          for key in all_keys.keys() {
              let v1 = map1.get(key);
              let v2 = map2.get(key);

              let child_plan = match (v1, v2) {
                  (Some(val1), Some(val2)) => create_plan(val1, val2, identity_keys),
                  (Some(val1), None) => create_scaffold_plan(val1, Status::Keep),
                  (None, Some(val2)) => create_scaffold_plan(val2, Status::Add),
                  (None, None) => unreachable!(),
              };
              plan_value.insert(key.clone(), serde_json::to_value(child_plan).unwrap());
          }

          PlanNode {
              _status: Status::Merge,
              _value: Value::Object(plan_value),
              _old_value: None,
          }
      }
      (Value::Array(arr1), Value::Array(arr2)) => {
          let mut plan_value = vec![];
          let items1_map: HashMap<_, _> = arr1
              .iter()
              .filter_map(|item| get_item_identity(item, identity_keys).map(|id| (id, item)))
              .collect();
          let items2_map: HashMap<_, _> = arr2
              .iter()
              .filter_map(|item| get_item_identity(item, identity_keys).map(|id| (id, item)))
              .collect();

          for item1 in arr1 {
              let plan_node = if let Some(id) = get_item_identity(item1, identity_keys) {
                  if let Some(item2) = items2_map.get(id) {
                      create_plan(item1, item2, identity_keys)
                  } else {
                      create_scaffold_plan(item1, Status::Keep)
                  }
              } else {
                  create_scaffold_plan(item1, Status::Keep)
              };
              plan_value.push(serde_json::to_value(plan_node).unwrap());
          }

          for item2 in arr2 {
              if get_item_identity(item2, identity_keys).map_or(true, |id| !items1_map.contains_key(id)) {
                  let plan_node = create_scaffold_plan(item2, Status::Add);
                  plan_value.push(serde_json::to_value(plan_node).unwrap());
              }
          }

          PlanNode {
              _status: Status::Merge,
              _value: Value::Array(plan_value),
              _old_value: None,
          }
      }
      _ => {
          if obj1 == obj2 {
              PlanNode {
                  _status: Status::Keep,
                  _value: obj1.clone(),
                  _old_value: None,
              }
          } else {
              PlanNode {
                  _status: Status::Overwrite,
                  _value: obj2.clone(),
                  _old_value: Some(obj1.clone()),
              }
          }
      }
  }
}

fn create_scaffold_plan(obj: &Value, status: Status) -> PlanNode {
  let value = match obj {
      Value::Object(map) => {
          let new_map = map
              .iter()
              .map(|(k, v)| {
                  let child_plan = create_scaffold_plan(v, status.clone());
                  (k.clone(), serde_json::to_value(child_plan).unwrap())
              })
              .collect();
          Value::Object(new_map)
      }
      Value::Array(arr) => {
          let new_arr = arr
              .iter()
              .map(|v| {
                  let child_plan = create_scaffold_plan(v, status.clone());
                  serde_json::to_value(child_plan).unwrap()
              })
              .collect();
          Value::Array(new_arr)
      }
      _ => obj.clone(),
  };

  PlanNode {
      _status: status,
      _value: value,
      _old_value: None,
  }
}

// === PLAN APPLICATION ===
fn apply_plan(plan_value: &Value) -> Result<Option<Value>> {
  let plan_node: PlanNode = match serde_json::from_value(plan_value.clone()) {
      Ok(node) => node,
      Err(_) => return Ok(Some(plan_value.clone())),
  };

  if plan_node._status == Status::Delete {
      return Ok(None);
  }

  match plan_node._value {
      Value::Object(map) => {
          let mut result_map = serde_json::Map::new();
          for (key, child_plan_value) in map {
              if let Some(child_result) = apply_plan(&child_plan_value)? {
                  result_map.insert(key, child_result);
              }
          }
          Ok(Some(Value::Object(result_map)))
      }
      Value::Array(arr) => {
          let mut result_list = Vec::new();
          for item_plan_value in arr {
              if let Some(item_result) = apply_plan(&item_plan_value)? {
                  result_list.push(item_result);
              }
          }
          Ok(Some(Value::Array(result_list)))
      }
      primitive_value => Ok(Some(primitive_value)),
  }
}

// === WASM BINDINGS ===
#[wasm_bindgen]
pub fn generate_plan(
  json1_str: &str,
  json2_str: &str,
  identity_keys: Vec<String>,
) -> Result<String, JsValue> {
  let json1: Value = serde_json::from_str(json1_str)
      .map_err(|e| JsValue::from_str(&format!("Failed to parse json1: {}", e)))?;
  let json2: Value = serde_json::from_str(json2_str)
      .map_err(|e| JsValue::from_str(&format!("Failed to parse json2: {}", e)))?;

  let plan = create_plan(&json1, &json2, &identity_keys);

  serde_json::to_string_pretty(&plan)
      .map_err(|e| JsValue::from_str(&format!("Failed to serialize plan: {}", e)))
}

#[wasm_bindgen]
pub fn execute_plan(plan_str: &str) -> Result<String, JsValue> {
  let plan_data: Value = serde_json::from_str(plan_str)
      .map_err(|e| JsValue::from_str(&format!("Failed to parse plan: {}", e)))?;

  match apply_plan(&plan_data) {
      Ok(Some(merged_data)) => serde_json::to_string_pretty(&merged_data)
          .map_err(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e))),
      Ok(None) => Ok("{}".to_string()),
      Err(e) => Err(JsValue::from_str(&format!("Failed to apply plan: {}", e))),
  }
}
