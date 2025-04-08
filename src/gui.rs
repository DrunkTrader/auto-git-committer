use eframe::egui;
use crate::logic::{update, pull, push};

pub struct MyApp {
    repo: String,
    url: String,
    commits: i32,
    status: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            repo: "".into(),
            url: "".into(),
            commits: 5,
            status: "".into(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
            ui.heading("Auto-Git-Committer");
            
            ui.add_space(10.0);

            ui.label("Repository Name:");
            ui.text_edit_singleline(&mut self.repo);

            ui.label("Repository URL:");
            ui.text_edit_singleline(&mut self.url);

            // Add horizontal centering for the slider
            ui.horizontal(|ui| {
                ui.add_space(ui.available_width() * 0.25); // Add 25% space on the left
                ui.add(egui::Slider::new(&mut self.commits, 1..=100).text("Commits")); // 50% width slider
            });

            ui.add_space(10.0);
            
            if ui.button("Run").clicked() {
                let mut count = 1;
                match pull(&self.repo, &self.url) {
                Ok(_) => {
                    for _ in 0..self.commits {
                    if update().is_err() || push(&mut count).is_err() {
                        self.status = "Error during update/push".into();
                        return;
                    }
                    }
                    self.status = format!("Done: {} commits pushed!", self.commits);
                }
                Err(_) => self.status = "Failed to pull repo".into(),
                }
            }

            ui.separator();
            ui.label(&self.status);
            });
            
            // Add developer name at the bottom
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.horizontal(|ui| {
                ui.label("Developed by ");
                ui.hyperlink_to("@drunktrader", "https://github.com/drunktrader");
            });
            ui.add_space(4.0);
            });
        });
    }
}

pub fn launch_gui() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default();
    options.viewport.inner_size = Some(egui::Vec2::new(400.0, 500.0)); // Width, height in pixels
    eframe::run_native("Auto-Git-Committer GUI", options, Box::new(|_cc| Box::new(MyApp::default())))
}
