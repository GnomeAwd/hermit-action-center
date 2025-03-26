use eframe::egui;
use egui::style::HandleShape;
use egui::{Button, Color32, Label, Layout, Rect, RichText, Slider, Vec2, Widget};
use egui_phosphor::regular::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::process::Command;
use std::time::{Duration, Instant};

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
    brightness_value: f32,
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
                // let monitor_height = 1080;
                let x = monitor_width - 270 - 30;
                let y = 60;
                let width = 270;
                let height = 400;

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

        self.colors = colors;
    }
}

impl eframe::App for ActionCenterWidget {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set the global visual style with our custom colors
        {
            let mut style = (*ctx.style()).clone();
            style.visuals.window_fill = self.colors.background;
            style.visuals.panel_fill = self.colors.background;
            ctx.set_style(style);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Create a scrollable area for the list of networks that takes up the full width and height
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .max_height(ui.available_height() - 8.0) // Reduce top padding
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        // Use the Layout to ensure consistent spacing
                        ui.with_layout(Layout::left_to_right(egui::Align::TOP), |ui| {
                            // First grid (left side)
                            egui::Frame::new()
                                .fill(self.colors.surface)
                                .inner_margin(12.0)
                                .corner_radius(16.0)
                                .outer_margin(4.0) // Make sure both frames have the same margin
                                .show(ui, |ui| {
                                    egui::Grid::new("button_grid_left")
                                        .spacing([10.0, 8.0])
                                        .min_col_width(40.0) // Ensure consistent column width
                                        .show(ui, |ui| {
                                            // Create a circular Wifi button with an icon
                                            let wifi_response = ui.add(
                                                Button::new(
                                                    RichText::new(WIFI_HIGH)
                                                        .size(20.0)
                                                        .color(self.colors.on_primary),
                                                )
                                                .min_size(Vec2::new(40.0, 40.0))
                                                .corner_radius(20.0)
                                                .fill(self.colors.primary),
                                            );

                                            if wifi_response.clicked() {
                                                println!("Wifi clicked!");
                                            }

                                            let network_response = ui.add(
                                                Button::new(
                                                    RichText::new(NETWORK)
                                                        .size(20.0)
                                                        .color(self.colors.on_primary),
                                                )
                                                .min_size(Vec2::new(40.0, 40.0))
                                                .corner_radius(20.0)
                                                .fill(self.colors.primary),
                                            );

                                            if network_response.clicked() {
                                                println!("Network clicked!");
                                            }

                                            ui.end_row(); // Move to the next row

                                            let bluetooth_response = ui.add(
                                                Button::new(
                                                    RichText::new(BLUETOOTH)
                                                        .size(20.0)
                                                        .color(self.colors.on_primary),
                                                )
                                                .min_size(Vec2::new(40.0, 40.0))
                                                .corner_radius(20.0)
                                                .fill(self.colors.primary),
                                            );

                                            if bluetooth_response.clicked() {
                                                println!("Bluetooth clicked!");
                                            }

                                            let airplane_response = ui.add(
                                                Button::new(
                                                    RichText::new(AIRPLANE)
                                                        .size(20.0)
                                                        .color(self.colors.on_primary),
                                                )
                                                .min_size(Vec2::new(40.0, 40.0))
                                                .corner_radius(20.0)
                                                .fill(self.colors.primary),
                                            );

                                            if airplane_response.clicked() {
                                                println!("Flight Mode clicked!");
                                            }
                                        });
                                });

                            // Second grid (right side)
                            egui::Frame::new()
                                .fill(self.colors.surface)
                                .inner_margin(12.0)
                                .corner_radius(16.0)
                                .outer_margin(4.0) // Make sure both frames have the same margin
                                .show(ui, |ui| {
                                    egui::Grid::new("button_grid_right")
                                        .spacing([10.0, 8.0])
                                        .min_col_width(40.0) // Ensure consistent column width
                                        .show(ui, |ui| {
                                            let battery_response = ui.add(
                                                Button::new(
                                                    RichText::new(BATTERY_FULL)
                                                        .size(20.0)
                                                        .color(self.colors.on_primary),
                                                )
                                                .min_size(Vec2::new(40.0, 40.0))
                                                .corner_radius(20.0)
                                                .fill(self.colors.primary),
                                            );

                                            if battery_response.clicked() {
                                                println!("Battery clicked!");
                                            }

                                            let volume_response = ui.add(
                                                Button::new(
                                                    RichText::new(SPEAKER_HIGH)
                                                        .size(20.0)
                                                        .color(self.colors.on_primary),
                                                )
                                                .min_size(Vec2::new(40.0, 40.0))
                                                .corner_radius(20.0)
                                                .fill(self.colors.primary),
                                            );

                                            if volume_response.clicked() {
                                                println!("Volume clicked!");
                                            }
                                            ui.end_row(); // Move to the next row

                                            let screenshot_response = ui.add(
                                                Button::new(
                                                    RichText::new(CROP)
                                                        .size(20.0)
                                                        .color(self.colors.on_primary),
                                                )
                                                .min_size(Vec2::new(40.0, 40.0))
                                                .corner_radius(20.0)
                                                .fill(self.colors.primary),
                                            );

                                            if screenshot_response.clicked() {
                                                println!("Screenshot clicked!");
                                            }

                                            let lock_response = ui.add(
                                                Button::new(
                                                    RichText::new(LOCK)
                                                        .size(20.0)
                                                        .color(self.colors.on_primary),
                                                )
                                                .min_size(Vec2::new(40.0, 40.0))
                                                .corner_radius(20.0)
                                                .fill(self.colors.primary),
                                            );

                                            if lock_response.clicked() {
                                                println!("Lock clicked!");
                                            }
                                        });
                                });
                        });
                        ui.add_space(4.0);
                        // Add a brightness slider with the same total width as the grid blocks above
                        ui.with_layout(Layout::left_to_right(egui::Align::TOP), |ui| {
                            // Create a frame for the slider
                            egui::Frame::new()
                                .fill(self.colors.surface)
                                .inner_margin(12.0)
                                .corner_radius(16.0)
                                .outer_margin(4.0)
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        // Set a minimum width for the entire horizontal layout
                                        ui.set_min_width(ui.available_width());

                                        // Add icon on the left
                                        ui.add(Label::new(
                                            RichText::new(SUN)
                                                .size(20.0)
                                                .color(self.colors.on_surface),
                                        ));

                                        ui.add_space(10.0);

                                        // Get the remaining width for the slider
                                        let available_width = ui.available_width();

                                        // Make the slider take up all remaining space with appropriate styling
                                        {
                                            // Apply custom styling to match the theme
                                            let mut style = (*ui.ctx().style()).clone();
                                            style.visuals.widgets.active.bg_fill =
                                                self.colors.primary;
                                            style.visuals.widgets.inactive.bg_fill =
                                                Color32::from_gray(60);

                                            let old_style = ui.ctx().style().clone();
                                            ui.ctx().set_style(style);

                                            ui.add_sized(
                                                [available_width, 16.0],
                                                Slider::new(
                                                    &mut self.brightness_value,
                                                    0.0..=100.0,
                                                )
                                                .show_value(false)
                                                .handle_shape(HandleShape::Circle)
                                                .trailing_fill(true),
                                            );

                                            ui.ctx().set_style(old_style);
                                        }
                                    });
                                });
                        });
                    });
                });
        });
        self.place_widgets();
    }
}

impl Default for ActionCenterWidget {
    fn default() -> Self {
        let mut widget = Self {
            colors: Colors::default(),
            positioned: false,
            brightness_value: 50.0,
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
