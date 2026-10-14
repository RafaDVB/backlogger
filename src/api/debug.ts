import { invoke } from "@tauri-apps/api/core";

export function seedSampleData(): Promise<void> {
  return invoke("seed_sample_data");
}