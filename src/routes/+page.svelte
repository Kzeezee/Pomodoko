<script lang="ts">
  import * as Types from "../util/types";
  import type { Cycle } from "../util/types";
  import { TaskTimer } from "tasktimer";
  import { Store } from "@tauri-apps/plugin-store";
  import { onMount } from "svelte";
  import { userPreferencesState } from "../util/state.svelte";
  import {
    DB,
    DEFAULT_TASK_NAME,
    formatMinutes,
    formatSeconds,
    LONG_REST_DEFAULT,
    POMODORO_DEFAULT,
    SHORT_REST_DEFAULT,
  } from "../util/util.svelte";
  import {
    isPermissionGranted,
    requestPermission,
    sendNotification,
  } from "@tauri-apps/plugin-notification";
  import SettingsIcon from "../components/icons/Settings_Icon.svelte";
  import PauseIcon from "../components/icons/Pause_Icon.svelte";
  import PlayIcon from "../components/icons/Play_Icon.svelte";
  import SkipForwardIcon from "../components/icons/Skip_Forward_Icon.svelte";
  import Settings from "../components/Settings.svelte";
  import ResetConfirmation from "../components/Reset_Confirmation.svelte";
  import AddIcon from "../components/icons/Add_Icon.svelte";
  import HelpIcon from "../components/icons/Help_Icon.svelte";
  import TrashIcon from "../components/icons/Trash_Icon.svelte";
  import { tasksObject } from "../util/util.svelte";
  import type { Task } from "../util/types";
  import {
    axis,
    bounds,
    BoundsFrom,
    ControlFrom,
    controls,
    draggable,
    events,
    position,
  } from "@neodrag/svelte";

  let cycle: Cycle = $state({
    state: Types.STATE.POMODORO,
    paused: true,
  });
  let set = $state(1);
  let time = $state(POMODORO_DEFAULT);
  let showSettings = $state(false);
  let showResetConfirmation = $state(false);

  let dragState: any = $state(null);

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

    // Notification permissions
    let permissionGranted = await isPermissionGranted();
    // If not we need to request it
    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === "granted";
    }

    // Tasks-related
    updateTasksState();
  });

  async function addNewTask() {
    await DB.execute(
      "INSERT into tasks (name, completed) VALUES ($1, $2)",
      ["Your new task", false]
    );
    updateTasksState();
  }

  async function updateTask(id: number, name: String, completed: boolean) {
    await DB.execute("UPDATE tasks SET name=$1, completed=$2 WHERE id = $3;", [
      name,
      completed,
      id,
    ]);
    updateTasksState();
  }

  async function deleteTask(id: number) {
    await DB.execute("DELETE FROM tasks WHERE id = $1;", [id]);
    updateTasksState();
  }

  async function updateTasksState() {
    let queryTasks = await DB.select("SELECT * FROM tasks LIMIT 10;");
    let taskArray = queryTasks as Task[];
    taskArray.forEach((e) => {
      e.position = { x: 0, y: 0 };
      e.completed = e.completed === "true"; // Have to do this due to SQLite storing boolean as string
    });
    tasksObject.tasks = taskArray;
  }

  function updateTime() {
    time -= 1;

    // If time reaches 0, change state
    if (time <= 0) {
      goNextState(false);
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

  function goNextState(skip: boolean) {
    let nextState = determineNextState();
    let msg = "";
    switch (nextState) {
      case Types.STATE.POMODORO:
        set++;
        time = userPreferencesState.pomodoro_time;
        msg = "Time to focus! Lock in!";
        break;
      case Types.STATE.SHORT_REST:
        time = userPreferencesState.short_rest_time;
        msg = "Time for a short break! Take a breather!";
        break;
      case Types.STATE.LONG_REST:
        time = userPreferencesState.long_rest_time;
        msg = "Time for a long break! Recharge with some tea or coffee!";
        break;
    }
    if (!skip) {
      sendNotification({ title: "Pomodoko", body: msg });
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
</script>

<main
  class="text-white flex flex-col text-center h-dvh p-4 overflow-x-hidden"
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
  <div class="controls self-center flex w-full items-center justify-center">
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
    <button
      onclick={() => {
        goNextState(true);
      }}
      class="hover:cursor-pointer"
    >
      <SkipForwardIcon />
    </button>
  </div>
  <hr class="border-2 mx-12 my-8" />
  <div class="tasks mb-4 flex flex-col items-center">
    <h1 class="text-xl mb-2 flex">
      Tasks
      <div class="self-center relative pl-1">
        <div class="absolute min-w-40 opacity-0 hover:opacity-100 text-sm z-20">
          <div class="relative -top-4 bg-gray-900/60 p-1 rounded-sm left-6">
            <p class="">Drag the tasks to the <br /> left to delete it</p>
          </div>
        </div>
        <HelpIcon />
      </div>
    </h1>
    <div class="task-container flex flex-col w-4/5">
      {#each tasksObject.tasks as task (task.id)}
        <div
          class="flex w-full bg-black/20 m-1 p-4 text-left max-h-14 hover:cursor-grab"
          {@attach draggable([
            bounds(BoundsFrom.viewport({ left: -200 })),
            axis("x"),
            position({ current: task.position }),
            controls({ block: ControlFrom.selector('.cancel') }),
            events({
              onDragStart: (data) => {
                dragState = { taskId: task.id, position: data.offset.x };
              },
              onDrag: (data) => {
                dragState = {
                  taskId: task.id,
                  position: data.offset.x,
                };
              },
              onDragEnd: (data) => {
                if (data.offset.x < -150) {
                  // Delete the task here
                  deleteTask(task.id);
                } else {
                  task.position = { x: 0, y: 0 };
                }
                dragState = null;
              },
            }),
          ])}
        >
          {#if dragState?.taskId === task.id}
            <div
              class="absolute flex top-0 left-0 w-full h-full z-10 bg-gray-100 justify-center items-center"
              style="opacity: {Math.min(
                Math.max(dragState?.position / -150, 0),
                1
              )};"
            >
              <TrashIcon bind:cycleState={cycle.state} />
            </div>
          {/if}
          <div
            class="task-name line-clamp-1 mr-4 cancel"
            class:crossed={task.completed === true}
          >
            <input
              class="field-sizing-content min-w-4 cancel"
              value={task.name}
              onchange={(e) => {
                let input = e.currentTarget as HTMLInputElement;
                if (input.value.length <= 0) {
                  input.value = DEFAULT_TASK_NAME;
                }
                task.name = input.value;
                updateTask(task.id, task.name, task.completed);
              }}
            />
          </div>
          <div class="spacing grow"></div>
          <div class="completed min-w-1/12 flex items-center cancel">
            <input
              class="checkbox flex"
              type="checkbox"
              name="completed"
              checked={task.completed === true}
              oninput={() => {
                task.completed = !task.completed;
                updateTask(task.id, task.name, task.completed);
              }}
            />
          </div>
        </div>
      {/each}
    </div>
    <div class="add-task flex w-4/5 items-center justify-center pt-2">
      <button
        onclick={addNewTask}
        class="btn bg-transparent p-0 w-12 border-0 border-white rounded-md cursor-pointer hover:bg-white shadow-none"
      >
        <AddIcon bind:cycleState={cycle.state} />
      </button>
    </div>
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

<style>
  .task-name {
    position: relative;
    display: inline-block;
    transition: color 0.3s;
  }
  .task-name::after {
    content: "";
    position: absolute;
    left: 0;
    top: 50%;
    height: 2px;
    background: currentColor;
    width: 0;
    transition: width 0.4s cubic-bezier(0.4, 0, 0.2, 1);
    pointer-events: none;
  }
  .task-name.crossed::after {
    width: 100%;
  }
  .task-name.crossed {
    color: #aaa;
  }
  .task-name.crossed::after {
    text-decoration: line-through;
  }
</style>
