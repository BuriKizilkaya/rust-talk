// ============================================
// DEMO 7: Desktop App — Tauri
// ============================================
// Zeigt: Native Desktop App, JS ↔ Rust Bridge, Systembefehle
// Echte Projekte: Zed, Gitbutler, Spacedrive, 1Password (teilweise)
//
// Bundle-Grösse:  Electron ~150MB  |  Tauri ~8MB
// RAM-Verbrauch:  Electron ~200MB  |  Tauri ~30MB
//
// Setup:
//   cargo install create-tauri-app
//   cargo create-tauri-app my_app
//   cargo tauri dev

use tauri::{command, State, Manager};
use std::sync::Mutex;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Note {
    id:      u32,
    title:   String,
    content: String,
}

struct AppState {
    notes: Mutex<Vec<Note>>,
}

// Rust-Funktionen direkt aus dem JavaScript-Frontend aufrufbar
#[command]
fn get_notes(state: State<AppState>) -> Vec<Note> {
    state.notes.lock().unwrap().clone()
}

#[command]
fn add_note(title: String, content: String, state: State<AppState>) -> Note {
    let mut notes = state.notes.lock().unwrap();
    let note = Note { id: notes.len() as u32 + 1, title, content };
    notes.push(note.clone());
    note
}

#[command]
fn delete_note(id: u32, state: State<AppState>) -> bool {
    let mut notes = state.notes.lock().unwrap();
    let before = notes.len();
    notes.retain(|n| n.id != id);
    notes.len() < before
}

#[command]
fn system_info() -> serde_json::Value {
    serde_json::json!({
        "os":        std::env::consts::OS,
        "arch":      std::env::consts::ARCH,
    })
}

fn main() {
    tauri::Builder::default()
        .manage(AppState { notes: Mutex::new(vec![]) })
        .invoke_handler(tauri::generate_handler![
            get_notes, add_note, delete_note, system_info
        ])
        .run(tauri::generate_context!())
        .unwrap();
}

// -----------------------------------------------
// JavaScript Frontend (React / Svelte / Vanilla):
//
// import { invoke } from '@tauri-apps/api/tauri';
//
// const notes = await invoke('get_notes');
// const note  = await invoke('add_note', { title: 'Hey', content: '🦀' });
// const info  = await invoke('system_info');
// -----------------------------------------------
