import { invoke } from "@tauri-apps/api/core";

export function greet(name: string) {
    return invoke<string>("greet", { name })
}