# REMS: Rust Eldewrito Master Server

## Project Description
REMS is a Master Server and Ranking Server for the popular Halo Online mod called "ElDewrito", written in Rust.

## How To Install
### Install Prerequisites
* Git: ```sudo apt-get install git```
* cURL: ```sudo apt-get install curl```
* Rust/Cargo:
    1. ```curl https://sh.rustup.rs -sSf | sh```
    2. ```1``` Proceed with installation (default)
    3. ```source $HOME/.cargo/env```
* Build-tools:
    * for Arch Linux: ```sudo pacman -S base-devel```
    * for Debian/Ubuntu: ```sudo apt install build-essential```

### Install REMS
* Clone the repo.
```bash
git clone https://github.com/thebeerkeg/rust-eldewrito-master-server.git
cd rust-eldewrito-master-server
```

* Install sqlx-cli:
```bash
cargo install sqlx-cli
```

* Run database migrations in sqlx-cli:
```bash
sqlx mig run
```

* Build the source code.
```bash
cargo build --release
```

## Usage
* Run REMS once to create the `config.toml` file:
```bash
./target/release/rust-eldewrito-master-server
```

* Edit the newly created config.toml file according to your liking. Eg:
```toml
bind_address = "0.0.0.0:3000"

[master_server]
enabled = true
announce_endpoint = "announce"
list_endpoint = "list"
ed_announce_interval = 150
max_time_without_announce = 30
update_interval = 10

[ranking_server]
enabled = true
submit_endpoint = "submit"
stats_endpoint = "stats"
max_rank = 37
default_emblem = "http://thebeerkeg.net/img/Default.png"
```

* Run the torrust-tracker again:
```bash
./target/release/rust-eldewrito-master-server
```

## Benchmarks
[NodeJS]: https://github.com/ElDewrito/ElDewrito-MasterServer
JMeter Benchmarks (empty /list endpoint): Concurrent requests per second = average latency in miliseconds.

| Requests / second | Rust(ms) | [NodeJS]\(ms) |
|------------|------|--------|
|     1       |   1   |    2   |
|     10       |   1   |    2    |
|      50      |   1   |    2    |
|      250      |   2   |    13    |
|      500      |   2   |    50-80    |
|      1000      |   2   |    100-400    |
|      2000      |   2   |    100-400    |
|      10000      |   2   |    1000+ (died on 5000 requests)   |

As you can see here, the Rust based master server performs significantly better than the NodeJS based master server under heavy load. Not that it matters.

## Credits
🍺
