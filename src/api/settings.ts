import { invoke } from "@tauri-apps/api/core";
import type { Settings } from "../types/game";
import type { Appearance } from "../lib/appearance";

export function getSettings(): Promise<Settings> {
    return invoke("get_settings");
}

export function setRawgApiKey(key: string): Promise<Settings> {
    return invoke("set_rawg_api_key", { key });
}

export function getUiState(): Promise<string | null> {
    return invoke("get_ui_state");
}

export function setUiState(json: string): Promise<void> {
    return invoke("set_ui_state", { json });
}

export function getAppearance(): Promise<Appearance> {
    return invoke("get_appearance");
}

export function setAppearance(appearance: Appearance): Promise<Appearance> {
    return invoke("set_appearance", { appearance });
}
