import type { Task } from "./types";

// Database
export const DB_NAME = "sqlite:pomodoko.db";

// Tasks
export const tasksObject = $state({ tasks: [] as Task[] });

// Timer related
export const POMODORO_DEFAULT = 25*60;
export const SHORT_REST_DEFAULT = 5*60;
export const LONG_REST_DEFAULT = 15*60;

export function formatMinutes(totalSeconds: number) {
    return Math.floor(totalSeconds / 60)
        .toString()
        .padStart(2, "0");
}

export function formatSeconds(totalSeconds: number) {
    return (totalSeconds%60).toString().padStart(2, "0");
}