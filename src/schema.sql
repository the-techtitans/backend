-- - info about various specialities
CREATE TABLE IF NOT EXISTS Specialities (
    id INT PRIMARY KEY ,
    name VARCHAR(255) NOT NULL,
    description TEXT
);

-- - info about doctors registered in system
CREATE TABLE IF NOT EXISTS Doctors (
    id INT PRIMARY KEY ,
    name VARCHAR(255) NOT NULL,
    speciality_id INT NOT NULL,
    city VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL,
    FOREIGN KEY (speciality_id) REFERENCES Specialities(id)
);

-- - generic appointment types stored here with some info about them and
-- - restricted to specialities; doesn't make sense for pediatrician to provide
-- - dental services for example
CREATE TABLE IF NOT EXISTS Appointment_Types (
    id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    speciality_id INT NOT NULL,
    description TEXT,
    FOREIGN KEY (speciality_id) REFERENCES Specialities(id)
);

-- - store prices for each appointment type as set by a doctor
CREATE TABLE IF NOT EXISTS Appointment_Prices (
    doctor_id INT NOT NULL,
    appointment_type INT NOT NULL,
    price INT NOT NULL,
    FOREIGN KEY (doctor_id) REFERENCES Doctors(id),
    FOREIGN KEY (appointment_type) REFERENCES Appointment_Types(id),
    PRIMARY KEY (doctor_id, appointment_type)
);

-- - info about patients
CREATE TABLE IF NOT EXISTS Patients (
    id INT PRIMARY KEY ,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(255) NOT NULL
);

-- - help doctors keep track of their appointments with patients
CREATE TABLE IF NOT EXISTS Appointments (
    id INT PRIMARY KEY ,
    doctor_id INT NOT NULL,
    patient_id INT NOT NULL,
    appointment_type INT NOT NULL,
    date_time TIMESTAMP NOT NULL,
    type VARCHAR(255) NOT NULL,
    status VARCHAR(255) NOT NULL,
    prescription TEXT,
    FOREIGN KEY (doctor_id) REFERENCES Doctors(id),
    FOREIGN KEY (patient_id) REFERENCES Patients(id),
    FOREIGN KEY (appointment_type) REFERENCES Appointment_Types(id),
    CONSTRAINT chk_type CHECK (type IN ('physical', 'virtual')),
    CONSTRAINT chk_status CHECK (status IN ('scheduled', 'fulfilled', 'cancelled'))
);

-- - store old appointments in here with same schema as regular Appointments table
CREATE TABLE IF NOT EXISTS Patients_Previous_Appointments () INHERITS (Appointments);

-- - keep track of notifications to deliver
CREATE TABLE IF NOT EXISTS Notifications (
    id INT PRIMARY KEY ,
    patient_id INT NOT NULL,
    message TEXT NOT NULL,
    date_time TIMESTAMP NOT NULL,
    FOREIGN KEY (patient_id) REFERENCES Patients(id)
);
