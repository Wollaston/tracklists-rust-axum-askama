-- Add migration script here
CREATE TABLE IF NOT EXISTS artists (
	artist_id INTEGER PRIMARY KEY,
	artist_name TEXT NOT NULL,
	real_name TEXT
);

INSERT INTO artists (artist_name, real_name)
VALUES 
  ("Tiësto", "Tijs Michiel Verwest"),
  ("Armin van Buuren", "Armin van Buuren");


