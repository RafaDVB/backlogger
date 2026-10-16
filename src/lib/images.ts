import { convertFileSrc } from "@tauri-apps/api/core";
import { getImagesDir } from "../api/images";

let imagesDir = "";

export async function initImages(): Promise<void> {
  imagesDir = (await getImagesDir()).replace(/\\/g, "/");
}

// Dev-only: draws a colored 16:9 image from the label, so we can test without real files.
function placeholderImage(label: string): string {
  let hue = 0;
  for (const ch of label) hue = (hue * 31 + ch.charCodeAt(0)) % 360;
  const svg =
    `<svg xmlns="http://www.w3.org/2000/svg" width="320" height="180">` +
    `<rect width="320" height="180" fill="hsl(${hue} 45% 35%)"/>` +
    `<text x="160" y="95" font-family="sans-serif" font-size="18" fill="white" ` +
    `text-anchor="middle">${label}</text></svg>`;
  return `data:image/svg+xml,${encodeURIComponent(svg)}`;
}

export function resolveImage(path: string | null): string | null {
  if (!path) return null;
  if (path.startsWith("placeholder:")) {
    return placeholderImage(path.slice("placeholder:".length));
  }
  if (/^(https?|data):/.test(path)) return path;
  return convertFileSrc(`${imagesDir}/${path}`);
}