import { defineStore } from "pinia";
import { ref } from "vue";
import { describeError } from "../api/errors";
import * as listsApi from "../api/lists";
import type { DeletePreview, Genre, Platform, Rating } from "../types/game";
import { useLibraryStore } from "./library";

export const useListsStore = defineStore("lists", () => {
  const library = useLibraryStore();

  const ratings = ref<Rating[]>([]);
  const genres = ref<Genre[]>([]);
  const platforms = ref<Platform[]>([]);
  const error = ref<string | null>(null);

  async function loadRatings() {
    ratings.value = await listsApi.listRatings();
  }

  async function loadAll() {
    [ratings.value, genres.value, platforms.value] = await Promise.all([
      listsApi.listRatings(),
      listsApi.listGenres(),
      listsApi.listPlatforms(),
    ]);
  }

  async function addPlatform(name: string): Promise<Platform> {
    const platform = await listsApi.createPlatform(name);
    platforms.value = await listsApi.listPlatforms();
    return platform;
  }

  async function run(action: () => Promise<unknown>, refreshLibrary = false): Promise<boolean> {
    error.value = null;
    try {
      await action();
      await loadAll();
      if (refreshLibrary) await library.load(); // game statuses may have changed
      return true;
    } catch (e) {
      error.value = describeError(e);
      return false;
    }
  }

  const createGenre = (name: string) => run(() => listsApi.createGenre(name));
  const renameGenre = (id: number, name: string) => run(() => listsApi.renameGenre(id, name));
  const deleteGenre = (id: number) => run(() => listsApi.deleteGenre(id));

  const createPlatform = (name: string) => run(() => listsApi.createPlatform(name));
  const renamePlatform = (id: number, name: string) =>
    run(() => listsApi.renamePlatform(id, name));
  const mergePlatform = (fromId: number, toId: number) =>
    run(() => listsApi.mergePlatform(fromId, toId), true);
  const deletePlatform = (id: number) => run(() => listsApi.deletePlatform(id), true);

  const createRating = (label: string, color: string | null) =>
    run(() => listsApi.createRating(label, color));
  const updateRating = (id: number, label: string, color: string | null) =>
    run(() => listsApi.updateRating(id, label, color));
  const reorderRatings = (ids: number[]) => run(() => listsApi.reorderRatings(ids));
  const deleteRating = (id: number) => run(() => listsApi.deleteRating(id));

  async function previewDelete(
    kind: "genre" | "rating" | "platform",
    id: number,
  ): Promise<DeletePreview | null> {
    error.value = null;
    try {
      if (kind === "genre") return await listsApi.previewDeleteGenre(id);
      if (kind === "rating") return await listsApi.previewDeleteRating(id);
      return await listsApi.previewDeletePlatform(id);
    } catch (e) {
      error.value = describeError(e);
      return null;
    }
  }

  return {
    ratings,
    genres,
    platforms,
    error,
    loadRatings,
    loadAll,
    addPlatform,
    createGenre,
    renameGenre,
    deleteGenre,
    createPlatform,
    renamePlatform,
    mergePlatform,
    deletePlatform,
    createRating,
    updateRating,
    reorderRatings,
    deleteRating,
    previewDelete,
  };
});