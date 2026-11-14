import { onBeforeUnmount, onMounted } from "vue";

const EXTENSIONS: Record<string, string> = {
  "image/png": "png",
  "image/jpeg": "jpg",
  "image/webp": "webp",
  "image/gif": "gif",
};

// WebKitGTK before 2.54 exposes no files, items or types in paste events for
// clipboard images (side effect of the CVE-2025-13947 mitigation), but the
// async Clipboard API still returns them. Only WebKitGTK needs this workaround,
// and limiting it to Linux avoids clipboard permission prompts elsewhere.
const canReadClipboardDirectly =
  typeof navigator !== "undefined" && navigator.userAgent.includes("Linux");

export interface PastedImage {
  bytes: Uint8Array;
  extension: string;
}

async function readClipboardImages(): Promise<PastedImage[]> {
  if (!navigator.clipboard?.read) return [];

  const images: PastedImage[] = [];
  for (const item of await navigator.clipboard.read()) {
    const type = item.types.find((candidate) => candidate in EXTENSIONS);
    if (!type) continue;
    try {
      const blob = await item.getType(type);
      images.push({
        bytes: new Uint8Array(await blob.arrayBuffer()),
        extension: EXTENSIONS[type],
      });
    } catch {
      // The item changed while reading it; skip it and keep the others.
    }
  }
  return images;
}

export function usePasteImages(onImages: (images: PastedImage[]) => void | Promise<void>) {
  async function handle(event: ClipboardEvent) {
    const files = Array.from(event.clipboardData?.files ?? []).filter(
      (file) => file.type in EXTENSIONS,
    );

    let images: PastedImage[];
    if (files.length > 0) {
      event.preventDefault();
      images = await Promise.all(
        files.map(async (file) => ({
          bytes: new Uint8Array(await file.arrayBuffer()),
          extension: EXTENSIONS[file.type],
        })),
      );
    } else if (canReadClipboardDirectly) {
      // Must run inside the paste gesture, while it still has user activation.
      try {
        images = await readClipboardImages();
      } catch (error) {
        // Expected when there is no image on the clipboard or permission is denied.
        if (
          !(
            error instanceof DOMException &&
            (error.name === "NotAllowedError" || error.name === "AbortError")
          )
        ) {
          console.warn("Could not read an image from the clipboard", error);
        }
        return;
      }
      if (images.length === 0) return;
    } else {
      return;
    }

    await onImages(images);
  }

  onMounted(() => document.addEventListener("paste", handle));
  onBeforeUnmount(() => document.removeEventListener("paste", handle));
}
