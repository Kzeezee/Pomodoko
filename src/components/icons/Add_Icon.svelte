<script>
    import * as Types from "../../util/types";
    export let cycleState = null;
</script>

<svg
  width="48"
  height="48"
  viewBox="0 0 24 24"
  fill="none"
  xmlns="http://www.w3.org/2000/svg"
  class="text-white/40 transition-colors duration-300"
  class:hover:text-red-400={cycleState === Types.STATE.POMODORO}
  class:hover:text-blue-400={cycleState === Types.STATE.SHORT_REST}
  class:hover:text-cyan-700={cycleState === Types.STATE.LONG_REST}
>
  <path
    d="M6 12H12M12 12H18M12 12V18M12 12V6"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
  />
</svg>
