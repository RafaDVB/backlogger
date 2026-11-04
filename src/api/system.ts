import { invoke } from "@tauri-apps/api/core";

export interface LegalInfo {
    license: string;
    thirdParty: string;
}

export function ping(message: string): Promise<string> {
    return invoke<string>("ping", { message });
}

export function getLegalInfo(): Promise<LegalInfo> {
    return invoke<LegalInfo>("get_legal_info");
}
