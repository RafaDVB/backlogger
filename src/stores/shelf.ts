import { defineStore } from "pinia";
import { ref } from "vue";
import { describeError } from "../api/errors";
import * as shelfApi from "../api/shelf";
import type { GameSummary } from "../types/game";
import type { RemoveResult, ShelfEntry } from "../types/shelf";
import { useLibraryStore } from "./library";

export const useShelfStore = defineStore("shelf", () => {
  const library = useLibraryStore();

  const slots = ref<ShelfEntry[]>([]);
  const error = ref<string | null>(null);
  const pendingRemoval = ref<GameSummary | null>(null);

  async function load() {
    try {
      slots.value = await shelfApi.getShelf();
    } catch (e) {
      error.value = describeError(e);
    }
  }

  async function run(action: () => Promise<void>): Promise<boolean> {
    error.value = null;
    try {
      await action();
      await Promise.all([load(), library.load()]);
      return true;
    } catch (e) {
      error.value = describeError(e);
      return false;
    }
  }

  const place = (gameId: number, slot: number) => run(() => shelfApi.placeOnShelf(gameId, slot));
  const move = (from: number, to: number) => run(() => shelfApi.moveShelfSlot(from, to));

  function requestRemoval(gameId: number) {
    pendingRemoval.value = slots.value.find((s) => s.game?.id === gameId)?.game ?? null;
  }

  function cancelRemoval() {
    pendingRemoval.value = null;
  }

  async function confirmRemoval(result: RemoveResult) {
    const game = pendingRemoval.value;
    if (!game) return;
    if (await run(() => shelfApi.removeFromShelf(game.id, result))) {
      pendingRemoval.value = null;
    }
  }

  return { slots, error, pendingRemoval, load, place, move, requestRemoval, cancelRemoval, confirmRemoval };
});