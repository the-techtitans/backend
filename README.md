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

## Endpoints

|URL| Type | Description | Parameters | Authentication Needed?
---|---|---|---|---
|/find| GET | Finds doctors in city specified who can give appointment for specified appointment type | city, apptype (both as queries in URL) | No
|/prevapp | POST | Displays the previous appointments for particular patient | patient_id (POST request) | Yes
|/doctors | POST | Displays doctors in a particular city | city (POST request) | No
|/patient | POST | Displays info about patient | patient_id (POST request) | Yes
|/newpatient | POST | Adds patient details to database | name, phone, email | Will be used for signup process
