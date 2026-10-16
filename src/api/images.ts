import { invoke } from "@tauri-apps/api/core";
import type { GameDetail, Screenshot } from "../types/game";

export function getImagesDir(): Promise<string> {
  return invoke("get_images_dir");
}

export function addScreenshot(gameId: number, sourcePath: string): Promise<Screenshot> {
  return invoke("add_screenshot", { gameId, sourcePath });
}

export function deleteScreenshot(id: number): Promise<void> {
  return invoke("delete_screenshot", { id });
}

export function setCover(gameId: number, sourcePath: string): Promise<GameDetail> {
  return invoke("set_cover", { gameId, sourcePath });
}