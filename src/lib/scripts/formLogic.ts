import { writable, get } from 'svelte/store';

// --- STORES ---
export const uploadedFiles = writable<File[]>([]);
export const tags = writable<string[]>([]);

// 1. Add new stores for managing the output view and its content.
export const showOutputView = writable<boolean>(false);
export const processOutput = writable<string>('');

// --- HELPER FUNCTIONS ---
/**
* A helper function to read a file's content as a string.
* It returns a Promise that resolves with the file's text content.
*/
function readFileAsString(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = () => resolve(reader.result as string);
      reader.onerror = () => reject(reader.error);
      reader.readAsText(file);
  });
}

// --- CORE LOGIC FUNCTIONS ---

/**
* Handles adding new files to our store.
*/
export function handleFileUpload(files: File[]) {
  uploadedFiles.update(currentFiles => [...currentFiles, ...files]);
}

/**
* This is the main function called by the "Process" button.
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

  console.log('--- Generating Plan ---');
  console.log('Tags to merge on:', currentTags);

  try {
      const fileContents = await Promise.all(currentFiles.map(readFileAsString));
      const [jsonString1, jsonString2] = fileContents;

      // 2. Instead of just logging, format the output and store it.
      const output = `--- PROCESSING COMPLETE ---

Tags to merge on:
${JSON.stringify(currentTags, null, 2)}

--- FILE 1: ${currentFiles[0].name} ---
${jsonString1}

--- FILE 2: ${currentFiles[1].name} ---
${jsonString2}
`;
      processOutput.set(output);

      // 3. Set the flag to switch to the output view.
      showOutputView.set(true);

  } catch (error) {
      console.error('Error reading files:', error);
      alert('There was an error reading one of the files.');
  }
}

/**
* Handles the "Reset" button click.
*/
export function handleReset() {
  uploadedFiles.set([]);
  tags.set([]);
  
  // 4. Also reset the output state when the user wants to start over.
  processOutput.set('');
  showOutputView.set(false);

  console.log('Inputs have been reset.');
}
