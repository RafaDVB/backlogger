INSERT INTO genres (name) VALUES ('RPG'), ('Platformer'), ('Strategy'), ('Puzzle'), ('Roguelike');
INSERT INTO platforms (name) VALUES ('PC'), ('Switch'), ('PlayStation 5');

INSERT INTO games (title, release_year, status, outcome, rating_id, shelf_slot, notes) VALUES
  ('Hollow Knight',   2017, 'owned',      NULL,        NULL, 1,    'Stuck at the Mantis Lords.'),
  ('Celeste',         2018, 'owned',      'completed', 9,    NULL, ''),
  ('Hades',           2020, 'owned',      NULL,        NULL, 2,    ''),
  ('Disco Elysium',   2019, 'interested', NULL,        NULL, NULL, ''),
  ('Baba Is You',     2019, 'interested', NULL,        NULL, NULL, ''),
  ('Stardew Valley',  2016, 'owned',      'dropped',   6,    NULL, '');

INSERT INTO game_platforms (game_id, platform_id) VALUES
  (1, 1), (1, 2), (2, 1), (2, 2), (3, 1), (3, 2), (4, 1), (5, 1), (5, 2), (6, 1);

INSERT INTO game_genres (game_id, genre_id) VALUES
  (1, 2), (2, 2), (3, 5), (4, 1), (5, 4), (6, 1);

UPDATE games SET cover_path = 'placeholder:' || title || ' (cover)';

INSERT INTO screenshots (game_id, path, sort_order) VALUES
  (1, 'placeholder:Hollow Knight 1', 0), (1, 'placeholder:Hollow Knight 2', 1), (1, 'placeholder:Hollow Knight 3', 2),
  (2, 'placeholder:Celeste 1', 0),       (2, 'placeholder:Celeste 2', 1),
  (3, 'placeholder:Hades 1', 0),         (3, 'placeholder:Hades 2', 1),         (3, 'placeholder:Hades 3', 2),
  (4, 'placeholder:Disco Elysium 1', 0), (4, 'placeholder:Disco Elysium 2', 1),
  (5, 'placeholder:Baba Is You 1', 0);

-- Hades has a chosen featured screenshot; Stardew Valley has no screenshots at all.
UPDATE games SET featured_screenshot_id =
  (SELECT id FROM screenshots WHERE path = 'placeholder:Hades 2') WHERE id = 3;