<script lang="ts">
  import * as Card from "$lib/components/ui/card/index.js";
  import { Eye, InfoSheet } from "$lib/components";
</script>

<div class="flex justify-center items-center w-full h-[10vh] ">
  <Card.Root class="flex flex-row justify-between items-center w-[98%] h-[70%]">
    <div class="ml-6" />
    <div>
      <Eye size={40}/>
    </div>
    <div class="mr-6">
    <InfoSheet />
    </div>
  </Card.Root>
</div>
