import { ref } from "vue";
import type { Status } from "../types/game";

export interface DragPayload {
  source: "grid" | "shelf";
  gameId: number;
  status: Status;
  fromSlot: number | null;
}

// Module-level, so every component shares the same value. Browsers don't let a drop
// target read the dragged data until the drop, but we need it earlier to show refusals.
const dragging = ref<DragPayload | null>(null);

export function useDragDrop() {
  function startDrag(event: DragEvent, payload: DragPayload) {
    dragging.value = payload;
    if (event.dataTransfer) {
      event.dataTransfer.effectAllowed = "move";
      event.dataTransfer.setData("text/plain", String(payload.gameId)); // some webviews need this to start a drag
    }
  }

  function endDrag() {
    dragging.value = null;
  }

  return { dragging, startDrag, endDrag };
}

// dragleave also fires when moving onto a child element; this tells the real exits apart.
export function isLeaving(event: DragEvent): boolean {
  return !(event.currentTarget as Node).contains(event.relatedTarget as Node | null);
}