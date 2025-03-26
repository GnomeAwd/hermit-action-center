use eframe::egui;
use egui::{Button, Color32, Label, Layout, RichText, Vec2};
use egui_phosphor::regular::*;

#[derive(Default)]
pub struct QuickSettingsState {
    pub wifi_enabled: bool,
    pub network_enabled: bool,
    pub bluetooth_enabled: bool,
    pub airplane_enabled: bool,
}

pub struct QuickSettings {
    state: QuickSettingsState,
    colors: super::Colors,
}

impl QuickSettings {
    pub fn new(colors: super::Colors) -> Self {
        Self {
            state: QuickSettingsState::default(),
            colors,
        }
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
                                    self.state.wifi_enabled = !self.state.wifi_enabled;
                                }
                                ui.vertical_centered(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new("Wi-Fi").color(self.colors.on_surface),
                                        );
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new("Off")
                                                .size(11.0)
                                                .color(self.colors.on_surface),
                                        );
                                    });
                                });
                            });
                            ui.end_row();

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
                                    self.state.bluetooth_enabled = !self.state.bluetooth_enabled;
                                }
                                ui.vertical_centered(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new("Bluetooth")
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
