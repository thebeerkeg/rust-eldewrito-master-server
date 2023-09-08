CREATE TABLE IF NOT EXISTS game_team_results (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    game_id INTEGER NOT NULL,
    one INTEGER NOT NULL DEFAULT 0,
    two INTEGER NOT NULL DEFAULT 0,
    three INTEGER NOT NULL DEFAULT 0,
    four INTEGER NOT NULL DEFAULT 0,
    five INTEGER NOT NULL DEFAULT 0,
    six INTEGER NOT NULL DEFAULT 0,
    seven INTEGER NOT NULL DEFAULT 0,
    eight INTEGER NOT NULL DEFAULT 0,
    updated DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(game_id) REFERENCES games(id)
)
