import { describe, expect, it } from "vitest";
import { defaultViewState, parseViewState, type ViewState } from "./viewState";

describe("parseViewState", () => {
    it("gives the defaults when nothing was saved", () => {
        expect(parseViewState(null)).toEqual(defaultViewState());
        expect(parseViewState("")).toEqual(defaultViewState());
    });

    it("gives the defaults for text that isn't JSON", () => {
        expect(parseViewState("{oops")).toEqual(defaultViewState());
        expect(parseViewState("[1, 2]")).toEqual(defaultViewState());
    });

    it("round-trips a saved view", () => {
        const saved: ViewState = {
            tab: "interested",
            filters: {
                search: "doom",
                genreIds: [1, 2],
                platformIds: [3],
                platformMode: "wanted",
                ratingIds: [null, 5],
                outcomes: ["none", "dropped"],
                yearFrom: 1990,
                yearTo: null,
            },
            sort: { field: "year", direction: "desc" },
        };
        expect(parseViewState(JSON.stringify(saved))).toEqual(saved);
    });

    it("fills in what is missing", () => {
        const state = parseViewState('{"tab":"owned"}');
        expect(state.tab).toBe("owned");
        expect(state.filters).toEqual(defaultViewState().filters);
        expect(state.sort).toEqual(defaultViewState().sort);
    });

    it("ignores values of the wrong type", () => {
        const messy = {
            tab: "nonsense",
            filters: {
                genreIds: "x",
                platformIds: [1, "a", null, 2],
                platformMode: "bogus",
                outcomes: ["completed", "weird"],
                yearFrom: "1990",
            },
            sort: { field: "price", direction: "sideways" },
        };
        const state = parseViewState(JSON.stringify(messy));
        expect(state.tab).toBe("all");
        expect(state.filters.genreIds).toEqual([]);
        expect(state.filters.platformIds).toEqual([1, 2]);
        expect(state.filters.platformMode).toBe("any");
        expect(state.filters.outcomes).toEqual(["completed"]);
        expect(state.filters.yearFrom).toBeNull();
        expect(state.sort).toEqual({ field: "title", direction: "asc" });
    });

    it("keeps null in the rating list, which means Unrated", () => {
        const state = parseViewState(JSON.stringify({ filters: { ratingIds: [null, 4, "x"] } }));
        expect(state.filters.ratingIds).toEqual([null, 4]);
    });
});