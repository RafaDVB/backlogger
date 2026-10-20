import { defineStore } from "pinia";
import { ref } from "vue";

export const useNavStore = defineStore("nav", () => {
  const openGameId = ref<number | null>(null);

  function open(id: number) {
    openGameId.value = id;
  }

  function close() {
    openGameId.value = null;
  }

  return { openGameId, open, close };
});