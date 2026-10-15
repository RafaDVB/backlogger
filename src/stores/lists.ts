import { defineStore } from "pinia";
import { ref } from "vue";
import * as listsApi from "../api/lists";
import type { Genre, Platform, Rating } from "../types/game";

export const useListsStore = defineStore("lists", () => {
  const ratings = ref<Rating[]>([]);
  const genres = ref<Genre[]>([]);
  const platforms = ref<Platform[]>([]);

  async function loadRatings() {
    ratings.value = await listsApi.listRatings();
  }

  async function loadAll() {
    [ratings.value, genres.value, platforms.value] = await Promise.all([
      listsApi.listRatings(),
      listsApi.listGenres(),
      listsApi.listPlatforms(),
    ]);
  }

  async function addPlatform(name: string): Promise<Platform> {
    const platform = await listsApi.createPlatform(name);
    platforms.value = await listsApi.listPlatforms();
    return platform;
  }

  return { ratings, genres, platforms, loadRatings, loadAll, addPlatform };
});