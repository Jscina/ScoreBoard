CREATE TABLE grades (
    id INTEGER PRIMARY KEY,
    student_id INTEGER NOT NULL,
    assignment_id INTEGER NOT NULL,
    score INTEGER NOT NULL CHECK (score >= 0 AND score <= (
        SELECT max_score
        FROM assignments
        WHERE id = assignment_id
    )),
    FOREIGN KEY (student_id) REFERENCES students (id) ON DELETE CASCADE,
    FOREIGN KEY (assignment_id) REFERENCES assignments (id) ON DELETE CASCADE
);
