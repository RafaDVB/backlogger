import { invoke } from "@tauri-apps/api/core";
import type { DeletePreview, Genre, Platform, Rating } from "../types/game";

export function listRatings(): Promise<Rating[]> {
  return invoke("list_ratings");
}

export function listGenres(): Promise<Genre[]> {
  return invoke("list_genres");
}

export function listPlatforms(): Promise<Platform[]> {
  return invoke("list_platforms");
}

export function createPlatform(name: string): Promise<Platform> {
  return invoke("create_platform", { name });
}

export function createGenre(name: string): Promise<Genre> {
  return invoke("create_genre", { name });
}
export function renameGenre(id: number, name: string): Promise<Genre> {
  return invoke("rename_genre", { id, name });
}
export function previewDeleteGenre(id: number): Promise<DeletePreview> {
  return invoke("preview_delete_genre", { id });
}
export function deleteGenre(id: number): Promise<void> {
  return invoke("delete_genre", { id });
}

export function createRating(label: string, color: string | null): Promise<Rating> {
  return invoke("create_rating", { label, color });
}
export function updateRating(id: number, label: string, color: string | null): Promise<Rating> {
  return invoke("update_rating", { id, label, color });
}
export function reorderRatings(ids: number[]): Promise<Rating[]> {
  return invoke("reorder_ratings", { ids });
}
export function previewDeleteRating(id: number): Promise<DeletePreview> {
  return invoke("preview_delete_rating", { id });
}
export function deleteRating(id: number): Promise<void> {
  return invoke("delete_rating", { id });
}

export function renamePlatform(id: number, name: string): Promise<Platform> {
  return invoke("rename_platform", { id, name });
}
export function mergePlatform(fromId: number, toId: number): Promise<void> {
  return invoke("merge_platform", { fromId, toId });
}
export function previewDeletePlatform(id: number): Promise<DeletePreview> {
  return invoke("preview_delete_platform", { id });
}
export function deletePlatform(id: number): Promise<void> {
  return invoke("delete_platform", { id });
}