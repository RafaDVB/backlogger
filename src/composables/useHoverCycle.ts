import { computed, onBeforeUnmount, ref, toValue, type MaybeRefOrGetter } from "vue";

export function useHoverCycle(images: MaybeRefOrGetter<string[]>, intervalMs = 900) {
  const index = ref(0);
  let timer: number | undefined;

  const current = computed(() => {
    const list = toValue(images);
    return list[index.value] ?? list[0] ?? null;
  });

  function start() {
    if (toValue(images).length < 2 || timer !== undefined) return;
    timer = window.setInterval(() => {
      index.value = (index.value + 1) % toValue(images).length;
    }, intervalMs);
  }

  function stop() {
    if (timer !== undefined) {
      window.clearInterval(timer);
      timer = undefined;
    }
    index.value = 0;
  }

  onBeforeUnmount(stop);

  return { current, start, stop };
}