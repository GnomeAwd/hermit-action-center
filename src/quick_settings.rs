use eframe::egui;
use egui::{Button, Color32, Label, Layout, RichText, Vec2};
use egui_phosphor::regular::*;
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Default)]
pub struct QuickSettingsState {
    pub wifi_enabled: bool,
    pub wifi_network_name: String,
    pub network_enabled: bool,
    pub bluetooth_enabled: bool,
    pub bluetooth_device_name: String,
    pub airplane_enabled: bool,
}

pub struct QuickSettings {
    state: QuickSettingsState,
    colors: super::Colors,
}

impl QuickSettings {
    pub fn new(colors: super::Colors) -> Self {
        let mut qs = Self {
            state: QuickSettingsState::default(),
            colors,
        };

        // Initialize states on startup
        qs.update_wifi_state();
        qs.update_bluetooth_state();

        qs
    }

    /// Fetches current WiFi state and connected network
    pub fn update_wifi_state(&mut self) {
        // Check if WiFi is enabled
        let wifi_status = Command::new("nmcli").args(&["radio", "wifi"]).output();

        if let Ok(output) = wifi_status {
            let status_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            self.state.wifi_enabled = status_str == "enabled";

            // If WiFi is enabled, get the connected network name
            if self.state.wifi_enabled {
                let network_cmd = Command::new("nmcli")
                    .args(&["-t", "-f", "NAME,DEVICE", "connection", "show", "--active"])
                    .output();

                if let Ok(net_output) = network_cmd {
                    let net_str = String::from_utf8_lossy(&net_output.stdout);
                    self.state.wifi_network_name = String::new(); // Reset before checking

                    // Parse the output for connections on WiFi devices (wlan0, etc.)
                    for line in net_str.lines() {
                        let parts: Vec<&str> = line.split(':').collect();
                        if parts.len() >= 2 && parts[1].starts_with("wl") {
                            self.state.wifi_network_name = parts[0].to_string();
                            break;
                        }
                    }
                }
            } else {
                self.state.wifi_network_name = String::new();
            }
        }
    }

    /// Fetches current Bluetooth state and connected device
    pub fn update_bluetooth_state(&mut self) {
        // Check if Bluetooth is powered on using grep to directly get the power state
        let bt_status = Command::new("sh")
            .arg("-c")
            .arg("bluetoothctl show | grep 'Powered:'")
            .output();

        if let Ok(output) = bt_status {
            let output_str = String::from_utf8_lossy(&output.stdout);

            // Parse the line "Powered: yes/no" and check if it contains "yes"
            self.state.bluetooth_enabled = output_str.contains("Powered: yes");

            // If Bluetooth is enabled, check for connected devices
            if self.state.bluetooth_enabled {
                // Get connected devices - using direct shell command for more reliable output
                let devices_cmd = Command::new("sh")
                    .arg("-c")
                    .arg("bluetoothctl devices Connected")
                    .output();

                if let Ok(devices_output) = devices_cmd {
                    let devices_str = String::from_utf8_lossy(&devices_output.stdout);
                    self.state.bluetooth_device_name = String::new(); // Reset before checking

                    // Parse the first connected device (if any)
                    // Format is typically: "Device XX:XX:XX:XX:XX:XX DeviceName"
                    for line in devices_str.lines() {
                        if line.starts_with("Device") {
                            let parts: Vec<&str> = line.split(' ').collect();
                            if parts.len() >= 3 {
                                // Join all parts after the MAC address to get the full device name
                                self.state.bluetooth_device_name = parts[2..].join(" ");
                                break;
                            }
                        }
                    }
                }
            } else {
                self.state.bluetooth_device_name = String::new();
            }
        }
    }

    /// Toggle WiFi on/off
    fn toggle_wifi(&mut self) {
        let new_state = if self.state.wifi_enabled { "off" } else { "on" };

        let _ = Command::new("nmcli")
            .args(&["radio", "wifi", new_state])
            .output();

        // Update the state after toggling
        self.update_wifi_state();
    }

    /// Toggle Bluetooth on/off
    fn toggle_bluetooth(&mut self) {
        let new_state = if self.state.bluetooth_enabled {
            "off"
        } else {
            "on"
        };

        // Use echo to pipe commands to bluetoothctl for more reliable execution
        let mut child = Command::new("bluetoothctl")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start bluetoothctl");

        // Write commands to bluetoothctl stdin
        if let Some(mut stdin) = child.stdin.take() {
            let cmd = format!("power {}\nquit\n", new_state);
            stdin
                .write_all(cmd.as_bytes())
                .expect("Failed to write to bluetoothctl stdin");
        }

        // Wait for command to complete
        let _ = child.wait();

        // Update the state after toggling - with a small delay to allow bluetoothctl to complete
        std::thread::sleep(std::time::Duration::from_millis(500));
        self.update_bluetooth_state();
    }

    pub fn update_colors(&mut self, colors: super::Colors) {
        self.colors = colors;
    }

