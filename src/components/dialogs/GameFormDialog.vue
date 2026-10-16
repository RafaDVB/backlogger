<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { describeError } from "../../api/errors";
import * as gamesApi from "../../api/games";
import * as imagesApi from "../../api/images";
import { resolveImage } from "../../lib/images";
import { pickImages } from "../../lib/pickImages";
import { useLibraryStore } from "../../stores/library";
import { useListsStore } from "../../stores/lists";
import { useShelfStore } from "../../stores/shelf";
import type { GameDetail, Screenshot, Status } from "../../types/game";

const props = defineProps<{ gameId: number | null; defaultStatus: Status }>();
const emit = defineEmits<{ close: [] }>();

const library = useLibraryStore();
const lists = useListsStore();
const shelf = useShelfStore();

const editingId = ref<number | null>(props.gameId);
const original = ref<GameDetail | null>(null);

const title = ref("");
const yearText = ref("");
const status = ref<Status>(props.defaultStatus);
const platformIds = ref<number[]>([]);
const genreIds = ref<number[]>([]);
const newPlatform = ref("");

const coverFile = ref<string | null>(null);
const newScreenshots = ref<string[]>([]);
const existingScreenshots = ref<Screenshot[]>([]);
const removedScreenshotIds = ref<number[]>([]);

const loading = ref(true);
const saving = ref(false);
const error = ref<string | null>(null);
const confirmingDelete = ref(false);

const heading = computed(() => (editingId.value === null ? "Add a game" : "Edit game"));

onMounted(async () => {
  try {
    await lists.loadAll();
    if (props.gameId !== null) {
      const game = await gamesApi.getGame(props.gameId);
      original.value = game;
      title.value = game.title;
      yearText.value = game.releaseYear === null ? "" : String(game.releaseYear);
      status.value = game.status;
      platformIds.value = game.platforms.map((p) => p.id);
      genreIds.value = game.genres.map((g) => g.id);
      existingScreenshots.value = game.screenshots;
    }
  } catch (e) {
    error.value = describeError(e);
  } finally {
    loading.value = false;
  }
});

function fileName(path: string): string {
  return path.split(/[\\/]/).pop() ?? path;
}

async function addPlatform() {
  const name = newPlatform.value.trim();
  if (!name) return;
  try {
    const platform = await lists.addPlatform(name);
    if (!platformIds.value.includes(platform.id)) platformIds.value.push(platform.id);
    newPlatform.value = "";
  } catch (e) {
    error.value = describeError(e);
  }
}

async function chooseCover() {
  const [file] = await pickImages(false);
  if (file) coverFile.value = file;
}

async function addScreenshots() {
  newScreenshots.value.push(...(await pickImages(true)));
}

function toggleRemoved(id: number) {
  const index = removedScreenshotIds.value.indexOf(id);
  if (index === -1) removedScreenshotIds.value.push(id);
  else removedScreenshotIds.value.splice(index, 1);
}

async function save() {
  error.value = null;
  if (!title.value.trim()) {
    error.value = "A game needs a title.";
    return;
  }
  const yearInput = yearText.value.trim();
  if (yearInput !== "" && !/^\d{4}$/.test(yearInput)) {
    error.value = "The release year should be four digits, like 2017.";
    return;
  }
  const releaseYear = yearInput === "" ? null : Number(yearInput);

  saving.value = true;
  try {
    let id = editingId.value;
    if (id === null) {
      const created = await gamesApi.createGame({
        title: title.value,
        releaseYear,
        status: status.value,
        platformIds: platformIds.value,
        genreIds: genreIds.value,
      });
      id = created.id;
      editingId.value = id;
    } else {
      await gamesApi.updateGame(id, {
        title: title.value,
        releaseYear,
        platformIds: platformIds.value,
        genreIds: genreIds.value,
      });
    }

    // Each finished step is removed from its list, so a retry only repeats what's left.
    while (removedScreenshotIds.value.length > 0) {
      const removedId = removedScreenshotIds.value[0];
      await imagesApi.deleteScreenshot(removedId);
      existingScreenshots.value = existingScreenshots.value.filter((s) => s.id !== removedId);
      removedScreenshotIds.value.shift();
    }
    while (newScreenshots.value.length > 0) {
      await imagesApi.addScreenshot(id, newScreenshots.value[0]);
      newScreenshots.value.shift();
    }
    if (coverFile.value) {
      await imagesApi.setCover(id, coverFile.value);
      coverFile.value = null;
    }

    await library.load();
    emit("close");
  } catch (e) {
    error.value = describeError(e);
    await library.load(); // the game may already exist even though a later step failed
  } finally {
    saving.value = false;
  }
}

