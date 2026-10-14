import { defineStore } from "pinia";
import { ref } from "vue";
import * as debugApi from "../api/debug";
import * as gamesApi from "../api/games";
import * as system from "../api/system";
import type { GameSummary } from "../types/game";

export const useDebugStore = defineStore("debug", () => {
  const reply = ref("");
  const games = ref<GameSummary[]>([]);
  const error = ref("");

  async function ping(message: string) {
    reply.value = await system.ping(message);
  }

  async function seedAndLoad() {
    try {
      error.value = "";
      await debugApi.seedSampleData();
      games.value = await gamesApi.listGames();
    } catch (e) {
      error.value = JSON.stringify(e);
    }
  }

  return { reply, games, error, ping, seedAndLoad };
});