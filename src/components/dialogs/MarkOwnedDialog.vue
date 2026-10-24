<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useListsStore } from "../../stores/lists";
import type { GamePlatform } from "../../types/game";
import BaseButton from "../ui/BaseButton.vue";
import BaseChip from "../ui/BaseChip.vue";
import BaseDialog from "../ui/BaseDialog.vue";

const props = defineProps<{ title: string; linked: GamePlatform[]; error: string | null }>();
const emit = defineEmits<{ confirm: [platformId: number]; cancel: [] }>();

const lists = useListsStore();

const selected = ref<number | null>(props.linked.length === 1 ? props.linked[0].id : null);

onMounted(() => lists.loadAll());

const others = computed(() =>
    lists.platforms.filter((p) => !props.linked.some((l) => l.id === p.id)),
);
const otherSelected = computed(
    () => selected.value !== null && !props.linked.some((l) => l.id === selected.value),
);

function onPickOther(event: Event) {
    const id = Number((event.target as HTMLSelectElement).value);
    if (id) selected.value = id;
}

function confirm() {
    if (selected.value !== null) emit("confirm", selected.value);
}
</script>

<template>
    <BaseDialog :title="`Mark ${title} as owned`" @close="emit('cancel')">
        <p>Which platform did you get it on?</p>

        <div v-if="linked.length" class="choices">
            <BaseChip v-for="platform in linked" :key="platform.id" :active="selected === platform.id"
                @click="selected = platform.id">
                {{ platform.name }}
            </BaseChip>
        </div>

        <label v-if="others.length" class="other">
            <span class="hint">{{ linked.length ? "Or another platform:" : "Platform:" }}</span>
            <select :value="otherSelected ? String(selected) : ''" @change="onPickOther">
                <option value="">choose…</option>
                <option v-for="platform in others" :key="platform.id" :value="String(platform.id)">
                    {{ platform.name }}
                </option>
            </select>
        </label>

        <p class="hint">
            That platform becomes owned. The game's other platforms stay on your wanted list.
        </p>
        <p v-if="error" class="error">{{ error }}</p>

        <template #footer>
            <BaseButton variant="primary" :disabled="selected === null" @click="confirm">
                Mark as owned
            </BaseButton>
            <BaseButton variant="quiet" @click="emit('cancel')">Cancel</BaseButton>
        </template>
    </BaseDialog>
</template>

<style scoped>
.choices {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
    margin-bottom: var(--space-3);
}

.other {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    margin-bottom: var(--space-3);
}
</style>