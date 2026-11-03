import { defineStore } from "pinia";
import { ref, watch } from "vue";
import * as settingsApi from "../api/settings";
import { applyTheme, defaultAppearance, type BackgroundId, type ThemeId } from "../lib/appearance";

export const useAppearanceStore = defineStore("appearance", () => {
    const defaults = defaultAppearance();
    const theme = ref<ThemeId>(defaults.theme);
    const background = ref<BackgroundId>(defaults.background);
    const librarySize = ref(defaults.librarySize);
    const loaded = ref(false);

    async function load() {
        try {
            const saved = await settingsApi.getAppearance();
            theme.value = saved.theme;
            background.value = saved.background;
            librarySize.value = saved.librarySize;
        } catch (e) {
            console.warn("Couldn't load the appearance", e);
        } finally {
            applyTheme(theme.value);
            loaded.value = true;
        }
    }

    function save() {
        settingsApi
            .setAppearance({
                theme: theme.value,
                background: background.value,
                librarySize: Math.round(librarySize.value),
            })
            .catch((e) => console.warn("Couldn't save the appearance", e));
    }

    watch(theme, (value) => applyTheme(value), { flush: "sync" });

    let timer: number | undefined;
    watch(
        [theme, background, librarySize],
        () => {
            if (!loaded.value) return;
            window.clearTimeout(timer);
            timer = window.setTimeout(save, 300);
        },
        { flush: "sync" },
    );

    return { theme, background, librarySize, load };
});