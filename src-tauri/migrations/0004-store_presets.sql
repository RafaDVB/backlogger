INSERT OR IGNORE INTO platforms (name) VALUES
  ('Steam'), ('Epic Games Store'), ('GOG'), ('itch.io'), ('Microsoft Store'),
  ('Battle.net'), ('EA app'), ('Ubisoft Connect');

DELETE FROM platforms
WHERE name IN ('PC', 'macOS', 'Linux')
  AND id NOT IN (SELECT platform_id FROM game_platforms);