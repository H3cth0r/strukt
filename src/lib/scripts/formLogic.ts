import { writable, get } from 'svelte/store';
// 1. Import the Wasm functions and the initializer
import init, { generate_plan, execute_plan } from '$lib/pkg/json_merger_lib.js';

// --- WASM INITIALIZATION ---
// We need to initialize the Wasm module once when the app loads.
// This self-invoking async function handles it.
(async () => {
try {
	await init();
	console.log('Wasm module initialized successfully.');
} catch (e) {
	console.error('Error initializing Wasm module:', e);
}
})();

// --- STORES ---
export const uploadedFiles = writable<File[]>([]);
export const tags = writable<string[]>([]);
export const showOutputView = writable<boolean>(false);

// 2. Rename `processOutput` to be more specific and add a new store for the final result.
export const planOutput = writable<string>(''); // Will hold the generated plan
export const mergedOutput = writable<string>(''); // Will hold the final merged JSON

// --- HELPER FUNCTIONS ---
function readFileAsString(file: File): Promise<string> {
return new Promise((resolve, reject) => {
	const reader = new FileReader();
	reader.onload = () => resolve(reader.result as string);
	reader.onerror = () => reject(reader.error);
	reader.readAsText(file);
});
}

// --- CORE LOGIC FUNCTIONS ---

export function handleFileUpload(files: File[]) {
uploadedFiles.update((currentFiles) => [...currentFiles, ...files]);
}

/**
* Calls the Wasm module to generate the merge plan.
*/
export async function generatePlan() {
const currentFiles = get(uploadedFiles);
const currentTags = get(tags);

if (currentFiles.length !== 2) {
	alert('Please upload exactly two JSON files.');
	return;
}
if (currentTags.length === 0) {
	alert('Please add at least one key to merge on.');
	return;
}

console.log('--- Calling Wasm to Generate Plan ---');
console.log('Tags to merge on:', currentTags);

try {
	const fileContents = await Promise.all(currentFiles.map(readFileAsString));
	const [jsonString1, jsonString2] = fileContents;

	// 3. Call the actual Wasm function
	const plan = generate_plan(jsonString1, jsonString2, currentTags);

	// 4. Store the plan in its dedicated store and switch views
	planOutput.set(plan);
	showOutputView.set(true);
} catch (error) {
	console.error('Error generating plan from Wasm:', error);
	alert(`An error occurred while generating the plan: ${error}`);
}
}

/**
* Calls the Wasm module to execute the plan and produce the final merge.
*/
export function executePlan() {
const plan = get(planOutput);
if (!plan) {
	alert('No plan available to execute.');
	return;
}

console.log('--- Calling Wasm to Execute Plan ---');

try {
	// 5. Call the Wasm function with the plan string
	const result = execute_plan(plan);

	// 6. Store the final result in its store, which will update the UI
	mergedOutput.set(result);
} catch (error) {
	console.error('Error executing plan from Wasm:', error);
	alert(`An error occurred while executing the plan: ${error}`);
}
}

/**
* Resets all stores to their initial state.
*/
export function handleReset() {
uploadedFiles.set([]);
tags.set([]);
planOutput.set('');
mergedOutput.set(''); // 7. Also reset the merged output
showOutputView.set(false);

console.log('Inputs have been reset.');
}
