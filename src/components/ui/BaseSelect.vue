<script setup lang="ts" generic="T extends string | number">
import { computed, nextTick, onBeforeUnmount, onMounted, ref } from "vue";
import BaseButton from "./BaseButton.vue";
import PixelIcon from "./PixelIcon.vue";

const props = defineProps<{
    options: { value: T; label: string }[];
    modelValue: T | null;
    placeholder?: string;
    disabled?: boolean;
}>();
const emit = defineEmits<{ "update:modelValue": [value: T] }>();

const open = ref(false);
const cursor = ref(0);
const root = ref<HTMLElement | null>(null);
const popupEl = ref<HTMLElement | null>(null);
const popupStyle = ref<Record<string, string>>({});

const currentLabel = computed(
    () => props.options.find((o) => o.value === props.modelValue)?.label ?? props.placeholder ?? "—",
);

function place() {
    const rect = root.value!.getBoundingClientRect();
    const below = window.innerHeight - rect.bottom - 8;
    const above = rect.top - 8;
    const up = below < 160 && above > below;
    popupStyle.value = {
        left: `${Math.round(rect.left)}px`,
        minWidth: `${Math.max(Math.round(rect.width), 140)}px`,
        maxHeight: `${Math.max(80, Math.min(300, Math.round(up ? above : below)))}px`,
        top: up ? "auto" : `${Math.round(rect.bottom) + 4}px`,
        bottom: up ? `${Math.round(window.innerHeight - rect.top) + 4}px` : "auto",
    };
}

function openMenu() {
    if (props.disabled) return;
    cursor.value = Math.max(0, props.options.findIndex((o) => o.value === props.modelValue));
    place();
    open.value = true;
    scrollToCursor();
}

function choose(index: number) {
    const option = props.options[index];
    if (option) emit("update:modelValue", option.value);
    open.value = false;
}

function move(step: number) {
    const count = props.options.length;
    if (count === 0) return;
    cursor.value = (cursor.value + step + count) % count;
    scrollToCursor();
}

async function scrollToCursor() {
    await nextTick();
    popupEl.value?.querySelectorAll(".popup-row")[cursor.value]?.scrollIntoView({ block: "nearest" });
}

function onKeydown(event: KeyboardEvent) {
    if (props.disabled) return;

    if (!open.value) {
        if (event.key === "ArrowDown" || event.key === "ArrowUp") {
            event.preventDefault();
            openMenu();
        }
        return;
    }

    switch (event.key) {
        case "ArrowDown":
            event.preventDefault();
            move(1);
            break;
        case "ArrowUp":
            event.preventDefault();
            move(-1);
            break;
        case "Home":
            event.preventDefault();
            cursor.value = 0;
            scrollToCursor();
            break;
        case "End":
            event.preventDefault();
            cursor.value = props.options.length - 1;
            scrollToCursor();
            break;
        case "Enter":
        case " ":
            event.preventDefault();
            choose(cursor.value);
            break;
        case "Escape":
            event.stopPropagation();
            open.value = false;
            break;
        case "Tab":
            open.value = false;
            break;
        default:
            if (event.key.length === 1 && !event.ctrlKey && !event.metaKey && !event.altKey) {
                const letter = event.key.toLowerCase();
                const count = props.options.length;
                for (let step = 1; step <= count; step++) {
                    const index = (cursor.value + step) % count;
                    if (props.options[index]!.label.toLowerCase().startsWith(letter)) {
                        cursor.value = index;
                        scrollToCursor();
                        break;
                    }
                }
            }
    }
}

function onDocumentClick(event: MouseEvent) {
    const target = event.target as Node;
    if (root.value?.contains(target) || popupEl.value?.contains(target)) return;
    open.value = false;
}

function onScroll(event: Event) {
    if (popupEl.value?.contains(event.target as Node)) return;
    open.value = false;
}

function close() {
    open.value = false;
}

onMounted(() => {
    document.addEventListener("click", onDocumentClick);
    window.addEventListener("scroll", onScroll, true);
    window.addEventListener("resize", close);
});
onBeforeUnmount(() => {
    document.removeEventListener("click", onDocumentClick);
    window.removeEventListener("scroll", onScroll, true);
    window.removeEventListener("resize", close);
});
</script>

<template>
    <div ref="root" class="select" @keydown="onKeydown">
        <BaseButton size="sm" :disabled="disabled" aria-haspopup="listbox" :aria-expanded="open"
            @click="open ? (open = false) : openMenu()">
            <span>{{ currentLabel }}</span>
            <PixelIcon name="tri-down" />
        </BaseButton>

        <Teleport to="body">
            <div v-if="open && !disabled" ref="popupEl" class="popup" role="listbox" :style="popupStyle"
                @mousedown.prevent>
                <div v-for="(option, index) in options" :key="String(option.value)" class="popup-row"
                    :class="{ cursor: index === cursor }" role="option" :aria-selected="option.value === modelValue"
                    @mouseenter="cursor = index" @click="choose(index)">
                    <i class="mark" :class="{ on: option.value === modelValue }"></i>
                    {{ option.label }}
                </div>
            </div>
        </Teleport>
    </div>
</template>

<style scoped>
.select {
    position: relative;
    display: inline-block;
}

.popup {
    position: fixed;
    z-index: 20;
}

.mark {
    flex: none;
    width: 10px;
    height: 10px;
    border: 1px solid var(--edge-1);
    background: var(--fill-1);
}

.mark.on {
    border-color: var(--edge-3);
    background: linear-gradient(135deg, var(--accent-a), var(--accent-b)), var(--fill-3);
}
</style>