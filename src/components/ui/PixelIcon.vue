<script setup lang="ts">
import arrDown from "../../assets/img/arr_down.png";
import arrLeft from "../../assets/img/arr_left.png";
import arrRight from "../../assets/img/arr_right.png";
import arrUp from "../../assets/img/arr_up.png";
import redo from "../../assets/img/redo.png";
import triDown from "../../assets/img/tri_down.png";
import triLeft from "../../assets/img/tri_left.png";
import triRight from "../../assets/img/tri_right.png";
import triUp from "../../assets/img/tri_up.png";

type IconName =
    | "arr-down"
    | "arr-left"
    | "arr-right"
    | "arr-up"
    | "redo"
    | "tri-down"
    | "tri-left"
    | "tri-right"
    | "tri-up";

const ICONS: Record<IconName, string> = {
    "arr-down": arrDown,
    "arr-left": arrLeft,
    "arr-right": arrRight,
    "arr-up": arrUp,
    redo,
    "tri-down": triDown,
    "tri-left": triLeft,
    "tri-right": triRight,
    "tri-up": triUp,
};

withDefaults(defineProps<{ name: IconName; size?: number }>(), { size: 8 });
</script>

<template>
    <img class="pixel-icon" :src="ICONS[name]" :width="size" :height="size" alt="" aria-hidden="true"
        draggable="false" :style="{ width: `${size}px`, height: `${size}px` }" />
</template>

<style scoped>
.pixel-icon {
    display: block;
    flex: none;
    image-rendering: pixelated;
}
</style>
