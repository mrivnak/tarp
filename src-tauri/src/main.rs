// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::project::Project;
use std::path::PathBuf;
use std::sync::Mutex;
use tarpaulin::SimpleReport;
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::State;

mod project;
mod tarpaulin;

struct AppState {
    project: Mutex<Option<Project>>,
}

#[tauri::command(async)]
fn load_project(state: State<AppState>) -> Option<PathBuf> {
    match FileDialogBuilder::new()
        .set_title("Open Project")
        .pick_folder()
    {
        Some(path) => {
            let project = Project { path: path.clone() };
            state.project.lock().unwrap().replace(project);
            Some(path)
        }
        None => None,
    }
}

#[tauri::command(async)]
fn run_tarpaulin(state: State<AppState>) -> Option<SimpleReport> {
    let lock = state.project.lock().unwrap();
    let project = lock.as_ref().unwrap();
    println!("Running tarpaulin on {:?}", project.path);
    let result = tarpaulin::run_tarpaulin(&project.path).unwrap();
    let mut simple = SimpleReport::from(result);
    simple.files.iter_mut().for_each(|file| {
        file.path = file.path.strip_prefix(&project.path).unwrap().to_path_buf();
    });
    Some(simple)
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            project: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![load_project, run_tarpaulin])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
