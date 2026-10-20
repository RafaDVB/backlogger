import { onBeforeUnmount, ref, watch, type Ref } from "vue";

export type SaveState = "idle" | "pending" | "saving" | "saved" | "error";

export function useAutosave(
    text: Ref<string>,
    save: (value: string) => Promise<void>,
    delayMs = 700,
) {
    const state = ref<SaveState>("idle");
    let lastSaved = text.value;
    let timer: number | undefined;

    function cancelTimer() {
        if (timer !== undefined) {
            window.clearTimeout(timer);
            timer = undefined;
        }
    }

    async function flush() {
        cancelTimer();
        if (text.value === lastSaved) return;
        const value = text.value;
        state.value = "saving";
        try {
            await save(value);
            lastSaved = value;
            state.value = text.value === value ? "saved" : "pending";
        } catch {
            state.value = "error";
        }
    }

    watch(text, () => {
        if (text.value === lastSaved) return;
        state.value = "pending";
        cancelTimer();
        timer = window.setTimeout(flush, delayMs);
    });

    function reset(value: string) {
        cancelTimer();
        lastSaved = value;
        text.value = value;
        state.value = "idle";
    }

    onBeforeUnmount(flush);

    return { state, flush, reset };
}