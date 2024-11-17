CREATE TABLE players(
player_id INTEGER AUTO_INCREMENT,
name VARCHAR(100),
league_id INTEGER,
PRIMARY_KEY(player_id)
);

CREATE TABLE leagues(
league_id INTEGER AUTO_INCREMENT,
league_name VARCHAR(100),
PRIMARY_KEY(league_id)
);

CREATE TABLE fixtures(
fixture_id INTEGER AUTO_INCREMENT,
season INTEGER,
league_id INTEGER,
player_one_id INTEGER,
player_two_id INTEGER,
player_one_set_one_games INTEGER,
player_two_set_one_games INTEGER,
player_one_set_two_games INTEGER,
player_two_set_two_games INTEGER,
player_one_tiebreak_games INTEGER,
player_two_tiebreak_games INTEGER,
PRIMARY_KEY(fixture_id)
);
