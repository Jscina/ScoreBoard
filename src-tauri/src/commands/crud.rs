use std::sync::Mutex;

use tauri::State;

use crate::AppState;

#[tauri::command(async)]
pub async fn insert_student(state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    todo!()
}
