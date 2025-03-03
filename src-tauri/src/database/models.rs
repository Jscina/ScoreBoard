use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::NaiveDate;

/// Represents a Student
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Student {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub student_id: String,
}

/// Represents a Class
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Class {
    pub id: i64,
    pub class_name: String,
}

/// Represents an Assignment
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Assignment {
    pub id: i64,
    pub class_id: i64,
    pub title: String,
    pub due_date: Option<NaiveDate>,
    pub category: String, 
    pub max_score: i64,
}

/// Represents a Student's Grade for an Assignment
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Grade {
    pub id: i64,
    pub student_id: i64,
    pub assignment_id: i64,
    pub score: i64,
}

