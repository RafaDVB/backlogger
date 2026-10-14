CREATE TABLE ratings (
  id         INTEGER PRIMARY KEY,
  label      TEXT NOT NULL,
  sort_order INTEGER NOT NULL,
  color      TEXT
);

CREATE TABLE genres (
  id   INTEGER PRIMARY KEY,
  name TEXT NOT NULL UNIQUE
);

CREATE TABLE platforms (
  id   INTEGER PRIMARY KEY,
  name TEXT NOT NULL UNIQUE
);

CREATE TABLE games (
  id                     INTEGER PRIMARY KEY,
  title                  TEXT NOT NULL,
  release_year           INTEGER,
  status                 TEXT NOT NULL CHECK (status IN ('interested', 'owned')),
  outcome                TEXT CHECK (outcome IN ('completed', 'dropped')),
  rating_id              INTEGER REFERENCES ratings(id) ON DELETE SET NULL,
  shelf_slot             INTEGER UNIQUE CHECK (shelf_slot BETWEEN 1 AND 6),
  featured_screenshot_id INTEGER REFERENCES screenshots(id) ON DELETE SET NULL,
  notes                  TEXT NOT NULL DEFAULT '',
  cover_path             TEXT,
  rawg_id                INTEGER UNIQUE,
  CHECK (shelf_slot IS NULL OR status = 'owned'),
  CHECK (outcome IS NULL OR status = 'owned')
);

CREATE TABLE screenshots (
  id         INTEGER PRIMARY KEY,
  game_id    INTEGER NOT NULL REFERENCES games(id) ON DELETE CASCADE,
  path       TEXT NOT NULL,
  sort_order INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX idx_screenshots_game ON screenshots(game_id);

CREATE TABLE game_platforms (
  game_id     INTEGER NOT NULL REFERENCES games(id) ON DELETE CASCADE,
  platform_id INTEGER NOT NULL REFERENCES platforms(id) ON DELETE CASCADE,
  PRIMARY KEY (game_id, platform_id)
);

CREATE TABLE game_genres (
  game_id  INTEGER NOT NULL REFERENCES games(id) ON DELETE CASCADE,
  genre_id INTEGER NOT NULL REFERENCES genres(id) ON DELETE CASCADE,
  PRIMARY KEY (game_id, genre_id)
);

INSERT INTO ratings (label, sort_order) VALUES
  ('1', 1), ('2', 2), ('3', 3), ('4', 4), ('5', 5),
  ('6', 6), ('7', 7), ('8', 8), ('9', 9), ('10', 10);