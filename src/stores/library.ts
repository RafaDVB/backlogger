import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { describeError } from "../api/errors";
import * as gamesApi from "../api/games";
import type { GameSummary } from "../types/game";

export const useLibraryStore = defineStore("library", () => {
  const games = ref<GameSummary[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  const interested = computed(() => games.value.filter((g) => g.status === "interested"));
  const owned = computed(() => games.value.filter((g) => g.status === "owned"));

  async function load() {
    loading.value = true;
    error.value = null;
    try {
      games.value = await gamesApi.listGames();
    } catch (e) {
      error.value = describeError(e);
    } finally {
      loading.value = false;
    }
  }

  async function markOwned(id: number, platformId: number | null = null): Promise<boolean> {
    error.value = null;
    try {
      await gamesApi.markOwned(id, platformId);
      await load();
      return true;
    } catch (e) {
      error.value = describeError(e);
      return false;
    }
  }

  return { games, loading, error, interested, owned, load, markOwned };
});