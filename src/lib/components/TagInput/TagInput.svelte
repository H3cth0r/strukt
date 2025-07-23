<!-- src/lib/components/TagInput.svelte -->
<script lang="ts">
export let tags: string[] = [];

let currentTag = '';

function handleKeydown(event: KeyboardEvent) {
  // Handle creating a new tag on "Enter"
  if (event.key === 'Enter') {
    event.preventDefault();
    const newTag = currentTag.trim();

    if (newTag && !tags.includes(newTag)) {
      tags = [...tags, newTag];
    }
    
    currentTag = '';
  } 
  // NEW: Handle deleting the last tag on "Backspace"
  else if (event.key === 'Backspace' && currentTag === '' && tags.length > 0) {
    // This prevents the browser from navigating back a page
    event.preventDefault();
    
    // Remove the last tag from the array
    tags = tags.slice(0, -1);
  }
}

function deleteTag(tagToDelete: string) {
  tags = tags.filter(tag => tag !== tagToDelete);
}
</script>

<!-- 
The `focus-within:*` classes have been removed from this div
to prevent the blue highlight on focus.
-->
<div class="flex w-full flex-wrap items-center gap-2 rounded-lg border border-gray-300 bg-white p-2 dark:border-gray-600 dark:bg-gray-900">

<!-- Loop through the existing tags and display them -->
{#each tags as tag (tag)}
  <span class="inline-flex items-center gap-1.5 rounded-md bg-black px-2 py-1 text-sm font-medium text-white">
    {tag}
    <button 
      type="button" 
      on:click={() => deleteTag(tag)}
      class="text-gray-400 transition-colors hover:text-white"
      aria-label="Remove {tag}"
    >
      &times;
    </button>
  </span>
{/each}

<!-- The actual input field where the user types -->
<input
  type="text"
  bind:value={currentTag}
  on:keydown={handleKeydown}
  placeholder={tags.length > 0 ? '' : 'Add a tag...'}
  class="min-w-[80px] flex-grow bg-transparent text-gray-900 placeholder-gray-500 outline-none dark:text-gray-100"
/>
</div>
