CREATE TABLE IF NOT EXISTS players(
name VARCHAR(100),
league_id INTEGER
);

CREATE TABLE IF NOT EXISTS leagues(
league_name VARCHAR(100),
league_tier INTEGER
);

CREATE TABLE IF NOT EXISTS fixtures(
season INTEGER,
league_id INTEGER,
player_one_id INTEGER,
player_two_id INTEGER,
player_one_set_one_games INTEGER,
player_two_set_one_games INTEGER,
player_one_set_two_games INTEGER,
player_two_set_two_games INTEGER,
player_one_tiebreak_points INTEGER,
player_two_tiebreak_points INTEGER,
completed INTEGER DEFAULT 0,
winner INTEGER
);
