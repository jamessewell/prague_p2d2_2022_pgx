# Prague P2D2 2022 Conference Talk Demo Repo

This repo hosts the demo code and slides from my talk at the Prague Postgres Developer Day 2022.

The talk synopsis is:

> PostgreSQL provides a lot of functionality to create custom aggregates, a design feature that’s essential to the success of TimescaleDB. Our users often find themselves wanting to extend and build upon the aggregate functions we provide.
>
> However, the prospect of creating a performant aggregate function in C, along with a potential of being the long-term maintainer of a new function, can be a high barrier for many would-be contributors.
>
> But there’s great news! Not only does PGX help with creating, testing and deploying aggregates using Rust, it’s easy enough for me to fit the adventure of addressing one GitHub user’s call for help into 25 minutes.

## Setup

To use the code in this repo follow these steps:

```
# Install the Rust toolset (if you don't already have it) 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install cargo-pgx into cargo (the Rust package manager)
cargo install cargo-pgx

# Intialize our locall PGX managed PostgreSQL versions
cargo pgx init

# Clone this repo ready for use!
git clone https://github.com/jamessewell/prague_p2d2_2022_pgx
cd prague_p2d2_2022_pgx
```

Now we are ready to use PGX!

## PGX Run

To compile our code for a specific version of Postgres and enter a `psql` shell with the extension ready to use you can run the following command:

```
cargo pgx run --target pg14
```

When you are dropped into the `psql` prompt you can run the following to create the extension, create some demo data, and finally show the extension working vs a PL/pgSQL version (which was installed from that weather.sql file).

```
CREATE EXTENSION prague_p2d2_2022_pgx;
\i ./sql/weather.sql
\timing
SELECT max_timed(instant, temperature) FROM weather;
SELECT max_timed_pgx(instant, temperature) FROM weather;
```

If you'd like you can run the `cargo pgx run` command again using `pg13` to drop into a PostgreSQL 13 shell.

## PGX Test

We have some tests build into our code, we can run these with:

```
cargo pgx test pg14
```

You will see a test harness start up and confirm that the tests were successful.

## PGX Schema

We can also inspect the schema objects which will be created as part of the extension with:

```
cargo pgx schema pg14
```

## PGX Package

Finally if we have a `pg_config` from another PostgreSQL installation (using the same architecture) then we can create a directory tree which we can copy directly to that machine with the following command:

```
cargo pgx package -g /path/to/pg_config
```

You will then see the correct tree has been made under the `./target/release/prague_p2d2_2022_pgx-pgXX/` directory.

## Benchmarking

You can look at some basic benchmarking by running the following command (assuming you've compiled and installed the extension into PG14):

```
/.pgx/14.3/pgx-install/bin/pgbench -t 100 -p 28814 -h 127.0.0.1 -r -n  -f pgbench.sql prague_p2d2_2022_pgx
```



