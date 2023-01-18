# Backend

This repo hosts the backend for the Excalibur 2023 project from team Tech Titans.

It is being written in Rust using [Axum web framework](https://lib.rs/crates/axum) and [sqlx](https://lib.rs/crates/sqlx) for interfacing with a Postgres database instance, along with other miscellaneous dependencies like Tokio.

## Setup

Make sure you have Postgres instance and Rust toolchain running on your system.

First, populate ```setup.env``` with DATABASE_URL according to [PostgreSQL standards](https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING), and a SECRET (which is a random string which will be used to generate JWTs)

Then, rename ```setup.env``` to anything that begins with .env, like ```.env```.

Then, run the following commands related to creating the database and tables (one time measure to setup development environment):
```
createdb <dbname you gave in DATABASE_URL>
psql <dbname you gave in DATABASE_URL> -f src/schema.sql
```

Feel free to add some dummy data at this stage or use the dummy data contained in ```src/dummydata.sql``` to get some sample data by running the following command:

```
psql <dbname you gave in DATABASE_URL> -f src/dummydata.sql
```

Note: this does NOT contain a single record for the login table! You will need to use the ```/newdoctor``` or ```/newpatient``` endpoints to create a new doctor/patient which will also insert into these tables. You can then use these credentials in the API testing to make sure authentication works as intended

Then, run the project using ```cargo run```. It will run on port 3000. For log messages, use the ```RUST_LOG``` env variable (setting to debug usually prints good messages to understand what is going on)

## Endpoints

|URL| Type | Description | Parameters | Authentication Needed?
---|---|---|---|---
|/find| GET | Finds doctors in city specified who can give appointment for specified appointment type | city, apptype (both as queries in URL) | No
|/prevapp | POST | Displays the previous appointments for particular patient | patient_id (POST request) | Yes
|/doctors | POST | Displays doctors in a particular city | city (POST request) | No
|/patient | POST | Displays info about patient | patient_id (POST request) | Yes
|/newpatient | POST | Adds patient details to database | name, phone, email, password | Will be used for signup process
|/specialities | GET | Gets speciality details | Nothing | No
|/apptypes | GET | Gets appointment types | Nothing | No
|/cities | GET | Gets all cities where doctors are available according to us | Nothing | No
|/newdoctor | POST | Adds doctor details to database | name, speciality (as an ID), city, address, phone, email, password | Will be used for signup process
|/newappointment | POST | Add new appointment to database | doctor_id, patient_id, apptype (as an ID), datetime (specific format of YYYY-MM-DD and then 24 hour HH:MM:SS), phyorvirt (just write either physical or virtual checkup), status (cancelled, fulfilled, scheduled), prescription | Yes
|/login | POST | Generate JWT for a user (doctor or patient) | email, password | No (JWT is used as token to get authentication implemented)
|/prescriptions | POST | Get the doctor name, date and time, and prescription text previously given | patient_id | Yes
|/doctorappointments | POST | Gets the doctor's appointments | patient_id (it recycles the same struct so just name it as such, it is interpreted as a doctor's ID only) | Yes

## Response Codes

|Number|Name|Description|
---|---|---
200|OK|Everything checked out, request is good
500|Internal Server Error| There is a problem with connecting to the database
401| Unauthorized| You didn't provide the right authorization token (the JWT) or it was not provided properly. In whatever case, you don't have the right to view what you requested so it was denied
400| Bad Request | This is returned whenever the database has no records for your request. It's intended as a shorthand to save you time to check whether you received *any* records
405 | Method Not Allowed| You should only make a POST request to an endpoint that expects a POST request and a GET request to one that expects a GET request
