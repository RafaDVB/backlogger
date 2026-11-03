export type ThemeId = "ember" | "aqua" | "violet";
export type BackgroundId = "stars" | "rain" | "depths" | "ocean" | "off";

export interface Appearance {
    theme: ThemeId;
    background: BackgroundId;
    librarySize: number;
}

export const LIBRARY_SIZE = { min: 140, max: 420, default: 240 } as const;

/**
 * For the previews we have to repeat the palettes in vars.css.
 * New palettes have to be added in three places: vars.css, this theme list, and THEMES in
 * src-tauri/src/db/settings.rs.
 */
export const themes: {
    id: ThemeId;
    label: string;
    accent: [string, string];
    wall: [string, string];
}[] = [
        { id: "ember", label: "Ember", accent: ["#ffb03a", "#ff4d4d"], wall: ["#1d0a0a", "#6a2410"] },
        { id: "aqua", label: "Aqua", accent: ["#35d0ff", "#3a7bff"], wall: ["#04142b", "#0c5a6a"] },
        { id: "violet", label: "Violet", accent: ["#d27bff", "#ff5fa8"], wall: ["#14052e", "#5a1052"] },
    ];

export const backgrounds: { id: BackgroundId; label: string }[] = [
    { id: "stars", label: "Starry sky" },
    { id: "rain", label: "Rainy day" },
    { id: "depths", label: "Depths" },
    { id: "ocean", label: "Dot ocean" },
    { id: "off", label: "None" },
];

export function defaultAppearance(): Appearance {
    return { theme: "ember", background: "stars", librarySize: LIBRARY_SIZE.default };
}

export function applyTheme(theme: ThemeId) {
    document.documentElement.dataset.theme = theme;
}