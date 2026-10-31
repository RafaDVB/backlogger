<script setup lang="ts" generic="T extends string | number | null">
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import BaseButton from "./BaseButton.vue";

const props = defineProps<{
    label: string;
    options: { value: T; label: string }[];
    modelValue: T[];
}>();
const emit = defineEmits<{ "update:modelValue": [value: T[]] }>();

const open = ref(false);
const query = ref("");
const root = ref<HTMLElement | null>(null);

const shown = computed(() => {
    const text = query.value.trim().toLowerCase();
    return text ? props.options.filter((o) => o.label.toLowerCase().includes(text)) : props.options;
});

function toggle(value: T) {
    emit(
        "update:modelValue",
        props.modelValue.includes(value)
            ? props.modelValue.filter((v) => v !== value)
            : [...props.modelValue, value],
    );
}

function onDocumentClick(event: MouseEvent) {
    if (root.value && !root.value.contains(event.target as Node)) open.value = false;
}

onMounted(() => document.addEventListener("click", onDocumentClick));
onBeforeUnmount(() => document.removeEventListener("click", onDocumentClick));
</script>

<template>
    <div ref="root" class="multi" @keydown.esc="open = false">
        <BaseButton size="sm" @click="open = !open">
            {{ label }}{{ modelValue.length ? ` (${modelValue.length})` : "" }} ▾
        </BaseButton>

        <div v-if="open" class="dropdown">
            <input v-if="options.length > 8" v-model="query" type="text" class="query"
                :placeholder="`Search ${label.toLowerCase()}…`" />
            <label v-for="option in shown" :key="String(option.value)" class="option">
                <input type="checkbox" :checked="modelValue.includes(option.value)" @change="toggle(option.value)" />
                {{ option.label }}
            </label>
            <p v-if="shown.length === 0" class="hint">Nothing matches.</p>
            <BaseButton v-if="modelValue.length" size="sm" variant="quiet" @click="emit('update:modelValue', [])">
                Clear
            </BaseButton>
        </div>
    </div>
</template>

<style scoped>
.multi {
    position: relative;
    display: inline-block;
}

.dropdown {
    position: absolute;
    top: calc(100% + var(--space-1));
    left: 0;
    z-index: 5;
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    min-width: 220px;
    max-height: 300px;
    overflow-y: auto;
    padding: var(--space-2);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-surface-raised);
    box-shadow: 0 4px 16px var(--color-overlay);
}

.query {
    margin-bottom: var(--space-1);
}

.option {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: 2px var(--space-1);
    border-radius: var(--radius-sm);
    cursor: pointer;
}

.option:hover {
    background: var(--color-hover);
}
</style>