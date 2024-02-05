-- Add migration script here
DROP TABLE IF EXISTS artists;
DROP TABLE IF EXISTS songs;

CREATE TABLE IF NOT EXISTS artists (
	uuid TEXT PRIMARY KEY,
  creator_uuid TEXT NOT NULL,
	artist_name TEXT NOT NULL,
	real_name TEXT,
  created_date DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS songs (
  uuid TEXT PRIMARY KEY,
  creator_uuid TEXT NOT NULL,
  title TEXT NOT NULL,
  created_date DATETIME NOT NULL
);

