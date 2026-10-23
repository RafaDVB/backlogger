export type Status = "interested" | "owned";
export type Outcome = "completed" | "dropped";
export type PlatformState = "owned" | "wanted";

export interface PlatformLink {
  platformId: number;
  state: PlatformState;
}

export interface GamePlatform {
  id: number;
  name: string;
  state: PlatformState;
}

export interface GameSummary {
  id: number;
  title: string;
  releaseYear: number | null;
  status: Status;
  outcome: Outcome | null;
  shelfSlot: number | null;
  coverPath: string | null;
  featuredPath: string | null;
  screenshotPaths: string[];
}

export interface Rating { id: number; label: string; sortOrder: number; color: string | null }
export interface Genre { id: number; name: string }
export interface Platform { id: number; name: string }
export interface Screenshot { id: number; path: string; sortOrder: number }


export interface GameDetail {
  id: number;
  title: string;
  releaseYear: number | null;
  status: Status;
  outcome: Outcome | null;
  shelfSlot: number | null;
  coverPath: string | null;
  featuredScreenshotId: number | null;
  notes: string;
  rawgId: number | null;
  rating: Rating | null;
  platforms: GamePlatform[];
  genres: Genre[];
  screenshots: Screenshot[];
}

export interface GameFilter { status?: Status, onShelf?: boolean }

export interface NewGame {
  title: string;
  releaseYear: number | null;
  platforms: PlatformLink[];
  genreIds: number[];
}

export interface GameEdit {
  title: string;
  releaseYear: number | null;
  platforms: PlatformLink[];
  genreIds: number[];
}

export interface PreviewGame {
  id: number;
  title: string;
  releaseYear: number | null;
}

export interface DeletePreview {
  total: number;
  first: PreviewGame[];
  blocked: string[];
}

export interface Settings {
  rawgApiKey: string | null;
}