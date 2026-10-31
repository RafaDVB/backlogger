import { describe, expect, it } from "vitest";
import type { GameSummary } from "../types/game";
import { activeFilterCount, applyFilters, emptyFilters, hasActiveFilters, matchesSearch, sortGames } from "./gameFilters";

let nextId = 1;

function game(partial: Partial<GameSummary> & { title: string }): GameSummary {
    return {
        id: nextId++,
        releaseYear: null,
        status: "owned",
        outcome: null,
        shelfSlot: null,
        coverPath: null,
        featuredPath: null,
        screenshotPaths: [],
        platforms: [],
        ratingId: null,
        genreIds: [],
        ...partial,
    };
}

const titlesOf = (games: GameSummary[]) => games.map((g) => g.title);
const label = (g: GameSummary) => `${g.title} ${g.releaseYear}`;

describe("search", () => {
    const doom1993 = game({ title: "Doom", releaseYear: 1993 });
    const doom2016 = game({ title: "Doom", releaseYear: 2016 });
    const re1 = game({ title: "Resident Evil", releaseYear: 1996 });
    const re2Original = game({ title: "Resident Evil 2", releaseYear: 1998 });
    const re2Remake = game({ title: "Resident Evil 2", releaseYear: 2019 });
    const re4 = game({ title: "Resident Evil 4", releaseYear: 2005 });
    const pokemon = game({ title: "Pokémon Red", releaseYear: 1996 });
    const library = [doom1993, doom2016, re1, re2Original, re2Remake, re4, pokemon];

    const found = (search: string) =>
        library.filter((g) => matchesSearch(g, search)).map(label);

    it("an empty search matches everything", () => {
        expect(found("")).toHaveLength(library.length);
        expect(found("   ")).toHaveLength(library.length);
    });

    it("finds both games that share a title", () => {
        expect(found("doom")).toEqual(["Doom 1993", "Doom 2016"]);
    });

    it("uses the year to tell them apart", () => {
        expect(found("doom 2016")).toEqual(["Doom 2016"]);
        expect(found("doom (2016)")).toEqual(["Doom 2016"]);
        expect(found("2016")).toEqual(["Doom 2016"]);
    });

    it("finds both Resident Evil 2 releases but not the other numbered games", () => {
        expect(found("resident evil 2")).toEqual(["Resident Evil 2 1998", "Resident Evil 2 2019"]);
        expect(found("resident evil 2 1998")).toEqual(["Resident Evil 2 1998"]);
    });

    it("ignores capitals and accents", () => {
        expect(found("POKEMON")).toEqual(["Pokémon Red 1996"]);
        expect(found("pokémon")).toEqual(["Pokémon Red 1996"]);
    });

    it("matches the start of a word, but a number only as a whole word", () => {
        expect(found("doo")).toEqual(["Doom 1993", "Doom 2016"]);
        expect(found("oom")).toEqual([]);
        expect(found("199")).toEqual([]);
    });
});

