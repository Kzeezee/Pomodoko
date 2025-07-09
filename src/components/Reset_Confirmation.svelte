<script lang="ts">
  export let updateTimeFromPreferences: any;
  export let showResetConfirmation: boolean;
</script>

<div
  class="absolute bg-gray-950/80 top-1/2 left-1/2 -translate-1/2 rounded-xl p-4"
>
  <p class="text-center">
    You just changed your preferred duration. Would you like to reset the timer?
  </p>
  <div class="flex pt-8 pb-4">
    <div class="grow"></div>
    <button
      onclick={() => {
        showResetConfirmation = !showResetConfirmation;
        updateTimeFromPreferences();
      }}
      class="border-1 rounded-sm p-2 hover:bg-white hover:text-gray-950 hover:cursor-pointer mx-4"
      >Yes</button
    >
    <button
      onclick={() => {
        showResetConfirmation = !showResetConfirmation;
      }}
      class="border-1 rounded-sm p-2 hover:bg-white hover:text-gray-950 hover:cursor-pointer mx-4"
      >No</button
    >
    <div class="grow"></div>
  </div>
</div>
