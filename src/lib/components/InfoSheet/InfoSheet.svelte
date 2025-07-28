<script lang="ts">
 import * as Sheet from "$lib/components/ui/sheet/index.js";
 import { buttonVariants } from "$lib/components/ui/button/index.js";
import Info from '@lucide/svelte/icons/info';
</script>

<Sheet.Root>
 <Sheet.Trigger class={buttonVariants({ variant: "outline" })}
   ><Info /><span class="sr-only">About Strukt</span></Sheet.Trigger
 >
 <Sheet.Content side="right" class="w-full sm:max-w-lg overflow-y-auto">
   <Sheet.Header>
     <Sheet.Title>About Strukt</Sheet.Title>
     <Sheet.Description>
       Strukt is a tool for intelligently merging two JSON files. It uses user-defined identity keys to match objects within arrays, then generates a transparent 'Merge Plan' that shows exactly what changes will be made. You can review this plan before executing it to produce the final merged result.
     </Sheet.Description>
   </Sheet.Header>
   <div class="grid flex-1 auto-rows-min gap-8 py-6 px-4">
     <div class="grid gap-3">
       <h4 class="text-lg font-medium">How It Works</h4>
       <ol class="list-decimal list-inside space-y-2 text-sm text-muted-foreground">
         <li>
           <strong>Provide Inputs:</strong> You upload two JSON files and specify one or more 'identity keys' (e.g., <code>id</code>, <code>name</code>). These keys are crucial for matching corresponding objects between the two files.
         </li>
         <li>
           <strong>Generate Plan:</strong> Strukt processes the inputs and generates a detailed <strong>Merge Plan</strong>. This plan is a preview of every proposed action (add, keep, overwrite, etc.) without modifying the original data.
         </li>
         <li>
           <strong>Execute Plan:</strong> After reviewing the color-coded plan, you execute it. This step applies the specified actions to create the final, merged JSON output.
         </li>
       </ol>
     </div>
     <div class="grid gap-4">
       <h4 class="text-lg font-medium">Understanding the Merge Plan</h4>
       <p class="text-sm text-muted-foreground">
         The plan is a new JSON structure where every element is wrapped in an object containing a <code>_status</code> and a <code>_value</code>. This allows for a clear, line-by-line preview of the merge logic.
       </p>
       <div>
         <h5 class="font-semibold mb-2">Statuses & Colors</h5>
         <ul class="space-y-3 text-sm">
           <li class="flex items-start gap-3">
             <span class="mt-1 block h-4 w-4 rounded-sm flex-shrink-0" style="background-color: #eff6ff;"></span>
             <div>
               <code class="font-bold">Merge</code>
               <p class="text-muted-foreground">Indicates a container (object or array) where child elements have been changed. It's a parent node for other actions.</p>
             </div>
           </li>
           <li class="flex items-start gap-3">
             <span class="mt-1 block h-4 w-4 rounded-sm flex-shrink-0" style="background-color: #dcfce7;"></span>
             <div>
               <code class="font-bold">Add</code>
               <p class="text-muted-foreground">This item exists only in the second file and will be added to the final result.</p>
             </div>
           </li>
            <li class="flex items-start gap-3">
             <span class="mt-1 block h-4 w-4 rounded-sm flex-shrink-0" style="background-color: #fef2f2;"></span>
             <div>
               <code class="font-bold">Overwrite</code>
               <p class="text-muted-foreground">The value from the first file will be replaced by the value from the second file.</p>
             </div>
           </li>
           <li class="flex items-start gap-3">
             <span class="mt-1 block h-4 w-4 rounded-sm flex-shrink-0" style="background-color: #f7fee7;"></span>
             <div>
               <code class="font-bold">Keep</code>
               <p class="text-muted-foreground">This item is identical in both files or only exists in the first file and will be kept as-is.</p>
             </div>
           </li>
           <li class="flex items-start gap-3">
             <span class="mt-1 block h-4 w-4 rounded-sm flex-shrink-0" style="background-color: #fee2e2;"></span>
             <div>
               <code class="font-bold">Delete</code>
               <p class="text-muted-foreground">This item from the first file will be removed. (Note: This status can be set by manually editing the plan before execution).</p>
             </div>
           </li>
         </ul>
       </div>
       <div>
         <h5 class="font-semibold mb-2">Values</h5>
          <ul class="space-y-2 text-sm">
            <li>
              <code>_value</code>
              <p class="text-muted-foreground">Represents the proposed new value for a key. For an <code style="color: #059669;">Overwrite</code> status, this is the value from the second file.</p>
            </li>
            <li>
              <code>_old_value</code>
              <p class="text-muted-foreground">Only appears when the status is <code>Overwrite</code>. It shows the original value from the first file that is being replaced, highlighted in <span style="color: #dc2626;">red</span>.</p>
            </li>
          </ul>
       </div>
     </div>
   </div>
   <Sheet.Footer>
     <Sheet.Close class={buttonVariants({ variant: "outline" })}
       >Close</Sheet.Close
     >
   </Sheet.Footer>
 </Sheet.Content>
</Sheet.Root>
