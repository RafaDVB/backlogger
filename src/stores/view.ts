import { defineStore } from "pinia";
import { ref, watch } from "vue";
import * as settingsApi from "../api/settings";
import { emptyFilters, type GameFilters, type GameSort } from "../lib/gameFilters";
import { defaultViewState, parseViewState, type Tab } from "../lib/viewState";

export const useViewStore = defineStore("view", () => {
    const defaults = defaultViewState();
    const tab = ref<Tab>(defaults.tab);
    const filters = ref<GameFilters>(defaults.filters);
    const sort = ref<GameSort>(defaults.sort);
    const loaded = ref(false);

    async function load() {
        try {
            const state = parseViewState(await settingsApi.getUiState());
            tab.value = state.tab;
            filters.value = state.filters;
            sort.value = state.sort;
        } catch (e) {
            console.warn("Couldn't load the saved view", e);
        } finally {
            loaded.value = true;
        }
    }

    function save() {
        const json = JSON.stringify({ tab: tab.value, filters: filters.value, sort: sort.value });
        settingsApi.setUiState(json).catch((e) => console.warn("Couldn't save the view", e));
    }

    let timer: number | undefined;
    watch(
        [tab, filters, sort],
        () => {
            if (!loaded.value) return;
            window.clearTimeout(timer);
            timer = window.setTimeout(save, 500);
        },
        { deep: true },
    );

    function clearFilters() {
        filters.value = emptyFilters();
    }

    function prune(valid: { genres: number[]; platforms: number[]; ratings: number[] }) {
        const f = filters.value;
        const genres = f.genreIds.filter((id) => valid.genres.includes(id));
        if (genres.length !== f.genreIds.length) f.genreIds = genres;
        const platforms = f.platformIds.filter((id) => valid.platforms.includes(id));
        if (platforms.length !== f.platformIds.length) f.platformIds = platforms;
        const ratings = f.ratingIds.filter((id) => id === null || valid.ratings.includes(id));
        if (ratings.length !== f.ratingIds.length) f.ratingIds = ratings;
    }

    return { tab, filters, sort, load, clearFilters, prune };
});