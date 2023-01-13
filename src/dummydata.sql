INSERT INTO Specialities (id, name, description) VALUES (1, 'Cardiology', 'Speciality dealing with diseases of the heart and blood vessels');
INSERT INTO Specialities (id, name, description) VALUES (2, 'Dermatology', 'Speciality dealing with skin diseases');
INSERT INTO Specialities (id, name, description) VALUES (3, 'Neurology', 'Speciality dealing with diseases of the nervous system');
INSERT INTO Specialities (id, name, description) VALUES (4, 'Oncology', 'Speciality dealing with cancer');
INSERT INTO Specialities (id, name, description) VALUES (5, 'Orthopedics', 'Speciality dealing with diseases and injuries of the musculoskeletal system');

INSERT INTO Doctors (id, name, speciality_id, city, address) VALUES (1, 'Dr. John Smith', 1, 'New York', '123 Main St');
INSERT INTO Doctors (id, name, speciality_id, city, address) VALUES (2, 'Dr. Jane Doe', 2, 'Los Angeles', '456 Park Ave');
INSERT INTO Doctors (id, name, speciality_id, city, address) VALUES (3, 'Dr. Michael Johnson', 3, 'Chicago', '789 Elm St');
INSERT INTO Doctors (id, name, speciality_id, city, address) VALUES (4, 'Dr. Sarah Lee', 4, 'Houston', '321 Oak St');
INSERT INTO Doctors (id, name, speciality_id, city, address) VALUES (5, 'Dr. David Brown', 5, 'Philadelphia', '654 Pine St');

INSERT INTO Appointment_Types (id, name, speciality_id, description) VALUES (1, 'Consultation', 1, 'General consultation with a cardiologist');
INSERT INTO Appointment_Types (id, name, speciality_id, description) VALUES (2, 'Skin Check', 2, 'Checkup for skin diseases');
INSERT INTO Appointment_Types (id, name, speciality_id, description) VALUES (3, 'Neurology Checkup', 3, 'Checkup for neurological diseases');
INSERT INTO Appointment_Types (id, name, speciality_id, description) VALUES (4, 'Oncology Checkup', 4, 'Checkup for cancer patients');
INSERT INTO Appointment_Types (id, name, speciality_id, description) VALUES (5, 'Orthopedics Checkup', 5, 'Checkup for musculoskeletal diseases and injuries');

INSERT INTO Appointment_Prices (doctor_id, appointment_type, price) VALUES (1, 1, 200);
INSERT INTO Appointment_Prices (doctor_id, appointment_type, price) VALUES (2, 2, 250);
INSERT INTO Appointment_Prices (doctor_id, appointment_type, price) VALUES (3, 3, 300);
INSERT INTO Appointment_Prices (doctor_id, appointment_type, price) VALUES (4, 4, 350);
INSERT INTO Appointment_Prices (doctor_id, appointment_type, price) VALUES (5, 5, 400);

INSERT INTO Patients (id, name, email, phone) VALUES (1, 'Alice Smith', 'alice.smith@email.com', '555-555-5555');
INSERT INTO Patients (id, name, email, phone) VALUES (2, 'Bob Johnson', 'bob.johnson@email.com', '555-555-5556');
INSERT INTO Patients (id, name, email, phone) VALUES (3, 'Charlie Brown', 'charlie.brown@email.com', '555-555-5557');
INSERT INTO Patients (id, name, email, phone) VALUES (4, 'David Lee', 'david.lee@email.com', '555-555-5558');
INSERT INTO Patients (id, name, email, phone) VALUES (5, 'Emily Davis', 'emily.davis@email.com', '555-555-5559');

INSERT INTO Appointments (id, doctor_id, patient_id, appointment_type, date_time, type, status, prescription) VALUES (1, 1, 1, 1, '2022-01-01 10:00:00', 'physical', 'scheduled', 'Prescription 1');
INSERT INTO Appointments (id, doctor_id, patient_id, appointment_type, date_time, type, status, prescription) VALUES (2, 2, 2, 2, '2022-01-02 15:00:00', 'virtual', 'scheduled', 'Prescription 2');
INSERT INTO Appointments (id, doctor_id, patient_id, appointment_type, date_time, type, status, prescription) VALUES (3, 3, 3, 3, '2022-01-03 09:00:00', 'physical', 'scheduled', 'Prescription 3');
INSERT INTO Appointments (id, doctor_id, patient_id, appointment_type, date_time, type, status, prescription) VALUES (4, 4, 4, 4, '2022-01-04 14:00:00', 'virtual', 'scheduled', 'Prescription 4');
INSERT INTO Appointments (id, doctor_id, patient_id, appointment_type, date_time, type, status, prescription) VALUES (5, 5, 5, 5, '2022-01-05 11:00:00', 'physical', 'scheduled', 'Prescription 5');

INSERT INTO Patients_Previous_Appointments (id, doctor_id, patient_id, appointment_type, date_time, type, status, prescription) VALUES (8, 3, 3, 3, '2021-12-29 09:00:00', 'physical', 'cancelled', 'Prescription 8');
INSERT INTO Patients_Previous_Appointments (id, doctor_id, patient_id, appointment_type, date_time, type, status, prescription) VALUES (9, 4, 4, 4, '2021-12-28 14:00:00', 'virtual', 'fulfilled', 'Prescription 9');
INSERT INTO Patients_Previous_Appointments (id, doctor_id, patient_id, appointment_type, date_time, type, status, prescription) VALUES (10, 5, 5, 5, '2021-12-27 11:00:00', 'physical', 'cancelled', 'Prescription 10');

INSERT INTO Notifications (id, patient_id, message, date_time) VALUES (4, 4, 'Reminder: Virtual appointment with Dr. Sarah Lee on 2022-01-04 14:00:00', '2021-12-31 23:59:59');
INSERT INTO Notifications (id, patient_id, message, date_time) VALUES (5, 5, 'Reminder: Appointment with Dr. David Brown at 654 Pine St on 2022-01-05 11:00:00', '2021-12-31 23:59:59');
