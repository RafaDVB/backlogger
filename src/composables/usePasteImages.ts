import { onBeforeUnmount, onMounted } from "vue";

const EXTENSIONS: Record<string, string> = {
  "image/png": "png",
  "image/jpeg": "jpg",
  "image/webp": "webp",
  "image/gif": "gif",
};

export interface PastedImage {
  bytes: Uint8Array;
  extension: string;
}

export function usePasteImages(onImages: (images: PastedImage[]) => void | Promise<void>) {
  async function handle(event: ClipboardEvent) {
    const files = Array.from(event.clipboardData?.files ?? []).filter(
      (file) => file.type in EXTENSIONS,
    );
    if (files.length === 0) return;

    event.preventDefault();
    const images = await Promise.all(
      files.map(async (file) => ({
        bytes: new Uint8Array(await file.arrayBuffer()),
        extension: EXTENSIONS[file.type],
      })),
    );
    await onImages(images);
  }

  onMounted(() => document.addEventListener("paste", handle));
  onBeforeUnmount(() => document.removeEventListener("paste", handle));
}