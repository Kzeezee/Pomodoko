<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Pause from "../components/icons/Pause_Icon.svelte";
  import * as Types from "../util/types";
  import type { Cycle } from "../util/types";
  import { TaskTimer } from "tasktimer";
  import Database from "@tauri-apps/plugin-sql";
  import { Store } from "@tauri-apps/plugin-store";
  import { onMount } from "svelte";
  import { userPreferencesState } from "../util/state.svelte";
  import {
    formatMinutes,
    formatSeconds,
    LONG_REST_DEFAULT,
    POMODORO_DEFAULT,
    SHORT_REST_DEFAULT,
  } from "../util/util";
  import SettingsIcon from "../components/icons/Settings_Icon.svelte";
  import PauseIcon from "../components/icons/Pause_Icon.svelte";
  import PlayIcon from "../components/icons/Play_Icon.svelte";
  import SkipForwardIcon from "../components/icons/Skip_Forward_Icon.svelte";
  import Settings from "../components/Settings.svelte";
  import ResetConfirmation from "../components/Reset_Confirmation.svelte";

  let cycle: Cycle = $state({
    state: Types.STATE.POMODORO,
    paused: true,
  });
  let set = $state(1);
  let time = $state(POMODORO_DEFAULT);
  let showSettings = $state(false);
  let showResetConfirmation = $state(false);

  // Constant will never change
  const timer = new TaskTimer(1000);
  timer.add(updateTime);

  // Read preferences from Tauri Store
  onMount(async () => {
    const store = await Store.load("preferences.json");
    const pomodoroResult = await store.get<{ value: number }>("pomodoro");
    const shortResult = await store.get<{ value: number }>("short_rest");
    const longResult = await store.get<{ value: number }>("long_rest");
    userPreferencesState.pomodoro_time =
      pomodoroResult?.value ?? POMODORO_DEFAULT;
    userPreferencesState.short_rest_time =
      shortResult?.value ?? SHORT_REST_DEFAULT;
    userPreferencesState.long_rest_time =
      longResult?.value ?? LONG_REST_DEFAULT;

    // Since Pomodoro is the first state, set it to user's preferred time
    time = pomodoroResult?.value ?? POMODORO_DEFAULT;
  });

  function updateTime() {
    time -= 1;

    // If time reaches 0, change state
    if (time <= 0) {
      goNextState();
    }
  }

  function toggleState() {
    cycle.paused = !cycle.paused;
    if (cycle.paused) {
      timer.pause();
    } else {
      timer.resume();
    }
  }

  function goNextState() {
    let nextState = determineNextState();
    switch (nextState) {
      case Types.STATE.POMODORO:
        set++;
        time = userPreferencesState.pomodoro_time;
        break;
      case Types.STATE.SHORT_REST:
        time = userPreferencesState.short_rest_time;
        break;
      case Types.STATE.LONG_REST:
        time = userPreferencesState.long_rest_time;
        break;
    }
    cycle.paused = true;
    timer.pause();
    cycle.state = nextState;
  }

  function determineNextState() {
    if (cycle.state === Types.STATE.POMODORO) {
      // Extra check to see if set of 4 for long rest
      if (set % 4 == 0) {
        return Types.STATE.LONG_REST;
      } else {
        return Types.STATE.SHORT_REST;
      }
    } else {
      return Types.STATE.POMODORO;
    }
  }

  // Determine there's a need to show the timer reset confirmation
  // if the current state's preferred duration is changed.
  function determinePreferencesChangeForCurrentState() {
    switch (cycle.state) {
      case Types.STATE.POMODORO:
        if (time != userPreferencesState.pomodoro_time) {
          showResetConfirmation = true;
        }
        break;
      case Types.STATE.SHORT_REST:
        if (time != userPreferencesState.short_rest_time) {
          showResetConfirmation = true;
        }
        break;
      case Types.STATE.LONG_REST:
        if (time != userPreferencesState.long_rest_time) {
          showResetConfirmation = true;
        }
        break;
    }
  }

  function updateTimeFromPreferences() {
    switch (cycle.state) {
      case Types.STATE.POMODORO:
        time = userPreferencesState.pomodoro_time;
        break;
      case Types.STATE.SHORT_REST:
        time = userPreferencesState.short_rest_time;
        break;
      case Types.STATE.LONG_REST:
        time = userPreferencesState.long_rest_time;
        break;
    }
  }

  // // async function greet(event: Event) {
  // //   event.preventDefault();
  // //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  // //   greetMsg = await invoke("greet", { name });
  // // }
</script>

<main
  class="text-white flex flex-col text-center h-dvh p-4 overflow-hidden"
  class:bg-red-400={cycle.state === Types.STATE.POMODORO}
  class:bg-blue-400={cycle.state === Types.STATE.SHORT_REST}
  class:bg-cyan-700={cycle.state === Types.STATE.LONG_REST}
>
  <div class="set text-gray-300 text-2xl">
    <span>Set #{set}</span>
  </div>
  <div class="timer text-8xl font-semibold items-center mt-2 mb-6">
    <span>{formatMinutes(time)}:{formatSeconds(time)}</span>
  </div>
  <hr class="border-2 mx-12 my-4" />
  <div
    class="controls mt-2 self-center flex w-full items-center justify-center"
  >
    <button
      onclick={() => {
        showSettings = !showSettings;
        cycle.paused = true;
      }}
      class="hover:cursor-pointer"
    >
      <SettingsIcon />
    </button>
    <button onclick={toggleState} class="hover:cursor-pointer mx-8">
      {#if cycle.paused}
        <PlayIcon />
      {:else}
        <PauseIcon />
      {/if}
    </button>
    <button onclick={goNextState} class="hover:cursor-pointer">
      <SkipForwardIcon />
    </button>
  </div>
  {#if showSettings}
    <div class="overlay absolute top-0 left-0 h-full w-full bg-black/50"></div>
    <Settings {determinePreferencesChangeForCurrentState} bind:showSettings />
  {/if}
  {#if showResetConfirmation}
    <div class="overlay absolute top-0 left-0 h-full w-full bg-black/50"></div>
    <ResetConfirmation {updateTimeFromPreferences} bind:showResetConfirmation />
  {/if}
</main>
