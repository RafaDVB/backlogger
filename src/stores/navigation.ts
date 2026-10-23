import { defineStore } from "pinia";
import { ref } from "vue";

export const useNavStore = defineStore("nav", () => {
  const openGameId = ref<number | null>(null);
  const settingsOpen = ref(false);

  function open(id: number) {
    openGameId.value = id;
    settingsOpen.value = false;
  }

  function close() {
    openGameId.value = null;
  }

  function openSettings() {
    settingsOpen.value = true;
    openGameId.value = null;
  }

  function closeSettings() {
    settingsOpen.value = false;
  }

  return { openGameId, settingsOpen, open, close, openSettings, closeSettings };
});