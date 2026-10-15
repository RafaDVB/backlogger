import { invoke } from "@tauri-apps/api/core";
import type { GameDetail, GameFilter, GameSummary } from "../types/game";

export function listGames(filter: GameFilter = {}): Promise<GameSummary[]> {
  return invoke("list_games", { filter });
}

export function getGame(id: number): Promise<GameDetail> {
  return invoke("get_game", { id });
}

export function markOwned(id: number): Promise<GameDetail> {
  return invoke("mark_owned", { id });
}