<script>
  import { app } from "@tauri-apps/api";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import "../app.css";
  import WindowMaximize from "../components/icons/Window_Maximize.svelte";
  import WindowClose from "../components/icons/Window_Close.svelte";
  import WindowMinimize from "../components/icons/Window_Minimize.svelte";

  let { children } = $props();

  const appWindow = getCurrentWindow();
</script>

<div class="titlebar z-10 h-8 select-none grid fixed top-0 left-0 right-0">
  <div data-tauri-drag-region></div>
  <div class="flex">
    <button onclick={() => appWindow.minimize()} class="btn-appearance-none p-0 m-0 border-none inline-flex justify-center
      items-center w-8 bg-white/0 rounded-sm hover:bg-white/30">
      <WindowMinimize/>
    </button>
    <button onclick={() => appWindow.toggleMaximize()} class="btn-appearance-none p-0 m-0 border-none inline-flex justify-center
      items-center w-8 bg-white/0 rounded-sm hover:bg-white/30">
      <WindowMaximize/>
    </button>
    <button onclick={() => appWindow.close()} class="btn-appearance-none p-0 m-0 border-none inline-flex justify-center
      items-center w-8 bg-white/0 rounded-sm hover:bg-white/30">
      <WindowClose/>
    </button>
  </div>
</div>

{@render children()}

<style>
  .titlebar {
    grid-template-columns: auto max-content;
  }
</style>
