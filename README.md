# Backend

This repo hosts the backend for the Excalibur 2023 project from team Tech Titans.

It is being written in Rust using [Axum web framework](https://lib.rs/crates/axum) and [sqlx](https://lib.rs/crates/sqlx) for interfacing with a Postgres database instance, along with other miscellaneous dependencies like Tokio.

## Setup

Make sure you have Postgres instance and Rust toolchain running on your system.

First, populate ```setup.env``` with DATABASE_URL according to [PostgreSQL standards](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING).

Then, rename ```setup.env``` to anything that begins with .env, like ```.env```.

Then, run the following commands related to creating the database and tables (one time measure to setup development environment):
```
createdb <dbname you gave in DATABASE_URL>
psql <dbname you gave in DATABASE_URL> -f src/schema.sql
```

Feel free to add some dummy data at this stage or use the dummy data contained in ```src/dummydata.sql``` to get some sample data.

Then, run the project using ```cargo run```. It will run on port 3000
