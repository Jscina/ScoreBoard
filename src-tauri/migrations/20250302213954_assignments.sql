CREATE TABLE assignments (
    id INTEGER PRIMARY KEY,
    class_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    due_date DATE,
    category TEXT CHECK (category IN ('Homework', 'Test')) NOT NULL,
    max_score INTEGER NOT NULL,
    FOREIGN KEY (class_id) REFERENCES classes(id) ON DELETE CASCADE
);
