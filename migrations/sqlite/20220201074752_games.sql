CREATE TABLE IF NOT EXISTS games (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_version text NOT NULL,
    server_name text NOT NULL,
    server_ip text NOT NULL,
    server_port text NOT NULL,
    host_player text NOT NULL,
    map_name text NOT NULL,
    map_file text NOT NULL,
    variant text NOT NULL,
    variant_type text NOT NULL,
    team_game INTEGER NOT NULL,
    updated DATETIME DEFAULT CURRENT_TIMESTAMP
)
