<script lang="ts">
import { NavBar, TagInput } from '$lib/components';
import * as Card from '$lib/components/ui/card/index.js';
import { FileDropZone } from '$lib/components/ui/file-drop-zone';
import { Button } from '$lib/components/ui/button/index.js';

// 1. Import the new stores for view management and output data.
import {
	uploadedFiles,
	tags,
	handleFileUpload,
	generatePlan,
	handleReset,
	showOutputView,
	processOutput
} from '$lib/scripts/formLogic';
</script>

<NavBar />

<!-- 2. Use an if/else block to switch between input and output views -->
{#if !$showOutputView}
<!-- INPUT VIEW (Original UI) -->
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
				<FileDropZone
					maxFiles={2}
					accept="application/json"
					fileCount={$uploadedFiles.length}
					onUpload={handleFileUpload}
					onFileRejected={(e) => console.warn('File rejected:', e.reason)}
				/>
				{#if $uploadedFiles.length > 0}
					<div class="mt-2 text-sm">
						<p class="font-medium">Uploaded files:</p>
						<ul class="list-disc pl-5 text-muted-foreground">
							{#each $uploadedFiles as file}
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
				<TagInput bind:tags={$tags} />
			</div>
		</Card.Content>

		<Card.Footer class="flex justify-end gap-x-4">
			<Button variant="outline" onclick={handleReset}>Reset</Button>
			<Button onclick={generatePlan}>Process</Button>
		</Card.Footer>
	</Card.Root>
</div>
{:else}
<!-- OUTPUT VIEW (New UI) -->
<div class="flex w-full h-[90vh] flex-col items-center justify-center p-4 gap-y-6">
	<!-- Container for the two parallel cards -->
	<div class="flex w-full max-w-7xl gap-x-6" style="height: 85%;">
		<!-- Left Card: Process Output -->
		<Card.Root class="w-1/2 h-full flex flex-col">
			<Card.Header>
				<Card.Title class="text-2xl">Process Output</Card.Title>
			</Card.Header>
			<Card.Content class="flex-grow overflow-auto">
				<!-- Use <pre> to preserve formatting of the output string -->
				<pre class="text-sm">{$processOutput}</pre>
			</Card.Content>
		</Card.Root>

		<!-- Right Card: Placeholder -->
		<Card.Root class="w-1/2 h-full flex flex-col">
			<Card.Header>
				<Card.Title class="text-2xl">Merge Preview</Card.Title>
			</Card.Header>
			<Card.Content>
				<!-- This area is intentionally blank for now -->
			</Card.Content>
		</Card.Root>
	</div>

	<!-- Action Button to go back -->
	<div class="flex w-full max-w-7xl justify-end">
		<Button onclick={handleReset}>Start Over</Button>
	</div>
</div>
{/if}
