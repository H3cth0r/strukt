<script lang="ts">
import { onMount, onDestroy } from 'svelte';
import { EditorView } from '@codemirror/view';
import { EditorState } from '@codemirror/state';
import { basicSetup } from 'codemirror';
import { oneDark } from '@codemirror/theme-one-dark';
import { ViewPlugin, Decoration } from '@codemirror/view';
import { json } from '@codemirror/lang-json';

// --- PROPS ---
export let content: string = '';
export let mode: 'diff' | 'json' = 'json';

// --- COMPONENT-INSTANCE-SPECIFIC STATE ---
let editorEl: HTMLDivElement;
let view: EditorView;
let copyButtonText = 'Copy';

// --- EDITOR SETUP (onMount) ---
onMount(() => {
	const extensions = [basicSetup, jsonDiffTheme];

	if (mode === 'diff') {
		extensions.push(jsonDiffHighlighting());
	} else {
		extensions.push(json());
	}

	if (document.body.classList.contains('dark')) {
		extensions.push(oneDark);
	}

	const state = EditorState.create({
		doc: content,
		extensions: extensions
	});

	view = new EditorView({
		state,
		parent: editorEl
	});
});

// --- REACTIVE UPDATE ---
$: if (view && content !== view.state.doc.toString()) {
	view.dispatch({
		changes: { from: 0, to: view.state.doc.length, insert: content }
	});
}

// --- CLEANUP ---
onDestroy(() => {
	if (view) {
		view.destroy();
	}
});

// --- COPY FUNCTIONALITY ---
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

// --- CUSTOM DIFF HIGHLIGHTING PLUGIN (for 'diff' mode) ---
function jsonDiffHighlighting() {
	return ViewPlugin.fromClass(
		class {
			decorations;

			constructor(view: EditorView) {
				this.decorations = this.getDecorations(view);
			}

			update(update: { docChanged: any; viewportChanged: any; view: EditorView }) {
				if (update.docChanged || update.viewportChanged) {
					this.decorations = this.getDecorations(update.view);
				}
			}

			getDecorations(view: EditorView) {
				const decorations = [];
				for (const { from, to } of view.visibleRanges) {
					for (let pos = from; pos <= to; ) {
						const line = view.state.doc.lineAt(pos);
						const lineText = line.text;

						const statusMatch = lineText.match(/"_status":\s*"(\w+)"/);
						if (statusMatch) {
							const status = statusMatch[1];
							decorations.push(
								Decoration.line({ class: `cm-line-highlight-${status}` }).range(line.from)
							);
						}

						const valueMatch = lineText.match(/"_value":\s*(.*)/);
						if (valueMatch) {
							const valueText = valueMatch[1].trim().replace(/,$/, '');
							const valueStart = line.from + valueMatch.index + valueMatch[0].indexOf(valueText);
							const valueEnd = valueStart + valueText.length;
							decorations.push(
								Decoration.mark({ class: 'cm-json-diff-value' }).range(valueStart, valueEnd)
							);
						}

						const oldValueMatch = lineText.match(/"_old_value":\s*(.*)/);
						if (oldValueMatch) {
							const oldValueText = oldValueMatch[1].trim().replace(/,$/, '');
							const oldValueStart =
								line.from + oldValueMatch.index + oldValueMatch[0].indexOf(oldValueText);
							const oldValueEnd = oldValueStart + oldValueText.length;
							decorations.push(
								Decoration.mark({ class: 'cm-json-diff-old-value' }).range(
									oldValueStart,
									oldValueEnd
								)
							);
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

// --- CUSTOM THEME (for both modes) ---
const jsonDiffTheme = EditorView.theme(
	{
		'&': {
			color: 'var(--editor-text-color)',
			backgroundColor: 'var(--editor-bg-color)',
			height: '100%'
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
		// Classes for diff mode
		'.cm-json-diff-value': { color: 'var(--editor-value-color)' },
		'.cm-json-diff-old-value': { color: 'var(--editor-old-value-color)' },
		// **FIX**: Added highlight rules for 'add' and 'delete'
		'.cm-line-highlight-add': { backgroundColor: 'var(--editor-highlight-add)' },
		'.cm-line-highlight-delete': { backgroundColor: 'var(--editor-highlight-delete)' },
		'.cm-line-highlight-merge': { backgroundColor: 'var(--editor-highlight-merge)' },
		'.cm-line-highlight-overwrite': { backgroundColor: 'var(--editor-highlight-overwrite)' },
		'.cm-line-highlight-keep': { backgroundColor: 'var(--editor-highlight-keep)' }
	},
	{ dark: false }
);
</script>

<div class="editor-wrapper">
<button class="copy-button" on:click={handleCopy}>{copyButtonText}</button>
<div class="editor-container" bind:this={editorEl} />
</div>

<style>
.editor-wrapper {
	position: relative;
	border: 1px solid var(--editor-border-color);
	border-radius: 8px;
	overflow: hidden;
	font-family: 'Fira Code', 'Courier New', Courier, monospace;
	font-size: 14px;
	height: 100%;
	width: 100%;
}
.editor-container {
	height: 100%;
	overflow: auto;
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

/* **FIX**: Added CSS variables for 'add' and 'delete' highlights */
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
	--editor-value-color: #059669; /* Green */
	--editor-old-value-color: #dc2626; /* Red */
	--editor-highlight-add: #dcfce7; /* Light Green */
	--editor-highlight-delete: #fee2e2; /* Light Red */
	--editor-highlight-merge: #eff6ff; /* Light Blue */
	--editor-highlight-overwrite: #fef2f2; /* Lighter Red */
	--editor-highlight-keep: #f7fee7; /* Light Lime */
}

/* **FIX**: Added CSS variables for 'add' and 'delete' highlights in dark mode */
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
	--editor-highlight-add: rgba(34, 197, 94, 0.15); /* Transparent Green */
	--editor-highlight-delete: rgba(220, 38, 38, 0.2); /* Transparent Red */
	--editor-highlight-merge: rgba(59, 130, 246, 0.15);
	--editor-highlight-overwrite: rgba(239, 68, 68, 0.15);
	--editor-highlight-keep: rgba(132, 204, 22, 0.1);
}

:global(.cm-editor) {
	height: 100%;
}
</style>
