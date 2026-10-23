import { defineStore } from "pinia";
import { ref } from "vue";
import { describeError } from "../api/errors";
import * as settingsApi from "../api/settings";

export const useSettingsStore = defineStore("settings", () => {
    const rawgApiKey = ref<string | null>(null);
    const error = ref<string | null>(null);

    async function load() {
        try {
            rawgApiKey.value = (await settingsApi.getSettings()).rawgApiKey;
        } catch (e) {
            error.value = describeError(e);
        }
    }

    async function saveRawgApiKey(key: string): Promise<boolean> {
        try {
            rawgApiKey.value = (await settingsApi.setRawgApiKey(key)).rawgApiKey;
            return true;
        } catch (e) {
            error.value = describeError(e);
            return false;
        }
    }

    return { rawgApiKey, error, load, saveRawgApiKey };
});