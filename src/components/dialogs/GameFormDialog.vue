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
import type {
  GameDetail,
  GamePlatform,
  PlatformLink,
  PlatformState,
  Screenshot,
  Status,
} from "../../types/game";
import PlatformEditor from "../detail/PlatformEditor.vue";
import BaseButton from "../ui/BaseButton.vue";
import BaseDialog from "../ui/BaseDialog.vue";

const props = defineProps<{ gameId: number | null; defaultStatus: Status }>();
const emit = defineEmits<{ close: [] }>();

const library = useLibraryStore();
const lists = useListsStore();
const shelf = useShelfStore();

const editingId = ref<number | null>(props.gameId);
const original = ref<GameDetail | null>(null);

const title = ref("");
const yearText = ref("");
const links = ref<PlatformLink[]>([]);
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

const linkedPlatforms = computed<GamePlatform[]>(() =>
  links.value.map((link) => ({
    id: link.platformId,
    name: lists.platforms.find((p) => p.id === link.platformId)?.name ?? "?",
    state: link.state,
  })),
);
const availablePlatforms = computed(() =>
  lists.platforms.filter((p) => !links.value.some((l) => l.platformId === p.id)),
);
const derivedStatus = computed(() =>
  links.value.some((link) => link.state === "owned") ? "Owned" : "Interested",
);

const newLinkState = computed<PlatformState>(() =>
  editingId.value === null && props.defaultStatus === "owned" ? "owned" : "wanted",
);

onMounted(async () => {
  try {
    await lists.loadAll();
    if (props.gameId !== null) {
      const game = await gamesApi.getGame(props.gameId);
      original.value = game;
      title.value = game.title;
      yearText.value = game.releaseYear === null ? "" : String(game.releaseYear);
      links.value = game.platforms.map((p) => ({ platformId: p.id, state: p.state }));
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

function setLink(platformId: number, state: PlatformState | null) {
  const index = links.value.findIndex((link) => link.platformId === platformId);
  if (state === null) {
    if (index !== -1) links.value.splice(index, 1);
  } else if (index === -1) {
    links.value.push({ platformId, state });
  } else {
    links.value[index].state = state;
  }
}

async function createPlatform() {
  const name = newPlatform.value.trim();
  if (!name) return;
  try {
    const platform = await lists.addPlatform(name);
    if (!links.value.some((link) => link.platformId === platform.id)) {
      setLink(platform.id, newLinkState.value);
    }
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
        platforms: links.value,
        genreIds: genreIds.value,
      });
      id = created.id;
      editingId.value = id;
    } else {
      await gamesApi.updateGame(id, {
        title: title.value,
        releaseYear,
        platforms: links.value,
        genreIds: genreIds.value,
      });
    }

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
  <BaseDialog :title="heading" width="560px" :dismissible="false">
    <form @submit.prevent="save">
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
          <span>Platforms</span>
          <PlatformEditor :platforms="linkedPlatforms" :available="availablePlatforms" :new-state="newLinkState"
            @set="setLink" />
          <div class="row">
            <input v-model="newPlatform" type="text" placeholder="A platform that isn't listed…"
              @keydown.enter.prevent="createPlatform" />
            <BaseButton size="sm" @click="createPlatform">Create</BaseButton>
          </div>
          <p class="hint">
            Status: <strong>{{ derivedStatus }}</strong>. Click a platform to switch it between
            owned (solid) and wanted (dashed). A game is Owned when you own it on at least one
            platform.
          </p>
        </div>

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
            <img v-if="original?.coverPath && !coverFile" class="thumb cover"
              :src="resolveImage(original.coverPath) ?? undefined" alt="" />
            <BaseButton size="sm" @click="chooseCover">Choose cover…</BaseButton>
            <span v-if="coverFile" class="staged">{{ fileName(coverFile) }}</span>
          </div>
        </div>

        <div class="field">
          <span>Screenshots</span>
          <div class="thumbs">
            <div v-for="shot in existingScreenshots" :key="shot.id" class="thumb-wrap"
              :class="{ removed: removedScreenshotIds.includes(shot.id) }">
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
          <div>
            <BaseButton size="sm" @click="addScreenshots">Add screenshots…</BaseButton>
          </div>
        </div>

        <p v-if="error" class="error">{{ error }}</p>

        <div class="actions">
          <BaseButton type="submit" variant="primary" :disabled="saving">
            {{ saving ? "Saving…" : "Save" }}
          </BaseButton>
          <BaseButton :disabled="saving" @click="emit('close')">Cancel</BaseButton>

          <template v-if="editingId !== null">
            <BaseButton v-if="!confirmingDelete" variant="danger" @click="confirmingDelete = true">
              Delete…
            </BaseButton>
            <template v-else>
              <span>Delete this game and its images?</span>
              <BaseButton variant="danger" :disabled="saving" @click="removeGame">
                Yes, delete
              </BaseButton>
              <BaseButton @click="confirmingDelete = false">No</BaseButton>
            </template>
          </template>
        </div>
      </template>
    </form>
  </BaseDialog>
</template>

<style scoped>
.field {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  margin: 0 0 var(--space-4);
  padding: 0;
  border: none;
}

.field>span,
.field>legend {
  padding: 0;
  font-weight: 600;
  font-size: var(--font-size-sm);
}

.row {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.checks {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-1) var(--space-4);
}

.thumbs {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
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
  border-radius: var(--radius-sm);
  background: var(--color-hover);
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
  border-radius: 0 var(--radius-sm) 0 var(--radius-sm);
  background: var(--color-on-image-bg);
  color: var(--color-on-image);
  opacity: 1;
}

.staged-list {
  margin: 0;
  padding-left: var(--space-4);
  font-size: var(--font-size-sm);
}

.staged {
  font-size: var(--font-size-sm);
  color: var(--color-text-muted);
}

.actions {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: var(--space-2);
  margin-top: var(--space-4);
}
</style>