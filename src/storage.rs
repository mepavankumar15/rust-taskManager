use std::fs;
use crate::task::Task;

pub fn save_tasks(tasks: &Vec<Task>) {
    if let Ok(json) = serde_json::to_string(tasks) {
        let _ = fs::write("tasks.json", json);
    }
}

pub fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}
