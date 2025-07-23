<script>
import { onMount, onDestroy } from 'svelte';
import { tweened } from 'svelte/motion';
import { cubicOut } from 'svelte/easing';

// --- PROPS ---
/** The size (width and height) of the eye container in pixels. */
export let size = 256; // NEW: Default size is 256px

// --- REACTIVE CALCULATIONS ---
// These values will automatically update whenever the `size` prop changes.
$: pupilSize = size * 0.375; // NEW: Pupil is 37.5% of the container size (96 / 256)
$: eyeMovementRadius = size * 0.24; // NEW: Movement radius is 24% of the container size (60 / 256)

// --- STORES ---
const eyePosition = tweened({ x: 0, y: 0 }, {
	duration: 400,
	easing: cubicOut
});

// --- STATE ---
let containerEl;
let mode = 'TRACKING';
let isBlinking = false;
let lastMousePosition = { x: 0, y: 0 };

// --- TIMERS ---
let blinkTimer;
let distractionTimer;

// --- CONFIGURATION (now mostly derived from props) ---
const BLINK_INTERVAL_MIN = 2000;
const BLINK_INTERVAL_MAX = 7000;
const DISTRACTION_INTERVAL_MIN = 5000;
const DISTRACTION_INTERVAL_MAX = 12000;

// --- LOGIC ---

function handleMouseMove(event) {
	lastMousePosition = { x: event.clientX, y: event.clientY };
	if (mode !== 'TRACKING' || !containerEl) return;

	const rect = containerEl.getBoundingClientRect();
	const centerX = rect.left + rect.width / 2;
	const centerY = rect.top + rect.height / 2;

	const deltaX = lastMousePosition.x - centerX;
	const deltaY = lastMousePosition.y - centerY;

	const angle = Math.atan2(deltaY, deltaX);
	const distance = Math.sqrt(deltaX * deltaX + deltaY * deltaY);

	// CHANGED: Use the reactive `eyeMovementRadius`
	const targetX = Math.cos(angle) * Math.min(distance, eyeMovementRadius);
	const targetY = Math.sin(angle) * Math.min(distance, eyeMovementRadius);

	eyePosition.set({ x: targetX, y: targetY }, { duration: 100 });
}

function triggerBlink() {
	isBlinking = true;
	setTimeout(() => {
		isBlinking = false;
	}, 150);
}

function startDistraction() {
	mode = 'DISTRACTED';

	const randomAngle = Math.random() * 2 * Math.PI;
	// CHANGED: Use the reactive `eyeMovementRadius`
	const randomRadius = Math.random() * eyeMovementRadius;
	const targetX = Math.cos(randomAngle) * randomRadius;
	const targetY = Math.sin(randomAngle) * randomRadius;

	eyePosition.set({ x: targetX, y: targetY }, { duration: 700 }).then(() => {
		setTimeout(() => {
			mode = 'TRACKING';
			handleMouseMove({ clientX: lastMousePosition.x, clientY: lastMousePosition.y });
			scheduleDistraction();
		}, 600);
	});
}

// --- SCHEDULERS (Unchanged) ---

function scheduleBlink() {
	const delay = Math.random() * (BLINK_INTERVAL_MAX - BLINK_INTERVAL_MIN) + BLINK_INTERVAL_MIN;
	clearTimeout(blinkTimer);
	blinkTimer = setTimeout(() => {
		triggerBlink();
		scheduleBlink();
	}, delay);
}

function scheduleDistraction() {
	const delay = Math.random() * (DISTRACTION_INTERVAL_MAX - DISTRACTION_INTERVAL_MIN) + DISTRACTION_INTERVAL_MIN;
	clearTimeout(distractionTimer);
	distractionTimer = setTimeout(startDistraction, delay);
}

// --- LIFECYCLE (Unchanged) ---

onMount(() => {
	scheduleBlink();
	scheduleDistraction();
});

onDestroy(() => {
	clearTimeout(blinkTimer);
	clearTimeout(distractionTimer);
});
</script>

<svelte:window on:mousemove={handleMouseMove} />

<!-- 
 CHANGED: Removed fixed w-64/h-64 classes.
 Now using inline styles to set the size dynamically based on the `size` prop.
-->
<div
bind:this={containerEl}
class="relative flex justify-center items-center bg-black rounded-3xl overflow-hidden shadow-lg"
style="width: {size}px; height: {size}px;"
>
<!-- 
   CHANGED: Removed fixed w-24/h-24 classes.
   Pupil size and position are now both controlled by style attributes.
 -->
<div
	class="absolute bg-white rounded-full transition-transform duration-100 ease-in-out"
	class:blink={isBlinking}
	style="
     width: {pupilSize}px; 
     height: {pupilSize}px; 
     transform: translate({$eyePosition.x}px, {$eyePosition.y}px);
   "
/>
</div>

<style>
.blink {
	@apply scale-y-0;
}
</style>
