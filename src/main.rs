use eframe::egui;

mod task;
mod storage;

use task::{Task, Status};
use storage::{load_tasks, save_tasks};

struct TaskApp {
    tasks: Vec<Task>,
    input: String,
    filter: Option<Status>,
    editing_index: Option<usize>,
    edit_buffer: String,
}

impl Default for TaskApp {
    fn default() -> Self {
        Self {
            tasks: load_tasks(),
            input: String::new(),
            filter: None,
            editing_index: None,
            edit_buffer: String::new(),
        }
    }
}

impl eframe::App for TaskApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("🦀 Rust Task Manager");

            // ================= ADD TASK =================
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.input);

                if ui.button("Add").clicked()
                    && !self.input.trim().is_empty()
                {
                    self.tasks.push(Task {
                        title: self.input.clone(),
                        status: Status::ToDo,
                    });

                    self.input.clear();
                }
            });

            ui.separator();

            // ================= FILTER =================
            ui.horizontal(|ui| {
                if ui.button("All").clicked() {
                    self.filter = None;
                }
                if ui.button("Todo").clicked() {
                    self.filter = Some(Status::ToDo);
                }
                if ui.button("InProgress").clicked() {
                    self.filter = Some(Status::InProgress);
                }
                if ui.button("Done").clicked() {
                    self.filter = Some(Status::Done);
                }
            });

            ui.separator();

            // ================= TASK LIST =================
            egui::ScrollArea::vertical().show(ui, |ui| {

                let mut delete_index: Option<usize> = None;

                for i in 0..self.tasks.len() {
                    let task = &mut self.tasks[i];

                    // Apply filter
                    if let Some(ref filter_status) = self.filter {
                        if &task.status != filter_status {
                            continue;
                        }
                    }

                    ui.group(|ui| {

                        // ---------- EDIT MODE ----------
                        if Some(i) == self.editing_index {

                            ui.text_edit_singleline(&mut self.edit_buffer);

                            ui.horizontal(|ui| {
                                if ui.button("Save").clicked() {
                                    task.title = self.edit_buffer.clone();
                                    self.editing_index = None;
                                }

                                if ui.button("Cancel").clicked() {
                                    self.editing_index = None;
                                }
                            });

                        } else {

                            // ---------- NORMAL VIEW ----------
                            ui.label(format!(
                                "{} ({:?})",
                                task.title,
                                task.status,
                            ));

                            ui.horizontal(|ui| {

                                if ui.button("Edit").clicked() {
                                    self.editing_index = Some(i);
                                    self.edit_buffer = task.title.clone();
                                }

                                if ui.button("Todo").clicked() {
                                    task.status = Status::ToDo;
                                }

                                if ui.button("InProgress").clicked() {
                                    task.status = Status::InProgress;
                                }

                                if ui.button("Done").clicked() {
                                    task.status = Status::Done;
                                }

                                if ui.button("Delete").clicked() {
                                    delete_index = Some(i);
                                }
                            });
                        }
                    });

                    ui.add_space(5.0);
                }

                // DELETE AFTER LOOP
                if let Some(i) = delete_index {
                    self.tasks.remove(i);
                }
            });

            // ================= SAVE =================
            save_tasks(&self.tasks);
        });
    }
}

fn main() -> eframe::Result<()> {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Task Manager",
        options,
        Box::new(|_cc| Box::new(TaskApp::default())),
    )
}
