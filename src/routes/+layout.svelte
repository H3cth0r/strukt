<script lang="ts">
	import '../app.css';
  import { NavBar } from '$lib/components';

  export const prerender = true;
	
	let { children } = $props();
</script>

<NavBar />

{@render children()}

