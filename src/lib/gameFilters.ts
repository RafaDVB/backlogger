import type { GameSummary, Outcome } from "../types/game";

export type PlatformMode = "any" | "owned" | "wanted";
export type SortField = "title" | "year";
export type SortDirection = "asc" | "desc";

export interface GameFilters {
    search: string;
    genreIds: number[];
    platformIds: number[];
    platformMode: PlatformMode;
    ratingIds: (number | null)[];
    outcomes: (Outcome | "none")[];
    yearFrom: number | null;
    yearTo: number | null;
}

export interface GameSort {
    field: SortField;
    direction: SortDirection;
}

export function emptyFilters(): GameFilters {
    return {
        search: "",
        genreIds: [],
        platformIds: [],
        platformMode: "any",
        ratingIds: [],
        outcomes: [],
        yearFrom: null,
        yearTo: null,
    };
}

export function activeFilterCount(f: GameFilters): number {
    return [
        f.search.trim() !== "",
        f.genreIds.length > 0,
        f.platformIds.length > 0,
        f.ratingIds.length > 0,
        f.outcomes.length > 0,
        f.yearFrom !== null || f.yearTo !== null,
    ].filter(Boolean).length;
}

export function hasActiveFilters(f: GameFilters): boolean {
    return activeFilterCount(f) > 0;
}

// ---- search ----

function normalize(text: string): string {
    return text.normalize("NFD").replace(/\p{Diacritic}/gu, "").toLowerCase();
}

function wordsOf(text: string): string[] {
    return normalize(text)
        .split(/[^\p{L}\p{N}]+/u)
        .filter((word) => word !== "");
}

export function matchesSearch(game: GameSummary, search: string): boolean {
    const typed = wordsOf(search);
    if (typed.length === 0) return true;
    const words = wordsOf(`${game.title} ${game.releaseYear ?? ""}`);
    return typed.every((word) =>
        /^\d+$/.test(word) ? words.includes(word) : words.some((w) => w.startsWith(word)),
    );
}

// ---- filters ----

function matchesGenres(game: GameSummary, f: GameFilters): boolean {
    return f.genreIds.length === 0 || game.genreIds.some((id) => f.genreIds.includes(id));
}

function matchesPlatforms(game: GameSummary, f: GameFilters): boolean {
    if (f.platformIds.length === 0) return true;
    return game.platforms.some(
        (p) =>
            f.platformIds.includes(p.id) && (f.platformMode === "any" || p.state === f.platformMode),
    );
}

function matchesRating(game: GameSummary, f: GameFilters): boolean {
    return f.ratingIds.length === 0 || f.ratingIds.includes(game.ratingId);
}

function matchesOutcome(game: GameSummary, f: GameFilters): boolean {
    return f.outcomes.length === 0 || f.outcomes.includes(game.outcome ?? "none");
}

function matchesYears(game: GameSummary, f: GameFilters): boolean {
    if (f.yearFrom === null && f.yearTo === null) return true;
    if (game.releaseYear === null) return false;
    if (f.yearFrom !== null && game.releaseYear < f.yearFrom) return false;
    if (f.yearTo !== null && game.releaseYear > f.yearTo) return false;
    return true;
}

export function applyFilters(games: GameSummary[], f: GameFilters): GameSummary[] {
    return games.filter(
        (game) =>
            matchesSearch(game, f.search) &&
            matchesGenres(game, f) &&
            matchesPlatforms(game, f) &&
            matchesRating(game, f) &&
            matchesOutcome(game, f) &&
            matchesYears(game, f),
    );
}

// ---- sorting ----

const collator = new Intl.Collator(undefined, { sensitivity: "base", numeric: true });

function compareTitles(a: GameSummary, b: GameSummary): number {
    return collator.compare(a.title, b.title);
}

function compareYears(a: GameSummary, b: GameSummary, direction: SortDirection): number {
    if (a.releaseYear === null && b.releaseYear === null) return 0;
    if (a.releaseYear === null) return 1;
    if (b.releaseYear === null) return -1;
    const difference = a.releaseYear - b.releaseYear;
    return direction === "asc" ? difference : -difference;
}

export function sortGames(games: GameSummary[], sort: GameSort): GameSummary[] {
    return [...games].sort((a, b) => {
        if (sort.field === "year") {
            return compareYears(a, b, sort.direction) || compareTitles(a, b);
        }
        const byTitle = compareTitles(a, b);
        return (sort.direction === "asc" ? byTitle : -byTitle) || compareYears(a, b, "asc");
    });
}