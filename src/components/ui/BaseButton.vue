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
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: background var(--transition);
}
.btn-md {
  padding: 6px 14px;
}
.btn-sm {
  padding: 3px 10px;
  font-size: var(--font-size-sm);
}
.btn:hover:not(:disabled) {
  background: var(--color-hover);
}
.btn:disabled {
  opacity: 0.45;
  cursor: default;
}
.btn-primary {
  background: var(--color-accent);
  border-color: var(--color-accent);
  color: var(--color-accent-text);
}
.btn-primary:hover:not(:disabled) {
  background: var(--color-accent);
  filter: brightness(1.1);
}
.btn-danger {
  border-color: var(--color-danger);
  color: var(--color-danger);
}
.btn-quiet {
  border-color: transparent;
  color: var(--color-text-muted);
}
</style>