# Vinted-rs: A Vinted API wrapper

A complete Vinted API-Wrapper in Rust

## Instalation

Via `cargo` you can add the library to your project's `Cargo.toml`

```toml
[dependencies]
vinted-rs = "0.0.1"
```

## Authors

[Álvaro Cabo](https://github.com/alvarocabo)

[Pepe Márquez](https://github.com/pxp9)

## DB setup

Advanced filters feature must require this setup before run.

- First start installing diesel-cli (in order to run the migrations in PostgreSQL database)

### VERY IMPORTANT

diesel-cli installation may fail if you do not have `libpq` library installed.

To install `libpq`, just install PostgreSQL package in you machine.

In `Arch` based is only necessary to install this package.

```
sudo pacman -S postgresql-libs
```

In `Debian` based distributions is only necessary to install this package.

```
sudo apt install libpq5
```


```
cargo install diesel_cli --features=postgres --no-default-features
```

### Create a migration

```
mkdir migrations
```

```
diesel migration generate my_migration
```

Program after that `up.sql` and `down.sql` scripts.

### Run a Docker container with PostgreSQL

- See in [Makefile](https://github.com/TuTarea/vinted-rs/blob/main/Makefile)

```
make db
```

### Run migrations

```
make diesel
```

### Stop DB
```
make stop
```

