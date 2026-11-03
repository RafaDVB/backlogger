<script setup lang="ts">
withDefaults(
  defineProps<{
    variant?: "default" | "primary" | "danger" | "quiet";
    size?: "md" | "sm";
    type?: "button" | "submit";
  }>(),
  { variant: "default", size: "md", type: "button" },
);
</script>

<template>
  <button class="btn" :class="[`btn-${variant}`, `btn-${size}`]" :type="type">
    <slot />
  </button>
</template>

<style scoped>
.btn {
  font: inherit;
  color: var(--color-text);
  background: transparent;
  border: 1px solid var(--edge-2);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: border-color var(--transition);
}

.btn-md {
  padding: 4px 12px;
}

.btn-sm {
  padding: 2px 8px;
  font-size: var(--font-size-sm);
}

.btn:hover:not(:disabled),
.btn:focus-visible {
  background: var(--gradient-focus);
  border-color: var(--edge-3);
}

.btn:disabled {
  opacity: 0.45;
  cursor: default;
}

.btn-primary {
  background: linear-gradient(90deg, var(--accent-a), var(--accent-b));
  border-color: var(--edge-3);
  color: var(--color-accent-text);
}

.btn-primary:hover:not(:disabled),
.btn-primary:focus-visible {
  background: linear-gradient(90deg, var(--accent-a), var(--accent-b));
  filter: brightness(1.15);
}

.btn-danger {
  border-color: var(--color-danger);
  color: var(--color-danger);
}

.btn-danger:hover:not(:disabled),
.btn-danger:focus-visible {
  background: color-mix(in srgb, var(--color-danger) 25%, transparent);
  border-color: var(--color-danger);
}

.btn-quiet {
  border-color: transparent;
  color: var(--color-text-muted);
}
</style>