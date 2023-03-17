-- Your SQL goes here
CREATE TABLE pishock_devices (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    sharecode VARCHAR(255) NOT NULL,
    max_intensity INT NOT NULL,
    max_duration INT NOT NULL
);