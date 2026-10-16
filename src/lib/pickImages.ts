import { open } from "@tauri-apps/plugin-dialog";

export async function pickImages(multiple: boolean): Promise<string[]> {
  const selected = await open({
    multiple,
    directory: false,
    filters: [{ name: "Images", extensions: ["png", "jpg", "jpeg", "webp", "gif"] }],
  });
  if (selected === null) return [];
  return Array.isArray(selected) ? selected : [selected];
}