async function removeGame() {
  if (editingId.value === null) return;
  saving.value = true;
  error.value = null;
  try {
    await gamesApi.deleteGame(editingId.value);
    await Promise.all([library.load(), shelf.load()]); // deleting may have freed a shelf slot
    emit("close");
  } catch (e) {
    error.value = describeError(e);
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <div class="overlay">
    <form class="panel" @submit.prevent="save">
      <h3>{{ heading }}</h3>

      <p v-if="loading">Loading…</p>

      <template v-else>
        <label class="field">
          <span>Title</span>
          <input v-model="title" type="text" />
        </label>

        <label class="field">
          <span>Release year</span>
          <input v-model="yearText" type="text" inputmode="numeric" placeholder="2017" />
        </label>

        <div class="field">
          <span>Status</span>
          <div v-if="editingId === null" class="row">
            <label><input v-model="status" type="radio" value="interested" /> Interested</label>
            <label><input v-model="status" type="radio" value="owned" /> Owned</label>
          </div>
          <div v-else>{{ status === "owned" ? "Owned" : "Interested" }}</div>
        </div>

        <fieldset class="field">
          <legend>Platforms</legend>
          <div class="checks">
            <label v-for="platform in lists.platforms" :key="platform.id">
              <input v-model="platformIds" type="checkbox" :value="platform.id" />
              {{ platform.name }}
            </label>
          </div>
          <div class="row">
            <input
              v-model="newPlatform"
              type="text"
              placeholder="Another platform…"
              @keydown.enter.prevent="addPlatform"
            />
            <button type="button" @click="addPlatform">Add</button>
          </div>
        </fieldset>

        <fieldset class="field">
          <legend>Genres</legend>
          <div class="checks">
            <label v-for="genre in lists.genres" :key="genre.id">
              <input v-model="genreIds" type="checkbox" :value="genre.id" />
              {{ genre.name }}
            </label>
          </div>
        </fieldset>

        <div class="field">
          <span>Cover</span>
          <div class="row">
            <img
              v-if="original?.coverPath && !coverFile"
              class="thumb cover"
              :src="resolveImage(original.coverPath) ?? undefined"
              alt=""
            />
            <button type="button" @click="chooseCover">Choose cover…</button>
            <span v-if="coverFile" class="staged">{{ fileName(coverFile) }}</span>
          </div>
        </div>

        <div class="field">
          <span>Screenshots</span>
          <div class="thumbs">
            <div
              v-for="shot in existingScreenshots"
              :key="shot.id"
              class="thumb-wrap"
              :class="{ removed: removedScreenshotIds.includes(shot.id) }"
            >
              <img class="thumb" :src="resolveImage(shot.path) ?? undefined" alt="" />
              <button type="button" class="x" @click="toggleRemoved(shot.id)">
                {{ removedScreenshotIds.includes(shot.id) ? "↺" : "×" }}
              </button>
            </div>
          </div>
          <ul v-if="newScreenshots.length" class="staged-list">
            <li v-for="(file, index) in newScreenshots" :key="file + index">
              {{ fileName(file) }}
              <button type="button" class="x" @click="newScreenshots.splice(index, 1)">×</button>
            </li>
          </ul>
          <button type="button" @click="addScreenshots">Add screenshots…</button>
        </div>

        <p v-if="error" class="error">{{ error }}</p>

        <div class="actions">
          <button type="submit" :disabled="saving">{{ saving ? "Saving…" : "Save" }}</button>
          <button type="button" :disabled="saving" @click="emit('close')">Cancel</button>

          <template v-if="editingId !== null">
            <button
              v-if="!confirmingDelete"
              type="button"
              class="danger"
              @click="confirmingDelete = true"
            >
              Delete…
            </button>
            <template v-else>
              <span>Delete this game and its images?</span>
              <button type="button" class="danger" :disabled="saving" @click="removeGame">
                Yes, delete
              </button>
              <button type="button" @click="confirmingDelete = false">No</button>
            </template>
          </template>
        </div>
      </template>
    </form>
  </div>
</template>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  display: grid;
  place-items: center;
  background: rgba(0, 0, 0, 0.5);
  z-index: 10;
}
.panel {
  width: min(560px, 92vw);
  max-height: 90vh;
  overflow-y: auto;
  padding: 20px;
  border-radius: 10px;
  background: Canvas;
  color: CanvasText;
}
.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin: 0 0 14px;
  padding: 0;
  border: none;
}
.field > span,
.field > legend {
  font-weight: 600;
  font-size: 14px;
}
.field input[type="text"] {
  padding: 6px 8px;
  border: 1px solid rgba(128, 128, 128, 0.5);
  border-radius: 6px;
  background: none;
  color: inherit;
  font: inherit;
}
.row {
  display: flex;
  align-items: center;
  gap: 12px;
}
.checks {
  display: flex;
  flex-wrap: wrap;
  gap: 4px 14px;
}
.thumbs {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}
.thumb-wrap {
  position: relative;
}
.thumb-wrap.removed .thumb {
  opacity: 0.25;
}
.thumb {
  width: 96px;
  height: 54px;
  object-fit: cover;
  border-radius: 4px;
  background: rgba(128, 128, 128, 0.25);
}
.thumb.cover {
  width: 54px;
  height: 72px;
}
.x {
  border: none;
  background: none;
  color: inherit;
  cursor: pointer;
  opacity: 0.7;
}
.thumb-wrap .x {
  position: absolute;
  top: 0;
  right: 0;
  padding: 0 5px;
  border-radius: 0 4px 0 4px;
  background: rgba(0, 0, 0, 0.7);
  color: white;
}
.staged-list {
  margin: 0;
  padding-left: 18px;
  font-size: 14px;
}
.staged {
  font-size: 14px;
  opacity: 0.8;
}
.actions {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  margin-top: 16px;
}
button {
  padding: 6px 14px;
  border: 1px solid rgba(128, 128, 128, 0.5);
  border-radius: 6px;
  background: none;
  color: inherit;
  font: inherit;
  cursor: pointer;
}
button:hover:not(:disabled) {
  background: rgba(128, 128, 128, 0.2);
}
button:disabled {
  opacity: 0.5;
  cursor: default;
}
.danger {
  border-color: crimson;
  color: crimson;
}
.error {
  color: crimson;
}
</style>