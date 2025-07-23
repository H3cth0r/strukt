<script lang="ts">
import { NavBar, TagInput } from '$lib/components';
import * as Card from '$lib/components/ui/card/index.js';
import { FileDropZone } from '$lib/components/ui/file-drop-zone';
import { Button } from '$lib/components/ui/button/index.js';

// --- STATE MANAGEMENT ---
// 1. State for the uploaded files.
let uploadedFiles: File[] = [];
// 2. State for the tags from the TagInput component.
let tags: string[] = [];

// --- LOGIC FOR FILE HANDLING ---
/**
 * Handles the onUpload event from the FileDropZone.
 * It adds the newly uploaded files to our state.
 */
function handleFileUpload(files: File[]) {
	// Appends new files to the existing array, respecting the maxFiles limit.
	uploadedFiles = [...uploadedFiles, ...files];
}

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

// --- LOGIC FOR BUTTONS ---
/**
 * This is the main function called by the "Process" button.
 * It reads the file contents and gets the tags.
 */
async function generatePlan() {
	// Basic validation
	if (uploadedFiles.length !== 2) {
		alert('Please upload exactly two JSON files.');
		return;
	}
	if (tags.length === 0) {
		alert('Please add at least one key to merge on.');
		return;
	}

	console.log('--- Generating Plan ---');
	console.log('Tags to merge on:', tags);

	try {
		// Read both files concurrently and wait for them to finish
		const fileContents = await Promise.all(uploadedFiles.map(readFileAsString));

		const [jsonString1, jsonString2] = fileContents;

		console.log('File 1 Content:', jsonString1);
		console.log('File 2 Content:', jsonString2);

		// Later, you can parse and process these strings:
		// const data1 = JSON.parse(jsonString1);
		// const data2 = JSON.parse(jsonString2);
		// ... your merge logic here ...
	} catch (error) {
		console.error('Error reading files:', error);
		alert('There was an error reading one of the files. Make sure they are valid text-based files.');
	}
}

/**
 * Handles the "Reset" button click.
 * It clears all state, resetting the form.
 */
function handleReset() {
	uploadedFiles = [];
	tags = [];
	console.log('Inputs have been reset.');
}
</script>

<NavBar />

<div class="flex w-full h-[90vh] items-center justify-center p-4">
<Card.Root class="w-full max-w-4xl">
	<Card.Header>
		<Card.Title class="text-3xl lg:text-4xl">Base Inputs</Card.Title>
	</Card.Header>

	<Card.Content class="flex flex-col gap-y-8">
		<!-- Group 1: File Drop Zone -->
		<div class="flex flex-col gap-y-3">
			<p class="text-sm text-muted-foreground">
				Select the two JSON files that you want to merge to continue.
			</p>
			<!--
         - maxFiles={2} restricts uploads to 2 files.
         - accept="application/json" restricts to JSON file types.
         - fileCount tells the component how many files are already uploaded.
         - onUpload is the event handler that receives the files.
       -->
			<FileDropZone
				maxFiles={2}
				accept="application/json"
				fileCount={uploadedFiles.length}
				onUpload={handleFileUpload}
				onFileRejected={(e) => console.warn('File rejected:', e.reason)}
			/>
			<!-- Added a simple display for uploaded files for better UX -->
			{#if uploadedFiles.length > 0}
				<div class="mt-2 text-sm">
					<p class="font-medium">Uploaded files:</p>
					<ul class="list-disc pl-5 text-muted-foreground">
						{#each uploadedFiles as file}
							<li>{file.name}</li>
						{/each}
					</ul>
				</div>
			{/if}
		</div>

		<!-- Group 2: Tag Input -->
		<div class="flex flex-col gap-y-3">
			<p class="text-sm text-muted-foreground">
				Type the JSON keys you want to merge the files on.
			</p>
			<!--
         - bind:tags creates a two-way binding with our `tags` array.
       -->
			<TagInput bind:tags />
		</div>
	</Card.Content>

	<Card.Footer class="flex justify-end gap-x-4">
		<!--
       - on:click handlers are added to trigger our logic.
     -->
		<Button variant="outline" onclick={handleReset}>Reset</Button>
		<Button onclick={generatePlan}>Process</Button>
	</Card.Footer>
</Card.Root>
</div>
