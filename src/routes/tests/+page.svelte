<script>
  import { onMount } from 'svelte';
  import { JsonEditor } from '$lib/components';
	import { FileDropZone } from "$lib/components/ui/file-drop-zone";

  const jsonData = {
   _status: 'merge',
   _value: {
    advancedConfig: {
     _status: 'merge',
     _value: {
      disableCustomSizes: { _status: 'keep', _value: null }
     }
    },
    animation: { _status: 'merge', _value: {} },
    animationConfig: {
     _status: 'merge',
     _value: {
      autoPlay: { _status: 'keep', _value: null },
      enableExperimentalAnimationArea: { _status: 'keep', _value: false },
      hideMarkersForUser: { _status: 'keep', _value: null },
      loop: { _status: 'keep', _value: null },
      totalDuration: { _status: 'keep', _value: null }
     }
    },
    categories: { _status: 'merge', _value: {} },
    containerClasses: {
     _status: 'overwrite',
     _value: 'brand-{{variables.brand}},headline-span-{{variables.headlineSpanDisplay}},headline-horizontal-{{ fn( return state.variables.headlinesPosition[state.variables.tactic]?.[state.variables.brand]?.[state.variables.mealType]?.[state.variables.zone]?.headlinePositionHorizontal) }},headline-vertical-{{ fn( return state.variables.headlinesPosition[state.variables.tactic]?.[state.variables.brand]?.[state.variables.mealType]?.[state.variables.zone]?.headlinePositionVertical) }},{{variables.product}},{{variables.mealType}},logo-with-text,headline-color-{{variables.varHeadlineColor}},{{variables.zone}},{{variables.subChannel}},{{variables.tactic}},{{variables.price}},{{variables.mealTypeVariant}},{{variables.fileExtension}},meal-overlay-{{variables.mealOverlayEnabled}}',
     _old_value: 'brand-{{variables.brand}},headline-span-{{variables.headlineSpanDisplay}},headline-horizontal-{{ fn( return state.variables.headlinesPosition[state.variables.tactic]?.[state.variables.brand]?.[state.variables.mealType]?.[state.variables.zone]?.headlinePositionHorizontal) }},headline-vertical-{{ fn( return state.variables.headlinesPosition[state.variables.tactic]?.[state.variables.brand]?.[state.variables.mealType]?.[state.variables.zone]?.headlinePositionVertical) }},{{variables.product}},{{variables.mealType}},logo-with-text,headline-color-{{variables.varHeadlineColor}},{{variables.zone}},{{variables.subChannel}},{{variables.tactic}},{{variables.price}},{{variables.mealTypeVariant}},{{variables.fileExtension}}'
    },
    createdByUser: {
     _status: 'merge',
     _value: {
      fullName: { _status: 'overwrite', _value: 'Alan Zuniga Ramos', _old_value: 'Joze Rozman' }
     }
    },
    creativePackageBusinessIds: { _status: 'merge', _value: [] }
   }
  };
  const jsonString = JSON.stringify(jsonData, null, 2);
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

<FileDropZone/>

<JsonEditor>
  <pre> { jsonString } </pre>
</JsonEditor>
