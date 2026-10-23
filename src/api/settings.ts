import { invoke } from "@tauri-apps/api/core";
import type { Settings } from "../types/game";

export function getSettings(): Promise<Settings> {
    return invoke("get_settings");
}

export function setRawgApiKey(key: string): Promise<Settings> {
    return invoke("set_rawg_api_key", { key });
}