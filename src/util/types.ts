export enum STATUS {
    PAUSED,
    RUNNING
}

export enum STATE {
    POMODORO,
    SHORT_REST,
    LONG_REST
}

export type Cycle = {
    state: STATE,
    paused: boolean,
}

export type UserPreferences = {
    pomodoro_time: number,
    short_rest_time: number,
    long_rest_time: number,
}