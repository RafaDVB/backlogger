ALTER TABLE game_platforms
  ADD COLUMN state TEXT NOT NULL DEFAULT 'wanted' CHECK (state IN ('owned', 'wanted'));

UPDATE game_platforms SET state = 'owned'
WHERE game_id IN (SELECT id FROM games WHERE status = 'owned');

INSERT OR IGNORE INTO platforms (name)
SELECT 'Unspecified'
WHERE EXISTS (
  SELECT 1 FROM games g
  WHERE g.status = 'owned'
    AND NOT EXISTS (SELECT 1 FROM game_platforms gp WHERE gp.game_id = g.id)
);

INSERT INTO game_platforms (game_id, platform_id, state)
SELECT g.id, (SELECT id FROM platforms WHERE name = 'Unspecified'), 'owned'
FROM games g
WHERE g.status = 'owned'
  AND NOT EXISTS (SELECT 1 FROM game_platforms gp WHERE gp.game_id = g.id);