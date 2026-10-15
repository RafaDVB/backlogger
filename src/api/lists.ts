import { invoke } from "@tauri-apps/api/core";
import type { Rating } from "../types/game";

export function listRatings(): Promise<Rating[]> {
  return invoke("list_ratings");
}