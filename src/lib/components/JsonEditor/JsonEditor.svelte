<!-- JsonEditor -->
<script>
import { onMount, onDestroy } from 'svelte';
import { EditorView } from '@codemirror/view';
import { EditorState } from '@codemirror/state';
import { basicSetup } from 'codemirror';
import { oneDark } from '@codemirror/theme-one-dark';
import { ViewPlugin, Decoration } from '@codemirror/view';

let editorEl;
let slotEl;
let view;
let copyButtonText = 'Copy';

// --- 1. Custom Syntax Highlighting Extension ---
function jsonDiffHighlighting() {
 return ViewPlugin.fromClass(
  class {
   decorations;

   constructor(view) {
    this.decorations = this.getDecorations(view);
   }

   update(update) {
    if (update.docChanged || update.viewportChanged) {
     this.decorations = this.getDecorations(update.view);
    }
   }

   // CORRECTED LOGIC
   getDecorations(view) {
    const decorations = []; // Use a single array for all decorations

    for (const { from, to } of view.visibleRanges) {
     for (let pos = from; pos <= to; ) {
      const line = view.state.doc.lineAt(pos);
      const lineText = line.text;

      const statusMatch = lineText.match(/"_status":\s*"(\w+)"/);
      if (statusMatch) {
       const status = statusMatch[1];
       decorations.push(Decoration.line({ class: `cm-line-highlight-${status}` }).range(line.from));
      }

      const valueMatch = lineText.match(/"_value":\s*(.*)/);
      if (valueMatch) {
       const valueText = valueMatch[1].trim().replace(/,$/, '');
       const valueStart = line.from + valueMatch.index + valueMatch[0].indexOf(valueText);
       const valueEnd = valueStart + valueText.length;
       decorations.push(Decoration.mark({ class: 'cm-json-diff-value' }).range(valueStart, valueEnd));
      }

      const oldValueMatch = lineText.match(/"_old_value":\s*(.*)/);
      if (oldValueMatch) {
       const oldValueText = oldValueMatch[1].trim().replace(/,$/, '');
       const oldValueStart = line.from + oldValueMatch.index + oldValueMatch[0].indexOf(oldValueText);
       const oldValueEnd = oldValueStart + oldValueText.length;
       decorations.push(Decoration.mark({ class: 'cm-json-diff-old-value' }).range(oldValueStart, oldValueEnd));
      }

      pos = line.to + 1;
     }
    }
    return Decoration.set(decorations);
   }
  },
  {
   decorations: (v) => v.decorations
  }
 );
}

// --- 2. Custom Theme for Light and Dark Mode ---
const jsonDiffTheme = EditorView.theme(
 {
  '&': {
   color: 'var(--editor-text-color)',
   backgroundColor: 'var(--editor-bg-color)'
  },
  '.cm-content': {
   caretColor: 'var(--editor-caret-color)'
  },
  '&.cm-focused .cm-cursor': {
   borderLeftColor: 'var(--editor-caret-color)'
  },
  '&.cm-focused .cm-selectionBackground, ::selection': {
   backgroundColor: 'var(--editor-selection-bg)'
  },
  '.cm-gutters': {
   backgroundColor: 'var(--editor-gutter-bg)',
   color: 'var(--editor-gutter-text)',
   border: 'none'
  },
  '.cm-json-diff-value': { color: 'var(--editor-value-color)' },
  '.cm-json-diff-old-value': { color: 'var(--editor-old-value-color)' },
  '.cm-line-highlight-merge': { backgroundColor: 'var(--editor-highlight-merge)' },
  '.cm-line-highlight-overwrite': { backgroundColor: 'var(--editor-highlight-overwrite)' },
  '.cm-line-highlight-keep': { backgroundColor: 'var(--editor-highlight-keep)' }
 },
 { dark: false }
);

// --- 3. Editor Setup ---
onMount(() => {
 const initialDoc = slotEl.textContent || '';

 const extensions = [
  basicSetup,
  jsonDiffTheme,
  jsonDiffHighlighting()
 ];

 if (document.body.classList.contains('dark')) {
  extensions.push(oneDark);
 }

 const state = EditorState.create({
  doc: initialDoc,
  extensions: extensions
 });

 view = new EditorView({
  state,
  parent: editorEl
 });
});

onDestroy(() => {
 if (view) {
  view.destroy();
 }
});

// --- 4. Copy Functionality ---
async function handleCopy() {
 if (!view) return;
 try {
  await navigator.clipboard.writeText(view.state.doc.toString());
  copyButtonText = 'Copied!';
  setTimeout(() => {
   copyButtonText = 'Copy';
  }, 2000);
 } catch (err) {
  copyButtonText = 'Failed!';
  console.error('Failed to copy text: ', err);
 }
}
</script>

<div class="editor-wrapper">
<button class="copy-button" on:click={handleCopy}>{copyButtonText}</button>
<div bind:this={editorEl} />
</div>

<div bind:this={slotEl} style="display: none;">
<slot />
</div>

<style>
/* --- 5. Component and Theme Styling --- */
.editor-wrapper {
 position: relative;
 border: 1px solid var(--editor-border-color);
 border-radius: 8px;
 overflow: hidden;
 font-family: 'Fira Code', 'Courier New', Courier, monospace;
 font-size: 14px;
}

.copy-button {
 position: absolute;
 top: 8px;
 right: 8px;
 z-index: 10;
 background-color: var(--editor-button-bg);
 color: var(--editor-button-text);
 border: 1px solid var(--editor-button-border);
 border-radius: 4px;
 padding: 4px 8px;
 cursor: pointer;
 transition: background-color 0.2s;
}

.copy-button:hover {
 background-color: var(--editor-button-hover-bg);
}

:root {
 --editor-text-color: #333;
 --editor-bg-color: #ffffff;
 --editor-border-color: #d1d5db;
 --editor-caret-color: #000000;
 --editor-selection-bg: #dbeafe;
 --editor-gutter-bg: #f9fafb;
 --editor-gutter-text: #6b7280;

 --editor-button-bg: #f9fafb;
 --editor-button-text: #374151;
 --editor-button-border: #d1d5db;
 --editor-button-hover-bg: #f3f4f6;

 --editor-value-color: #059669;
 --editor-old-value-color: #dc2626;
 --editor-highlight-merge: #eff6ff;
 --editor-highlight-overwrite: #fef2f2;
 --editor-highlight-keep: #f7fee7;
}

:global(body.dark) {
 --editor-text-color: #d1d5db;
 --editor-bg-color: #1f2937;
 --editor-border-color: #4b5563;
 --editor-caret-color: #f9fafb;
 --editor-selection-bg: #374151;
 --editor-gutter-bg: #111827;
 --editor-gutter-text: #9ca3af;

 --editor-button-bg: #374151;
 --editor-button-text: #d1d5db;
 --editor-button-border: #4b5563;
 --editor-button-hover-bg: #4b5563;

 --editor-value-color: #34d399;
 --editor-old-value-color: #f87171;
 --editor-highlight-merge: rgba(59, 130, 246, 0.15);
 --editor-highlight-overwrite: rgba(239, 68, 68, 0.15);
 --editor-highlight-keep: rgba(132, 204, 22, 0.1);
}

:global(.cm-editor) {
 height: 100%;
}
</style>
 