describe("filters", () => {
    const a = game({
        title: "A",
        releaseYear: 2000,
        genreIds: [1],
        ratingId: 5,
        outcome: "completed",
        platforms: [
            { id: 10, name: "Switch", state: "owned" },
            { id: 11, name: "Steam", state: "wanted" },
        ],
    });
    const b = game({
        title: "B",
        releaseYear: 2010,
        genreIds: [2],
        platforms: [{ id: 11, name: "Steam", state: "wanted" }],
    });
    const c = game({
        title: "C",
        releaseYear: null,
        genreIds: [1, 2],
        ratingId: 6,
        outcome: "dropped",
    });
    const games = [a, b, c];

    const run = (change: Partial<ReturnType<typeof emptyFilters>>) =>
        titlesOf(applyFilters(games, { ...emptyFilters(), ...change }));

    it("with no filters, everything passes", () => {
        expect(run({})).toEqual(["A", "B", "C"]);
    });

    it("a genre filter keeps games with any of the chosen genres", () => {
        expect(run({ genreIds: [1] })).toEqual(["A", "C"]);
        expect(run({ genreIds: [1, 2] })).toEqual(["A", "B", "C"]);
    });

    it("the platform filter can ask for any link, owned or wanted", () => {
        expect(run({ platformIds: [11] })).toEqual(["A", "B"]);
        expect(run({ platformIds: [11], platformMode: "wanted" })).toEqual(["A", "B"]);
        expect(run({ platformIds: [11], platformMode: "owned" })).toEqual([]);
        expect(run({ platformIds: [10], platformMode: "owned" })).toEqual(["A"]);
        expect(run({ platformIds: [10], platformMode: "wanted" })).toEqual([]);
    });

    it("the rating filter can include unrated games", () => {
        expect(run({ ratingIds: [5] })).toEqual(["A"]);
        expect(run({ ratingIds: [null] })).toEqual(["B"]);
        expect(run({ ratingIds: [null, 6] })).toEqual(["B", "C"]);
    });

    it("the outcome filter can ask for games with no outcome", () => {
        expect(run({ outcomes: ["none"] })).toEqual(["B"]);
        expect(run({ outcomes: ["completed", "dropped"] })).toEqual(["A", "C"]);
    });

    it("the year range is inclusive and leaves out games with no year", () => {
        expect(run({ yearFrom: 2005 })).toEqual(["B"]);
        expect(run({ yearTo: 2005 })).toEqual(["A"]);
        expect(run({ yearFrom: 2000, yearTo: 2010 })).toEqual(["A", "B"]);
        expect(run({ yearFrom: 2010, yearTo: 2010 })).toEqual(["B"]);
    });

    it("different filters combine with and", () => {
        expect(run({ genreIds: [1], outcomes: ["dropped"] })).toEqual(["C"]);
        expect(run({ genreIds: [1], search: "b" })).toEqual([]);
    });

    it("knows whether any filter is active", () => {
        expect(hasActiveFilters(emptyFilters())).toBe(false);
        expect(hasActiveFilters({ ...emptyFilters(), search: "  " })).toBe(false);
        expect(hasActiveFilters({ ...emptyFilters(), yearFrom: 1990 })).toBe(true);
        expect(hasActiveFilters({ ...emptyFilters(), ratingIds: [null] })).toBe(true);
    });

    it("counts the filter groups in use", () => {
        expect(activeFilterCount(emptyFilters())).toBe(0);
        const filters = { ...emptyFilters(), search: "x", yearFrom: 1990, yearTo: 2000 };
        expect(activeFilterCount(filters)).toBe(2); // the search, and the year range as one
    });
});

describe("sorting", () => {
    const g = (title: string, year: number | null) => game({ title, releaseYear: year });

    it("sorts titles in natural order, ascending and descending", () => {
        const list = [g("Game 10", 1), g("Game 2", 1), g("game 1", 1)];
        expect(titlesOf(sortGames(list, { field: "title", direction: "asc" }))).toEqual([
            "game 1",
            "Game 2",
            "Game 10",
        ]);
        expect(titlesOf(sortGames(list, { field: "title", direction: "desc" }))).toEqual([
            "Game 10",
            "Game 2",
            "game 1",
        ]);
    });

    it("sorts by year, with unknown years last in both directions", () => {
        const list = [g("X", 2010), g("Y", null), g("Z", 1990)];
        expect(titlesOf(sortGames(list, { field: "year", direction: "asc" }))).toEqual(["Z", "X", "Y"]);
        expect(titlesOf(sortGames(list, { field: "year", direction: "desc" }))).toEqual(["X", "Z", "Y"]);
    });

    it("breaks year ties by title", () => {
        const list = [g("B", 2000), g("A", 2000)];
        expect(titlesOf(sortGames(list, { field: "year", direction: "desc" }))).toEqual(["A", "B"]);
    });

    it("keeps games with the same title in release order", () => {
        const remake = g("Resident Evil 2", 2019);
        const original = g("Resident Evil 2", 1998);
        const list = [remake, original];
        expect(sortGames(list, { field: "title", direction: "asc" })).toEqual([original, remake]);
        expect(sortGames(list, { field: "title", direction: "desc" })).toEqual([original, remake]);
    });

    it("does not change the list it was given", () => {
        const list = [g("B", 2), g("A", 1)];
        sortGames(list, { field: "title", direction: "asc" });
        expect(titlesOf(list)).toEqual(["B", "A"]);
    });
});