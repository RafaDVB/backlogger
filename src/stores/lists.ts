import { defineStore } from "pinia";
import { ref } from "vue";
import * as listsApi from "../api/lists";
import type { Rating } from "../types/game";

export const useListsStore = defineStore("lists", () => {
  const ratings = ref<Rating[]>([]);

  async function loadRatings() {
    ratings.value = await listsApi.listRatings();
  }

  return { ratings, loadRatings };
});