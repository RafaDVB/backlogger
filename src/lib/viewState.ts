import type { Outcome } from "../types/game";
import { emptyFilters, type GameFilters, type GameSort, type PlatformMode } from "./gameFilters";

export type Tab = "all" | "owned" | "interested";

export interface ViewState {
    tab: Tab;
    filters: GameFilters;
    sort: GameSort;
}

export function defaultViewState(): ViewState {
    return { tab: "all", filters: emptyFilters(), sort: { field: "title", direction: "asc" } };
}

const TABS: string[] = ["all", "owned", "interested"];
const PLATFORM_MODES: string[] = ["any", "owned", "wanted"];
const OUTCOMES: string[] = ["none", "completed", "dropped"];

function isRecord(value: unknown): value is Record<string, unknown> {
    return typeof value === "object" && value !== null && !Array.isArray(value);
}

function numbers(value: unknown): number[] {
    return Array.isArray(value)
        ? value.filter((v): v is number => typeof v === "number" && Number.isFinite(v))
        : [];
}

function numbersOrNull(value: unknown): (number | null)[] {
    return Array.isArray(value)
        ? value.filter(
            (v): v is number | null => v === null || (typeof v === "number" && Number.isFinite(v)),
        )
        : [];
}

function year(value: unknown): number | null {
    return typeof value === "number" && Number.isInteger(value) ? value : null;
}

export function parseViewState(json: string | null): ViewState {
    const defaults = defaultViewState();
    if (!json) return defaults;

    let raw: unknown;
    try {
        raw = JSON.parse(json);
    } catch {
        return defaults;
    }
    if (!isRecord(raw)) return defaults;

    const filters = isRecord(raw.filters) ? raw.filters : {};
    const sort = isRecord(raw.sort) ? raw.sort : {};

    return {
        tab: TABS.includes(raw.tab as string) ? (raw.tab as Tab) : defaults.tab,
        filters: {
            search: typeof filters.search === "string" ? filters.search : "",
            genreIds: numbers(filters.genreIds),
            platformIds: numbers(filters.platformIds),
            platformMode: PLATFORM_MODES.includes(filters.platformMode as string)
                ? (filters.platformMode as PlatformMode)
                : "any",
            ratingIds: numbersOrNull(filters.ratingIds),
            outcomes: Array.isArray(filters.outcomes)
                ? (filters.outcomes.filter((o) => OUTCOMES.includes(o as string)) as (Outcome | "none")[])
                : [],
            yearFrom: year(filters.yearFrom),
            yearTo: year(filters.yearTo),
        },
        sort: {
            field: sort.field === "year" ? "year" : "title",
            direction: sort.direction === "desc" ? "desc" : "asc",
        },
    };
}