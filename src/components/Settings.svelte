<script lang="ts">
  import { Store } from "@tauri-apps/plugin-store";
  import { userPreferencesState } from "../util/state.svelte";
  import { formatMinutes, formatSeconds } from "../util/util.svelte";

  export let showSettings: boolean;
  export let determinePreferencesChangeForCurrentState: any;

  function formatInput(element: Event) {
    const input = element.currentTarget as HTMLInputElement;
    const value = input.value.toString();
    input.value = value.padStart(2, "0");
    if (value.length > 2) {
      input.value = value.substring(1);
    }
  }

  async function savePreferences(event: Event) {
    const form = event.currentTarget as HTMLFormElement;
    const data = new FormData(form);

    const newPomodoroTime = Number(data.get('pomodoro_min'))*60 + Number(data.get('pomodoro_sec'));
    const newShortRestTime = Number(data.get('short_rest_min'))*60 + Number(data.get('short_rest_sec'));
    const newLongRestTime = Number(data.get('long_rest_min'))*60 + Number(data.get('long_rest_sec'));

    userPreferencesState.pomodoro_time = newPomodoroTime;
    userPreferencesState.short_rest_time = newShortRestTime;
    userPreferencesState.long_rest_time = newLongRestTime;
    // Save to store
    const store = await Store.load("preferences.json");
    await store.set("pomodoro", {value:newPomodoroTime});
    await store.set("short_rest", {value:newShortRestTime});
    await store.set("long_rest", {value:newLongRestTime});
    await store.save();

    showSettings = false;
    determinePreferencesChangeForCurrentState();
  }
</script>

<div class="absolute bg-gray-950/80 top-1/2 left-1/2 -translate-1/2 rounded-xl p-2">
  <button onclick={() => {showSettings = !showSettings}} class="absolute right-0 top-0 mt-2 mr-4 hover:cursor-pointer">
    X
  </button>
  <h1 class="font-semibold mt-2 mb-4 text-xl">Preferences</h1>
  <form class="min-w-80" onsubmit={savePreferences}>
    <label class="grid grid-cols-2">
      Pomodoro Time
      <div class="">
        <input
          class="input max-w-14 validator text-center"
          name="pomodoro_min"
          value={formatMinutes(userPreferencesState.pomodoro_time)}
          type="number"
          max="99"
          min="0"
          oninput={(element) => formatInput(element)}
        />
        :
        <input
          class="input max-w-14 validator text-center"
          name="pomodoro_sec"
          value={formatSeconds(userPreferencesState.pomodoro_time)}
          type="number"
          max="59"
          min="0"
          oninput={(element) => formatInput(element)}
        />
      </div>
    </label>
    <label class="grid grid-cols-2">
      Short Rest Time
      <div class="">
        <input
          class="input max-w-14 validator text-center"
          name="short_rest_min"
          value={formatMinutes(userPreferencesState.short_rest_time)}
          type="number"
          max="99"
          min="0"
          oninput={(element) => formatInput(element)}
        />
        :
        <input
          class="input max-w-14 validator text-center"
          name="short_rest_sec"
          value={formatSeconds(userPreferencesState.short_rest_time)}
          type="number"
          max="59"
          min="0"
          oninput={(element) => formatInput(element)}
        />
      </div>
    </label>
    <label class="grid grid-cols-2">
      Long Rest Time
      <div class="">
        <input
          class="input max-w-14 validator text-center"
          name="long_rest_min"
          value={formatMinutes(userPreferencesState.long_rest_time)}
          type="number"
          max="99"
          min="0"
          oninput={(element) => formatInput(element)}
        />
        :
        <input
          class="input max-w-14 validator text-center"
          name="long_rest_sec"
          value={formatSeconds(userPreferencesState.long_rest_time)}
          type="number"
          max="59"
          min="0"
          oninput={(element) => formatInput(element)}
        />
      </div>
    </label>
    <button class="border rounded-md p-2 mt-4 hover:cursor-pointer hover:bg-white hover:text-gray-950/80" type="submit">Save</button
    >
  </form>
</div>
