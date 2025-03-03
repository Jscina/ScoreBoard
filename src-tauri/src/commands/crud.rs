use tokio::sync::Mutex;
use tauri::State;
use crate::{AppState, database::models::*};
use chrono::NaiveDate;

#[tauri::command]
pub async fn insert_student(
    state: State<'_, Mutex<AppState>>, 
    first_name: String, 
    last_name: String, 
    student_id: String
) -> Result<(), String> {
    let state = state.lock().await;
    sqlx::query!(
        "INSERT INTO students (first_name, last_name, student_id) VALUES (?, ?, ?)",
        first_name,
        last_name,
        student_id
    )
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_students(
    state: State<'_, Mutex<AppState>>
) -> Result<Vec<Student>, String> {
    let state = state.lock().await;
    let students = sqlx::query_as!(
        Student,
        "SELECT id, first_name, last_name, student_id FROM students"
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(students)
}

#[tauri::command]
pub async fn update_student(
    state: State<'_, Mutex<AppState>>,
    id: i64,
    first_name: String,
    last_name: String,
    student_id: String,
) -> Result<(), String> {
    let state = state.lock().await;
    sqlx::query!(
        "UPDATE students SET first_name = ?, last_name = ?, student_id = ? WHERE id = ?",
        first_name,
        last_name,
        student_id,
        id
    )
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_student(
    state: State<'_, Mutex<AppState>>,
    id: i64,
) -> Result<(), String> {
    let state = state.lock().await;
    sqlx::query!("DELETE FROM students WHERE id = ?", id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn insert_class(
    state: State<'_, Mutex<AppState>>,
    class_name: String,
) -> Result<(), String> {
    let state = state.lock().await;
    sqlx::query!(
        "INSERT INTO classes (class_name) VALUES (?)",
        class_name
    )
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_classes(
    state: State<'_, Mutex<AppState>>
) -> Result<Vec<Class>, String> {
    let state = state.lock().await;
    let classes = sqlx::query_as!(
        Class,
        "SELECT id, class_name FROM classes"
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(classes)
}

#[tauri::command]
pub async fn update_class(
    state: State<'_, Mutex<AppState>>,
    id: i64,
    class_name: String,
) -> Result<(), String> {
    let state = state.lock().await;
    sqlx::query!(
        "UPDATE classes SET class_name = ? WHERE id = ?",
        class_name,
        id
    )
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_class(
    state: State<'_, Mutex<AppState>>,
    id: i64,
) -> Result<(), String> {
    let state = state.lock().await;
    sqlx::query!("DELETE FROM classes WHERE id = ?", id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn insert_assignment(
    state: State<'_, Mutex<AppState>>,
    class_id: i64,
    title: String,
    due_date: Option<String>,
    category: String,
    max_score: i64,
) -> Result<(), String> {
    let state = state.lock().await;
    let parsed_date = match due_date {
        Some(date_str) => Some(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|e| e.to_string())?),
        None => None,
    };
    sqlx::query!(
        "INSERT INTO assignments (class_id, title, due_date, category, max_score) VALUES (?, ?, ?, ?, ?)",
        class_id,
        title,
        parsed_date,
        category,
        max_score
    )
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_assignments(
    state: State<'_, Mutex<AppState>>
) -> Result<Vec<Assignment>, String> {
    let state = state.lock().await;
    let assignments = sqlx::query_as!(
        Assignment,
        "SELECT id, class_id, title, due_date, category, max_score FROM assignments"
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(assignments)
}

#[tauri::command]
pub async fn update_assignment(
    state: State<'_, Mutex<AppState>>,
    id: i64,
    class_id: i64,
    title: String,
    due_date: Option<String>, // Expected format: "YYYY-MM-DD"
    category: String,
    max_score: i64,
) -> Result<(), String> {
    let state = state.lock().await;
    let parsed_date = match due_date {
        Some(date_str) => Some(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|e| e.to_string())?),
        None => None,
    };
    sqlx::query!(
        "UPDATE assignments SET class_id = ?, title = ?, due_date = ?, category = ?, max_score = ? WHERE id = ?",
        class_id,
        title,
        parsed_date,
        category,
        max_score,
        id
    )
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_assignment(
    state: State<'_, Mutex<AppState>>,
    id: i64,
) -> Result<(), String> {
    let state = state.lock().await;
    sqlx::query!("DELETE FROM assignments WHERE id = ?", id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn insert_grade(
    state: State<'_, Mutex<AppState>>,
    student_id: i64,
    assignment_id: i64,
    score: i64,
) -> Result<(), String> {
    let state = state.lock().await;
    sqlx::query!(
        "INSERT INTO grades (student_id, assignment_id, score) VALUES (?, ?, ?)",
        student_id,
        assignment_id,
        score
    )
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_grades(
    state: State<'_, Mutex<AppState>>
) -> Result<Vec<Grade>, String> {
    let state = state.lock().await;
    let grades = sqlx::query_as!(
        Grade,
        "SELECT id, student_id, assignment_id, score FROM grades"
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(grades)
}

#[tauri::command]
pub async fn update_grade(
    state: State<'_, Mutex<AppState>>,
    id: i64,
    student_id: i64,
    assignment_id: i64,
    score: i64,
) -> Result<(), String> {
    let state = state.lock().await;
    sqlx::query!(
        "UPDATE grades SET student_id = ?, assignment_id = ?, score = ? WHERE id = ?",
        student_id,
        assignment_id,
        score,
        id
    )
    .execute(&state.db.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_grade(
    state: State<'_, Mutex<AppState>>,
    id: i64,
) -> Result<(), String> {
    let state = state.lock().await;
    sqlx::query!("DELETE FROM grades WHERE id = ?", id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}


