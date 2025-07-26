<script lang="ts">
  import * as Sheet from "$lib/components/ui/sheet/index.js";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
	import Info from '@lucide/svelte/icons/info';
</script>


<Sheet.Root>
  <Sheet.Trigger class={buttonVariants({ variant: "outline" })}
    ><Info /></Sheet.Trigger
  >
  <Sheet.Content side="right">
    <Sheet.Header>
      <Sheet.Title>Edit profile</Sheet.Title>
      <Sheet.Description>
        Make changes to your profile here. Click save when you're done.
      </Sheet.Description>
    </Sheet.Header>
    <div class="grid flex-1 auto-rows-min gap-6 px-4">
      <div class="grid gap-3">
      </div>
      <div class="grid gap-3">
      </div>
    </div>
    <Sheet.Footer>
      <Sheet.Close class={buttonVariants({ variant: "outline" })}
        >Close</Sheet.Close
      >
    </Sheet.Footer>
  </Sheet.Content>
</Sheet.Root>