    pub fn state(&self) -> &QuickSettingsState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut QuickSettingsState {
        &mut self.state
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Quick Settings grid
            egui::Frame::new()
                .fill(self.colors.surface)
                .inner_margin(12.0)
                .corner_radius(18.0)
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    egui::Grid::new("quick_settings_grid")
                        .spacing([10.0, 22.0])
                        .min_col_width(40.0)
                        .show(ui, |ui| {
                            let on_primary = self.colors.on_primary;
                            let on_surface = self.colors.on_surface;
                            let primary = self.colors.primary;
                            let surface = self.colors.surface;

                            // WiFi row
                            ui.horizontal(|ui| {
                                ui.set_min_width(ui.available_width());
                                if self.add_button(
                                    ui,
                                    WIFI_HIGH,
                                    "Wi-Fi",
                                    self.state.wifi_enabled,
                                    on_primary,
                                    on_surface,
                                    primary,
                                    surface,
                                ) {
                                    // Toggle WiFi when clicked
                                    self.toggle_wifi();
                                }
                                ui.vertical_centered(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new("Wi-Fi").color(self.colors.on_surface),
                                        );
                                    });
                                    ui.horizontal(|ui| {
                                        let status_text = if self.state.wifi_enabled {
                                            if !self.state.wifi_network_name.is_empty() {
                                                &self.state.wifi_network_name
                                            } else {
                                                "On"
                                            }
                                        } else {
                                            "Off"
                                        };

                                        ui.label(
                                            RichText::new(status_text)
                                                .size(12.0)
                                                .color(self.colors.on_surface),
                                        );
                                    });
                                });
                            });
                            ui.end_row();

                            // Bluetooth row
                            ui.horizontal(|ui| {
                                if self.add_button(
                                    ui,
                                    BLUETOOTH,
                                    "Bluetooth",
                                    self.state.bluetooth_enabled,
                                    on_primary,
                                    on_surface,
                                    primary,
                                    surface,
                                ) {
                                    // Toggle Bluetooth when clicked
                                    self.toggle_bluetooth();
                                }
                                ui.vertical_centered(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new("Bluetooth")
                                                .color(self.colors.on_surface),
                                        );
                                    });
                                    ui.horizontal(|ui| {
                                        let status_text = if self.state.bluetooth_enabled {
                                            if !self.state.bluetooth_device_name.is_empty() {
                                                &self.state.bluetooth_device_name
                                            } else {
                                                "On"
                                            }
                                        } else {
                                            "Off"
                                        };

                                        ui.label(
                                            RichText::new(status_text)
                                                .size(12.0)
                                                .color(self.colors.on_surface),
                                        );
                                    });
                                });
                            });
                            ui.end_row();

                            // Network row
                            ui.horizontal(|ui| {
                                if self.add_button(
                                    ui,
                                    NETWORK,
                                    "Ethernet",
                                    self.state.network_enabled,
                                    on_primary,
                                    on_surface,
                                    primary,
                                    surface,
                                ) {
                                    self.state.network_enabled = !self.state.network_enabled;
                                }
                                ui.vertical_centered(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new("Ethernet").color(self.colors.on_surface),
                                        );
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new("Disconnected")
                                                .size(12.0)
                                                .color(self.colors.on_surface),
                                        );
                                    });
                                });
                            });
                            ui.end_row();

                            // Airplane mode row
                            ui.horizontal(|ui| {
                                if self.add_button(
                                    ui,
                                    AIRPLANE,
                                    "Airplane Mode",
                                    self.state.airplane_enabled,
                                    on_primary,
                                    on_surface,
                                    primary,
                                    surface,
                                ) {
                                    self.state.airplane_enabled = !self.state.airplane_enabled;
                                }
                                ui.vertical_centered(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new("Airplane Mode")
                                                .color(self.colors.on_surface),
                                        );
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new("Off")
                                                .size(12.0)
                                                .color(self.colors.on_surface),
                                        );
                                    });
                                });
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                if self.add_button(
                                    ui,
                                    RECORD,
                                    "Screen Recording",
                                    self.state.airplane_enabled,
                                    on_primary,
                                    on_surface,
                                    primary,
                                    surface,
                                ) {
                                    self.state.airplane_enabled = !self.state.airplane_enabled;
                                }
                                ui.vertical_centered(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new("Screen Recording")
                                                .color(self.colors.on_surface),
                                        );
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new("Off")
                                                .size(12.0)
                                                .color(self.colors.on_surface),
                                        );
                                    });
                                });
                            });
                            ui.end_row();

                            ui.horizontal(|ui| {
                                if self.add_button(
                                    ui,
                                    BELL_SLASH,
                                    "Do Not Disturb",
                                    self.state.airplane_enabled,
                                    on_primary,
                                    on_surface,
                                    primary,
                                    surface,
                                ) {
                                    self.state.airplane_enabled = !self.state.airplane_enabled;
                                }
                                ui.vertical_centered(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new("Do Not Disturb")
                                                .color(self.colors.on_surface),
                                        );
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new("Off")
                                                .size(12.0)
                                                .color(self.colors.on_surface),
                                        );
                                    });
                                });
                            });
                            ui.end_row();
                        });
                });
        });
    }

    fn add_button(
        &self,
        ui: &mut egui::Ui,
        icon: &str,
        tooltip: &str,
        is_enabled: bool,
        on_primary: Color32,
        on_surface: Color32,
        primary: Color32,
        surface: Color32,
    ) -> bool {
        let button = ui.add(
            Button::new(RichText::new(icon).size(20.0).color(if is_enabled {
                on_primary
            } else {
                on_surface
            }))
            .min_size(Vec2::new(40.0, 40.0))
            .corner_radius(20.0)
            .fill(if is_enabled { primary } else { surface }),
        );

        let clicked = button.clicked();
        button.on_hover_text(tooltip);
        clicked
    }
}
