import { invoke } from "@tauri-apps/api/core";
import type { RemoveResult, ShelfEntry } from "../types/shelf";

export function getShelf(): Promise<ShelfEntry[]> {
  return invoke("get_shelf");
}

export function placeOnShelf(gameId: number, slot: number): Promise<void> {
  return invoke("place_on_shelf", { gameId, slot });
}

export function moveShelfSlot(from: number, to: number): Promise<void> {
  return invoke("move_shelf_slot", { from, to });
}

export function removeFromShelf(gameId: number, result: RemoveResult): Promise<void> {
  return invoke("remove_from_shelf", { gameId, result });
}