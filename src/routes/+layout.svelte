<script lang="ts">
	import '../app.css';
  import { NavBar } from '$lib/components';
	
	let { children } = $props();
</script>

<NavBar />

{@render children()}

