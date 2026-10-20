import { invoke } from "@tauri-apps/api/core";
import type { GameDetail, GameFilter, GameSummary, NewGame, GameEdit, Outcome } from "../types/game";

export function listGames(filter: GameFilter = {}): Promise<GameSummary[]> {
  return invoke("list_games", { filter });
}

export function getGame(id: number): Promise<GameDetail> {
  return invoke("get_game", { id });
}

export function markOwned(id: number): Promise<GameDetail> {
  return invoke("mark_owned", { id });
}

export function createGame(newGame: NewGame): Promise<GameDetail> {
  return invoke("create_game", { newGame });
}

export function updateGame(id: number, edit: GameEdit): Promise<GameDetail> {
  return invoke("update_game", { id, edit });
}

export function deleteGame(id: number): Promise<void> {
  return invoke("delete_game", { id });
}

export function setFeaturedScreenshot(gameId: number, screenshotId: number | null): Promise<GameDetail> {
  return invoke("set_featured_screenshot", { gameId, screenshotId });
}

export function setRating(gameId: number, ratingId: number | null): Promise<GameDetail> {
  return invoke("set_rating", { gameId, ratingId });
}

export function setOutcome(gameId: number, outcome: Outcome | null): Promise<GameDetail> {
  return invoke("set_outcome", { gameId, outcome });
}
