# Vinted-rs: A Vinted API wrapper

[![github]](https://github.com/TuTarea/vinted-rs/)&ensp;[![crates-io]](https://crates.io/crates/vinted-rs)&ensp;[![docs-rs]](https://docs.rs/vinted-rs/latest/vinted_rs/)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

## Table of Contents

- [Vinted-rs: A Vinted API wrapper](#vinted-rs-a-vinted-api-wrapper)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [DB setup](#db-setup)
    - [Create a migration](#create-a-migration)
    - [Run a Docker container with PostgreSQL](#run-a-docker-container-with-postgresql)
    - [Run migrations](#run-migrations)
    - [Stop DB](#stop-db)
  - [Running Tests](#running-tests)

## Installation

Via `cargo` you can add the library to your project's `Cargo.toml`

```toml
[dependencies]
vinted-rs = "0.3.2"
```

## DB setup

Advanced filtering features must require this setup before running.

- First start installing diesel-cli (in order to run the migrations in PostgreSQL database)

⚠️**Very important:** diesel-cli installation may fail if you do not have `libpq` library installed.

To install `libpq`, just install PostgreSQL package on your machine.

In `Arch` based is only necessary to install this package.

```bash
sudo pacman -S postgresql-libs
```

In `Debian` based distributions is only necessary to install this package.

```bash
sudo apt install libpq-dev
```

```bash
cargo install diesel_cli --features=postgres --no-default-features
```

### Create a migration

```bash
mkdir migrations
```

```bash
diesel migration generate my_migration
```

Program after that `up.sql` and `down.sql` scripts.

### Run a Docker container with PostgreSQL

- See in [Makefile](https://github.com/TuTarea/vinted-rs/blob/main/Makefile)

```bash
make db
```

### Run migrations

```bash
make diesel
```

### Stop DB

```bash
make stop
```

## Running Tests

⚠️**Very important:** Before running tests is important to do the [DB setup](#db-setup)

Then run the tests

```bash
cargo test
```
