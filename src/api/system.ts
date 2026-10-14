import { invoke } from "@tauri-apps/api/core";

export function ping(message: string): Promise<string> {
    return invoke<string>("ping", { message });
}
