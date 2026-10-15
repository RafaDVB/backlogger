import { invoke } from "@tauri-apps/api/core";
import type { Genre, Platform, Rating } from "../types/game";

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