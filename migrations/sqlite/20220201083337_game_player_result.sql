CREATE TABLE IF NOT EXISTS game_player_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_id INTEGER NOT NULL,
    uid text NOT NULL,
    team INTEGER NOT NULL,
    player_index INTEGER NOT NULL,
    score INTEGER NOT NULL,
    kills INTEGER NOT NULL,
    assists INTEGER NOT NULL,
    deaths INTEGER NOT NULL,
    betrayals INTEGER NOT NULL,
    time_spent_alive INTEGER NOT NULL,
    suicides INTEGER NOT NULL,
    best_streak INTEGER NOT NULL,
    exp INTEGER NOT NULL DEFAULT 0,
    updated DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(game_id) REFERENCES games(id),
    FOREIGN KEY(uid) REFERENCES players(uid) ON UPDATE CASCADE
)
