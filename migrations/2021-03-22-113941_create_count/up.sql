CREATE TABLE counts (
    id SERIAL PRIMARY KEY,
    device_name TEXT NOT NULL,
    num_of_people INTEGER NOT NULL,
    recorded_at TIMESTAMP NOT NULL
)
