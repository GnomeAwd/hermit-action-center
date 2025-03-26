use eframe::egui;
use egui::{Button, Color32, Label, Layout, RichText, Vec2, Widget};
use egui_phosphor::regular::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::process::Command;
use std::time::{Duration, Instant};

mod active_actions;
mod quick_settings;
mod slider_controls;

use active_actions::ActiveActions;
use quick_settings::QuickSettings;
use slider_controls::IconSlider;

#[derive(Clone)]
struct Colors {
    background: Color32,
    surface: Color32,
    on_surface: Color32,
    primary: Color32,
    on_primary: Color32,
    secondary: Color32,
    on_secondary: Color32,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            background: Color32::from_rgb(30, 30, 30),      // Dark gray
            surface: Color32::from_rgb(30, 30, 30),         // Dark gray
            on_surface: Color32::from_rgb(200, 200, 200),   // Light gray
            primary: Color32::from_rgb(0, 120, 215),        // Blue
            on_primary: Color32::from_rgb(255, 255, 255),   // White
            secondary: Color32::from_rgb(0, 153, 204),      // Light blue
            on_secondary: Color32::from_rgb(255, 255, 255), // White
        }
    }
}

struct ActionCenterWidget {
    colors: Colors,
    positioned: bool,
    brightness_slider: IconSlider,
    volume_slider: IconSlider,
    quick_settings: QuickSettings,
    active_actions: ActiveActions,
    last_wifi_update: std::time::Instant,
    last_bluetooth_update: std::time::Instant,
}

#[derive(Serialize, Deserialize, Debug)]
struct Workspace {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Client {
    address: String,
    mapped: bool,
    hidden: bool,
    at: Vec<i32>,
    size: Vec<i32>,
    workspace: Workspace,
    floating: bool,
    pseudo: bool,
    monitor: i32,
    class: String,
    title: String,
    initialClass: String,
    initialTitle: String,
    pid: i32,
    xwayland: bool,
    pinned: bool,
    fullscreen: i32,
    fullscreenClient: i32,
    grouped: Vec<String>,
    tags: Vec<String>,
    swallowing: String,
    focusHistoryID: i32,
    inhibitingIdle: bool,
}

impl ActionCenterWidget {
    fn place_widgets(&mut self) {
        if self.positioned {
            return;
        }

        // get hyprctl clients
        let output = match Command::new("hyprctl").arg("clients").arg("-j").output() {
            Ok(output) => output,
            Err(e) => {
                eprintln!("Failed to execute command: {}", e);
                return; // Exit the function if the command fails
            }
        };

        let clients: Vec<Client> = match serde_json::from_slice(&output.stdout) {
            Ok(clients) => clients,
            Err(e) => {
                eprintln!("Failed to parse clients: {}", e);
                return; // Exit the function if parsing fails
            }
        };

        if clients.len() > 0 {
            // get client with title "Action Center "
            let action_center_client = clients
                .iter()
                .find(|client| client.title == "Action Center");
            if let Some(action_center_client) = action_center_client {
                // println!("Action Center client: {:?}", action_center_client);
                let address = action_center_client.address.clone();
                let monitor_width = 1920;
                let monitor_height = 1080;
                let x = monitor_width - 370 - 10;
                let y = 60;
                let width = 370;
                let height = monitor_height - 70;

                let move_cmd = format!(
                    "hyprctl dispatch movewindowpixel \"exact {} {},address:{}\"",
                    x, y, address
                );
                Command::new("sh").args(&["-c", &move_cmd]).output().ok();

                // Resize window
                let resize_cmd = format!(
                    "hyprctl dispatch resizewindowpixel \"exact {} {},address:{}\"",
                    width, height, address
                );
                Command::new("sh").args(&["-c", &resize_cmd]).output().ok();

                // Pin window
                let address_arg = format!("address:{}", address);
                Command::new("hyprctl")
                    .args(&["dispatch", "pin", &address_arg])
                    .output()
                    .ok();

                self.positioned = true;

                self.positioned = true;
                // println!("Address: {}", address);
            }
        }
    }
    fn get_colors(&mut self) {
        let file_path = "./src/colors.css";
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening colors.css: {}", e);
                return; // Return early, using default colors if file cannot be opened
            }
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            eprintln!("Error reading colors.css: {}", e);
            return;
        }

        let mut colors = Colors::default(); // Use default colors

