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

        <div v-if="open" class="popup">
            <input v-if="options.length > 8" v-model="query" type="text" class="query"
                :placeholder="`Search ${label.toLowerCase()}…`" />
            <label v-for="option in shown" :key="String(option.value)" class="popup-row">
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

.query {
    margin-bottom: var(--space-1);
}
</style>