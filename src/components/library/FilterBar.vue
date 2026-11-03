<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { activeFilterCount } from "../../lib/gameFilters";
import { useListsStore } from "../../stores/lists";
import { useViewStore } from "../../stores/view";
import type { Outcome } from "../../types/game";
import BaseButton from "../ui/BaseButton.vue";
import BaseChip from "../ui/BaseChip.vue";
import BaseSelect from "../ui/BaseSelect.vue";
import MultiSelect from "../ui/MultiSelect.vue";

const view = useViewStore();
const lists = useListsStore();
const panelOpen = ref(false);

const activeCount = computed(() => activeFilterCount(view.filters));

const genreOptions = computed(() => lists.genres.map((g) => ({ value: g.id, label: g.name })));
const platformOptions = computed(() =>
    lists.platforms.map((p) => ({ value: p.id, label: p.name })),
);
const ratingOptions = computed<{ value: number | null; label: string }[]>(() => [
    { value: null, label: "Unrated" },
    ...lists.ratings.map((r) => ({ value: r.id, label: r.label })),
]);
const outcomeOptions: { value: Outcome | "none"; label: string }[] = [
    { value: "none", label: "No outcome" },
    { value: "completed", label: "Completed" },
    { value: "dropped", label: "Dropped" },
];
const sortOptions: { value: "title" | "year"; label: string }[] = [
    { value: "title", label: "Title" },
    { value: "year", label: "Release year" },
];
const platformModeOptions: { value: "any" | "owned" | "wanted"; label: string }[] = [
    { value: "any", label: "linked to" },
    { value: "owned", label: "owned on" },
    { value: "wanted", label: "wanted on" },
];

function toggleOutcome(value: Outcome | "none") {
    const current = view.filters.outcomes;
    view.filters.outcomes = current.includes(value)
        ? current.filter((v) => v !== value)
        : [...current, value];
}

const directionLabel = computed(() => {
    const ascending = view.sort.direction === "asc";
    if (view.sort.field === "year") return ascending ? "Oldest first" : "Newest first";
    return ascending ? "A → Z" : "Z → A";
});

function toggleDirection() {
    view.sort.direction = view.sort.direction === "asc" ? "desc" : "asc";
}

// ---- the year range: typed text on one side, a number (or null) in the filters on the other ----

function parseYear(text: string): number | null {
    const trimmed = text.trim();
    return /^\d{4}$/.test(trimmed) ? Number(trimmed) : null;
}

const fromText = ref(view.filters.yearFrom?.toString() ?? "");
const toText = ref(view.filters.yearTo?.toString() ?? "");

watch(fromText, (text) => (view.filters.yearFrom = parseYear(text)));
watch(toText, (text) => (view.filters.yearTo = parseYear(text)));

watch(
    () => view.filters.yearFrom,
    (year) => {
        if (parseYear(fromText.value) !== year) fromText.value = year === null ? "" : String(year);
    },
);
watch(
    () => view.filters.yearTo,
    (year) => {
        if (parseYear(toText.value) !== year) toText.value = year === null ? "" : String(year);
    },
);
</script>

<template>
    <div class="bar">
        <div class="row">
            <input v-model="view.filters.search" type="text" class="search" placeholder="Search by title or year…"
                @keydown.esc="view.filters.search = ''" />

            <span class="group">
                <span class="hint">Sort</span>
                <BaseSelect v-model="view.sort.field" :options="sortOptions" />
            </span>
            <BaseButton size="sm" title="Change the sort direction" @click="toggleDirection">
                {{ directionLabel }}
            </BaseButton>

            <BaseButton size="sm" @click="panelOpen = !panelOpen">
                Filters{{ activeCount ? ` (${activeCount})` : "" }} {{ panelOpen ? "▴" : "▾" }}
            </BaseButton>
            <BaseButton v-if="activeCount" size="sm" variant="quiet" @click="view.clearFilters()">
                Clear
            </BaseButton>
        </div>

        <div v-if="panelOpen" class="panel">
            <MultiSelect v-model="view.filters.genreIds" label="Genres" :options="genreOptions" />

            <span class="group">
                <MultiSelect v-model="view.filters.platformIds" label="Platforms" :options="platformOptions" />
                <BaseSelect v-model="view.filters.platformMode" :options="platformModeOptions"
                    :disabled="view.filters.platformIds.length === 0" />
            </span>

            <MultiSelect v-model="view.filters.ratingIds" label="Ratings" :options="ratingOptions" />

            <span class="group">
                <BaseChip v-for="option in outcomeOptions" :key="option.value"
                    :active="view.filters.outcomes.includes(option.value)" @click="toggleOutcome(option.value)">
                    {{ option.label }}
                </BaseChip>
            </span>

            <span class="group">
                <span class="hint">Year</span>
                <input v-model="fromText" type="text" inputmode="numeric" maxlength="4" placeholder="from"
                    class="year" />
                <span class="hint">to</span>
                <input v-model="toText" type="text" inputmode="numeric" maxlength="4" placeholder="to" class="year" />
            </span>
        </div>
    </div>
</template>

<style scoped>
.bar {
    margin-bottom: var(--space-4);
}

.row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-2);
}

.search {
    flex: 1;
    min-width: 200px;
}

.group {
    display: inline-flex;
    align-items: center;
    gap: var(--space-2);
}

.panel {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-4);
    margin-top: var(--space-3);
    padding: var(--space-3);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    background: var(--color-surface);
}

.year {
    width: 64px;
}
</style>