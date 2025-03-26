use eframe::egui;
use egui::{Color32, Frame, Layout, RichText, Vec2};
use egui_phosphor::regular::*;

pub struct ActiveActions {
    colors: super::Colors,
}

impl ActiveActions {
    pub fn new(colors: super::Colors) -> Self {
        Self { colors }
    }

    pub fn update_colors(&mut self, colors: super::Colors) {
        self.colors = colors;
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Music Player Block
            Frame::new()
                .fill(self.colors.surface)
                .corner_radius(18.0)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    ui.vertical(|ui| {
                        ui.add_space(12.0);
                        // Album art placeholder
                        Frame::new()
                            .fill(Color32::from_gray(50))
                            .corner_radius(8.0)
                            .show(ui, |ui| {
                                ui.set_min_width(ui.available_width());
                                ui.add_space(24.0);
                                ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                                    ui.label(
                                        RichText::new(MUSIC_NOTE)
                                            .size(24.0)
                                            .color(self.colors.on_surface),
                                    );
                                });
                                ui.add_space(24.0);
                            });

                        ui.add_space(12.0);

                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("Track title").color(self.colors.on_surface));
                            ui.label(
                                RichText::new("Artist name")
                                    .size(12.0)
                                    .color(self.colors.on_surface.gamma_multiply(0.7)),
                            );
                        });
                    });

                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.add_space(24.0);
                        ui.label(
                            RichText::new(SKIP_BACK)
                                .size(20.0)
                                .color(self.colors.on_surface),
                        );
                        ui.add_space(12.0);
                        ui.label(RichText::new(PLAY).size(20.0).color(self.colors.on_surface));
                        ui.add_space(12.0);
                        ui.label(
                            RichText::new(SKIP_FORWARD)
                                .size(20.0)
                                .color(self.colors.on_surface),
                        );
                    });
                    ui.add_space(12.0);
                });

            ui.add_space(8.0);

            // Focus Mode Block
            Frame::new()
                .fill(self.colors.surface)
                .corner_radius(18.0)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.set_min_width(ui.available_width());
                        ui.add_space(34.0);
                        ui.label(RichText::new(MOON).size(20.0).color(self.colors.on_surface));
                        ui.add_space(4.0);
                        ui.label(RichText::new("Focus").color(self.colors.on_surface));
                    });
                });

            ui.add_space(8.0);

            // Create a 3x2 button layout using horizontal layouts
            // Each row is a separate horizontal layout

            // First row of buttons
            ui.horizontal(|ui| {
                ui.set_min_width(ui.available_width());

                ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                    egui::Frame::new()
                        .fill(self.colors.surface)
                        .corner_radius(18.0)
                        .inner_margin(6.0)
                        .show(ui, |ui| {
                            ui.add(
                                egui::Button::new(
                                    RichText::new(CORNERS_OUT)
                                        .size(20.0)
                                        .color(self.colors.on_surface),
                                )
                                .fill(Color32::TRANSPARENT)
                                .min_size(Vec2::new(36.0, 36.0)),
                            );
                        });

                    ui.add_space(4.0);
                    egui::Frame::new()
                        .fill(self.colors.surface)
                        .corner_radius(18.0)
                        .inner_margin(6.0)
                        .show(ui, |ui| {
                            ui.add(
                                egui::Button::new(
                                    RichText::new(EYE).size(20.0).color(self.colors.on_surface),
                                )
                                .fill(Color32::TRANSPARENT)
                                .min_size(Vec2::new(36.0, 36.0)),
                            );
                        });
                    ui.add_space(4.0);
                    egui::Frame::new()
                        .fill(self.colors.surface)
                        .corner_radius(18.0)
                        .inner_margin(6.0)
                        .show(ui, |ui| {
                            ui.add(
                                egui::Button::new(
                                    RichText::new(SPEAKER_HIGH)
                                        .size(20.0)
                                        .color(self.colors.on_surface),
                                )
                                .fill(Color32::TRANSPARENT)
                                .min_size(Vec2::new(36.0, 36.0)),
                            );
                        });
                });
            });
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.set_min_width(ui.available_width());
                ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                    egui::Frame::new()
                        .fill(self.colors.surface)
                        .corner_radius(18.0)
                        .inner_margin(6.0)
                        .show(ui, |ui| {
                            ui.add(
                                egui::Button::new(
                                    RichText::new(FOLDER_OPEN)
                                        .size(20.0)
                                        .color(self.colors.on_surface),
                                )
                                .fill(Color32::TRANSPARENT)
                                .min_size(Vec2::new(36.0, 36.0)),
                            );
                        });

                    ui.add_space(4.0);
                    egui::Frame::new()
                        .fill(self.colors.surface)
                        .corner_radius(18.0)
                        .inner_margin(6.0)
                        .show(ui, |ui| {
                            ui.add(
                                egui::Button::new(
                                    RichText::new(DOWNLOAD_SIMPLE)
                                        .size(20.0)
                                        .color(self.colors.on_surface),
                                )
                                .fill(Color32::TRANSPARENT)
                                .min_size(Vec2::new(36.0, 36.0)),
                            );
                        });
                    ui.add_space(4.0);
                    egui::Frame::new()
                        .fill(self.colors.surface)
                        .corner_radius(18.0)
                        .inner_margin(6.0)
                        .show(ui, |ui| {
                            ui.add(
                                egui::Button::new(
                                    RichText::new(IMAGES_SQUARE)
                                        .size(20.0)
                                        .color(self.colors.on_surface),
                                )
                                .fill(Color32::TRANSPARENT)
                                .min_size(Vec2::new(36.0, 36.0)),
                            );
                        });
                });

                // Frame::new()
                //     .fill(self.colors.surface)
                //     .corner_radius(18.0)
                //     .inner_margin(6.0)
                //     .show(ui, |ui| {
                //         ui.set_min_width(ui.available_width());

                //     });
            });
        });
    }
}
