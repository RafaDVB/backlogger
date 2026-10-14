export type Status = "interested" | "owned";
export type Outcome = "completed" | "dropped";

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
  platforms: Platform[];
  genres: Genre[];
  screenshots: Screenshot[];
}

export interface GameFilter { status?: Status }