        for line in contents.lines() {
            // Check for color definitions in the format "@define-color key #value;"
            if line.contains("@define-color") {
                let line = line.replace("@define-color ", "");
                let parts: Vec<&str> = line.split(' ').collect();
                if parts.len() >= 2 {
                    let key = parts[0].trim();

                    // Extract the color value and clean it
                    let mut value = parts[1].trim().replace(";", "");

                    // Ensure the value starts with #
                    if !value.starts_with('#') {
                        continue;
                    }

                    // Remove the # prefix for hex parsing
                    value = value.trim_start_matches('#').to_string();

                    // Ensure 6 characters for RGB (add alpha if needed)
                    if value.len() == 6 {
                        value = format!("#{}ff", value); // Add alpha channel
                    } else if value.len() != 8 {
                        println!("Invalid color format for {}: {}", key, value);
                        continue;
                    }

                    // Try to parse the hex value
                    let color = match Color32::from_hex(&value) {
                        Ok(color) => color,
                        Err(e) => {
                            println!("Failed to parse color for {}: {:?}", key, e);
                            continue;
                        }
                    };

                    // Map the color based on the key
                    match key {
                        "surface_container_lowest" => colors.background = color,
                        "surface_container_low" => colors.surface = color,
                        "on_surface_variant" => colors.on_surface = color,
                        "primary_fixed_dim" => colors.primary = color,
                        "on_primary_fixed" => colors.on_primary = color,
                        "secondary_fixed_dim" => colors.secondary = color,
                        "on_secondary_fixed" => colors.on_secondary = color,
                        _ => {} // Ignore other keys
                    }
                }
            }
        }

        self.colors = colors.clone();
        self.quick_settings.update_colors(colors.clone());
        self.active_actions.update_colors(colors);
    }
}

impl eframe::App for ActionCenterWidget {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = std::time::Instant::now();

        // Update WiFi status every 5 seconds
        if now.duration_since(self.last_wifi_update).as_secs() >= 5 {
            self.quick_settings.update_wifi_state();
            self.last_wifi_update = now;
        }

        // Update Bluetooth status every 5 seconds
        if now.duration_since(self.last_bluetooth_update).as_secs() >= 5 {
            self.quick_settings.update_bluetooth_state();
            self.last_bluetooth_update = now;
        }

        // Set the global visual style with our custom colors
        {
            let mut style = (*ctx.style()).clone();
            style.visuals.window_fill = self.colors.background;
            style.visuals.panel_fill = self.colors.background;
            ctx.set_style(style);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .max_height(ui.available_height() - 8.0)
                .show(ui, |ui| {
                    ui.add_space(4.0);
                    ui.vertical(|ui| {
                        // Top section with equal-width widgets
                        ui.horizontal(|ui| {
                            ui.add_space(4.0);
                            ui.set_min_width(ui.available_width());
                            let available_width = ui.available_width() / 2.0 - 8.0; // Half spacing between columns

                            // Left column - Quick Settings
                            ui.scope(|ui| {
                                ui.set_min_width(available_width);
                                ui.set_max_width(available_width);
                                self.quick_settings.show(ui);
                            });

                            ui.add_space(4.0);

                            // Right column - Active Actions
                            ui.scope(|ui| {
                                ui.set_min_width(available_width);
                                ui.set_max_width(available_width);
                                self.active_actions.show(ui);
                            });
                        });
                        //ADD seperator
                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);

                        // Display section with brightness slider
                        self.brightness_slider.show(ui, &self.colors);
                        ui.add_space(8.0);

                        // Sound section with volume slider
                        self.volume_slider.show(ui, &self.colors);
                        ui.add_space(8.0);

                        ui.separator();
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.add_space(8.0);
                            ui.label(RichText::new("Notifications").color(self.colors.on_surface));
                            ui.add_space(8.0);
                        });
                    });
                });
        });
        self.place_widgets();
    }
}

impl Default for ActionCenterWidget {
    fn default() -> Self {
        let colors = Colors::default();
        let mut widget = Self {
            colors: colors.clone(),
            positioned: false,
            brightness_slider: IconSlider::new(50.0, SUN.to_string()).with_title("Display"),
            volume_slider: IconSlider::new(50.0, SPEAKER_HIGH.to_string()).with_title("Sound"),
            quick_settings: QuickSettings::new(colors.clone()),
            active_actions: ActiveActions::new(colors),
            last_wifi_update: std::time::Instant::now(),
            last_bluetooth_update: std::time::Instant::now(),
        };
        widget.get_colors();
        widget
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Action Center",
        options,
        Box::new(|cc| {
            // Register the phosphor icon font with egui
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(ActionCenterWidget::default()))
        }),
    )
}
