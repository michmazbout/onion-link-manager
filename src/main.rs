use eframe::egui;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;

#[derive(Serialize, Deserialize, Clone)]
struct OnionLink {
    title: String,
    url: String,
}

struct OnionLinkManager {
    links: HashMap<String, OnionLink>,
    new_title: String,
    new_url: String,
    edit_title: Option<String>,
    edit_url: Option<String>,
    file_path: PathBuf,
    tor_browser_path: Option<PathBuf>,
    copy_feedback: Option<(String, String, f32)>, // (title, message, time left)

}

impl Default for OnionLinkManager {
    fn default() -> Self {
        let mut app = Self {
            links: HashMap::new(),
            new_title: String::new(),
            new_url: String::new(),
            edit_title: None,
            edit_url: None,
            file_path: PathBuf::from("onion_links.json"),
            tor_browser_path: None,
            copy_feedback: None,
        };
        app.load_links();
        app.search_tor_browser();
        app
    }
}

impl OnionLinkManager {
    fn load_links(&mut self) {
        if let Ok(mut file) = File::open(&self.file_path) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            if let Ok(links) = serde_json::from_str::<HashMap<String, OnionLink>>(&contents) {
                self.links = links;
            }
        }
    }

    fn save_links(&self) {
        let contents = serde_json::to_string(&self.links).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)
            .unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }

    fn search_tor_browser(&mut self) {
        let possible_paths = [
            PathBuf::from("/home/elias/.local/share/torbrowser/tbb/x86_64/tor-browser/Browser/start-tor-browser"),
            PathBuf::from("/usr/local/bin/torbrowser"),
            PathBuf::from("/opt/tor-browser/Browser/start-tor-browser"),
            PathBuf::from("/home/elias/Downloads/tor-browser/Browser/start-tor-browser"),
        ];

        for path in possible_paths.iter() {
            if path.exists() {
                self.tor_browser_path = Some(path.clone());
                break;
            }
        }
    }

    fn open_in_tor_browser(&self, url: &str) {
        if let Some(path) = &self.tor_browser_path {
            Command::new(path)
                .arg(url)
                .spawn()
                .expect("Failed to open Tor Browser");
        } else {
            eprintln!("Tor Browser not found.");
        }
    }
}

impl eframe::App for OnionLinkManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Onion Link Manager");

            // Modern UI styling
            ui.style_mut().spacing.item_spacing = egui::vec2(10.0, 10.0);
            ui.style_mut().visuals.widgets.noninteractive.rounding = 5.0.into();
            ui.style_mut().visuals.widgets.inactive.rounding = 5.0.into();
            ui.style_mut().visuals.widgets.hovered.rounding = 5.0.into();
            ui.style_mut().visuals.widgets.active.rounding = 5.0.into();
            ui.style_mut().visuals.widgets.open.rounding = 5.0.into();


            // Add a new link
            ui.horizontal(|ui| {
                ui.label("Title:");
                ui.text_edit_singleline(&mut self.new_title);
                ui.label("URL:");
                ui.text_edit_singleline(&mut self.new_url);
                if ui.button("Add").clicked() {
                    if !self.new_title.is_empty() && !self.new_url.is_empty() {
                        self.links.insert(
                            self.new_title.clone(),
                            OnionLink {
                                title: self.new_title.clone(),
                                url: self.new_url.clone(),
                            },
                        );
                        self.new_title.clear();
                        self.new_url.clear();
                        self.save_links();
                    }
                }

                
            });

            // Display existing links
            ui.separator();
            ui.label("Saved Links:");

            // Collect keys to avoid borrowing issues
            let keys: Vec<String> = self.links.keys().cloned().collect();
            for title in keys {
                if let Some(link) = self.links.get(&title) {
                    let link_clone = link.clone(); // Clone the link to avoid borrowing issues
                    ui.horizontal(|ui| {
                        if let Some(edit_title) = &self.edit_title {
                            if edit_title == &title {
                                let mut edit_title_clone = edit_title.clone();
                                let mut edit_url_clone = self.edit_url.clone().unwrap_or_default();
                                ui.text_edit_singleline(&mut edit_title_clone);
                                ui.text_edit_singleline(&mut edit_url_clone);
                                if ui.button("Save").clicked() {
                                    let mut updated_link = link_clone.clone();
                                    updated_link.title = edit_title_clone.clone();
                                    updated_link.url = edit_url_clone.clone();
                                    self.links.insert(edit_title_clone.clone(), updated_link);
                                    if edit_title_clone != title {
                                        self.links.remove(&title);
                                    }
                                    self.edit_title = None;
                                    self.edit_url = None;
                                    self.save_links();
                                }
                            } else {
                                ui.label(&link_clone.title);
                                if ui.link(&link_clone.url).clicked() {
                                    self.open_in_tor_browser(&link_clone.url);
                                }
                                if ui.button("Copy").clicked() {
                                    ctx.output_mut(|o| o.copied_text = link_clone.url.clone());
                                    self.copy_feedback = Some((link_clone.title.clone(), "Copied!".to_string(), 2.0));
                                }
                                
                                // Show 'Copied!' if this link was copied
                                if let Some((copied_title, msg, time_left)) = &mut self.copy_feedback {
                                    if copied_title == &link_clone.title {
                                        ui.colored_label(egui::Color32::LIGHT_GREEN, msg);
                                        *time_left -= ctx.input(|i| i.stable_dt);
                                        if *time_left <= 0.0 {
                                            self.copy_feedback = None;
                                        } else {
                                            ctx.request_repaint(); // keep repainting so the timer runs
                                        }
                                    }
                                }
                                                                          
                                if ui.button("Edit").clicked() {
                                    self.edit_title = Some(link_clone.title.clone());
                                    self.edit_url = Some(link_clone.url.clone());
                                }
                                if ui.button("Delete").clicked() {
                                    self.links.remove(&title);
                                    self.save_links();
                                }
                            }
                        } else {
                            ui.label(&link_clone.title);
                            if ui.link(&link_clone.url).clicked() {
                                self.open_in_tor_browser(&link_clone.url);
                            }
                            if ui.button("Edit").clicked() {
                                self.edit_title = Some(link_clone.title.clone());
                                self.edit_url = Some(link_clone.url.clone());
                            }
                            if ui.button("Copy").clicked() {
                                ctx.output_mut(|o| o.copied_text = link_clone.url.clone());
                                self.copy_feedback = Some((link_clone.title.clone(), "Copied!".to_string(), 2.0));
                            }
                            
                            // Show 'Copied!' if this link was copied
                            if let Some((copied_title, msg, time_left)) = &mut self.copy_feedback {
                                if copied_title == &link_clone.title {
                                    ui.colored_label(egui::Color32::LIGHT_GREEN, msg);
                                    *time_left -= ctx.input(|i| i.stable_dt);
                                    if *time_left <= 0.0 {
                                        self.copy_feedback = None;
                                    } else {
                                        ctx.request_repaint_after(std::time::Duration::from_secs_f32(*time_left));
                                    }
                                }
                            }
                                                      
                            if ui.button("Delete").clicked() {
                                self.links.remove(&title);
                                self.save_links();
                            }
                        }
                    });
                }
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Onion Link Manager",
        options,
        Box::new(|_cc| Box::new(OnionLinkManager::default())),
    );
}
