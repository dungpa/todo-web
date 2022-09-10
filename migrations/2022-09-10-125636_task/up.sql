CREATE TABLE task (
    id INTEGER NOT NULL,
    title TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    completed BOOLEAN NOT NULL,
    PRIMARY KEY (id)
);