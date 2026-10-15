import type { GameSummary } from "./game";

export interface ShelfEntry {
  slot: number;
  game: GameSummary | null;
}

export type RemoveResult =
  | { kind: "finished"; ratingId?: number | null }
  | { kind: "dropped" }
  | { kind: "notFinished" };