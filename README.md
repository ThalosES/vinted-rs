# Vinted-rs: A Vinted API wrapper

A complete Vinted API-Wrapper in Rust

## Installation

Via `cargo` you can add the library to your project's `Cargo.toml`

```toml
[dependencies]
vinted-rs = "0.0.1"
```

## Authors

[Álvaro Cabo](https://github.com/alvarocabo)

[Pepe Márquez](https://github.com/pxp9)

## DB setup

Advanced filtering features must require this setup before running.

- First start installing diesel-cli (in order to run the migrations in PostgreSQL database)

### VERY IMPORTANT

diesel-cli installation may fail if you do not have `libpq` library installed.

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

### Very important

Before running tests is important to do the DB setup

Then run the tests

```bash
cargo test
```