# Vinted-rs: A Vinted API wrapper

[![github]](https://github.com/TuTarea/vinted-rs/)&ensp;[![crates-io]](https://crates.io/crates/vinted-rs)&ensp;[![docs-rs]](https://docs.rs/vinted-rs/latest/vinted_rs/)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

## Table of Contents

- [Vinted-rs: A Vinted API wrapper](#vinted-rs-a-vinted-api-wrapper)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
    - [Environment set-up](#environment-set-up)
    - [Database set-up](#database-set-up)
    - [Testing set-up](#testing-set-up)
  - [Features](#features)
    - [Advanced filters](#advanced-filters)
    - [Redis](#redis)

## Installation

Via `cargo` you can add the library to your project's `Cargo.toml`

```toml
[dependencies]
vinted-rs = { version = "0.9.1", 
              features = ["advanced_filters", "redis"] }
```

### Environment set-up

1. Copy the `.env.example`

    ```sh
    cp .env.example .env
    ```

2. Modify the variables to your liking

### Database set-up
Advanced filtering features must require this setup before running.

1. ⚠️ `diesel-cli` installation may fail if you do not have `libpq` library installed. To install `libpq`, just install PostgreSQL package on your machine.

   - In `Arch` based is only necessary to install this package.

      ```bash
      sudo pacman -S postgresql-libs
      ```

   - In `Debian` based distributions is only necessary to install this package.

      ```bash
      sudo apt install libpq-dev
      ```

2. Install `diesel-cli` in order to run the migrations in PostgreSQL database
      
    ```bash
    cargo install diesel_cli --features=postgres --no-default-features
    ```

**Available interactions** (See [Makefile](./Makefile)) 

1. Create a migration

    ```bash
    mkdir -p migrations #
    diesel migration generate my_migration
    ```

    Program after that `up.sql` and `down.sql` scripts.

2. Run a Docker container with PostgreSQL

   - See in [Makefile](https://github.com/ThalosES/vinted-rs/blob/main/Makefile)

   ```bash
   make db
   ```

3. Run migrations

    ```bash
    make diesel
    ```

4. Stop DB

    ```bash
    make stop
    ```

### Testing set-up

> This step requires completing the [DB setup](#database-set-up)

```bash
cargo test
```

## Features

### Advanced filters

> This feature requires [setting up the Postgres DB](#database-set-up)

Uses the data pulled by the [scrapping module](./scrapping/vinted-db-feeder/), which is stored in the diesel [migrations](./migrations/) folder.

### Redis

This feature allows recovered results to be cached on a Redis instance.

A development instance can be created using:

```bash
make cache
```