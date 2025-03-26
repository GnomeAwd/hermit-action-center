use eframe::egui;
use egui::{Color32, Rect, RichText};

use crate::Colors;

/// Draw a custom colored slider that matches the design in the image
pub fn draw_colored_slider(
    ui: &mut egui::Ui,
    value: &mut f32,
    full_width: f32,
    colors: &Colors,
    icon: Option<&str>,
) {
    let height = 40.0; // Height to match the image

    // Configure colors - based on the image and theme
    let track_bg = Color32::from_rgb(80, 80, 80); // Medium gray background for the track
    let filled_color = colors.primary; // Use primary color for the filled portion
    let thumb_color = Color32::WHITE; // Keep thumb white for contrast
    let icon_color = colors.background; // Use on_surface color for icon

    // Calculate filled width based on the value
    let fill_ratio = (*value / 100.0).clamp(0.0, 1.0);
    let filled_width = full_width * fill_ratio;

    // Get available space and create a rectangle
    let (rect, response) = ui.allocate_at_least(
        egui::vec2(full_width, height),
        egui::Sense::click_and_drag(),
    );

    // Handle interaction - update value based on mouse click/drag
    if response.dragged() || response.clicked() {
        if let Some(mouse_pos) = response.interact_pointer_pos() {
            let new_ratio = ((mouse_pos.x - rect.min.x) / rect.width()).clamp(0.0, 1.0);
            *value = new_ratio * 100.0;
        }
    }

    // Get the painter to draw custom visuals
    let painter = ui.painter();

    // Draw the background track (full width)
    painter.rect_filled(
        rect,
        height / 2.0, // Make the track fully rounded (corner radius = height/2)
        track_bg,     // Dark background
    );

    // Draw the filled portion (left side of the slider)
    if filled_width > 0.0 {
        painter.rect_filled(
            Rect::from_min_size(rect.min, egui::vec2(filled_width, height)),
            height / 2.0, // Fully rounded corners
            filled_color, // Use primary color for the filled part
        );
    }

    // Calculate thumb position - make it exactly match the track height
    let thumb_radius = height - 6.0; // Slightly smaller than the track height

    // Adjust the x position clamping to keep the thumb fully within the slider bounds
    let thumb_radius_half = thumb_radius / 2.0;
    let thumb_x = rect.min.x + (rect.width() * fill_ratio);

    // Clamp the thumb position to prevent it from extending outside the slider area
    let clamped_thumb_x = thumb_x.clamp(
        rect.min.x + thumb_radius_half,
        rect.max.x - thumb_radius_half,
    );

    let thumb_center = egui::pos2(clamped_thumb_x, rect.center().y);

    // Draw white circle for the thumb
    painter.circle_filled(thumb_center, thumb_radius / 2.0, thumb_color);

    // Draw the icon if provided
    if let Some(icon_str) = icon {
        let icon_pos = rect.min + egui::vec2(20.0, height / 2.0);
        painter.text(
            icon_pos,
            egui::Align2::LEFT_CENTER,
            icon_str,
            egui::FontId::proportional(20.0),
            icon_color,
        );
    }
}

/// A component for displaying a slider with an icon
pub struct IconSlider {
    pub value: f32,
    pub icon: String,
    pub title: Option<String>,
}

impl IconSlider {
    pub fn new(value: f32, icon: String) -> Self {
        Self {
            value,
            icon,
            title: None,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, colors: &Colors) {
        // If we have a title, show it
        if let Some(title) = &self.title {
            ui.horizontal(|ui| {
                ui.add_space(8.0);
                ui.label(RichText::new(title).color(colors.on_surface));
                ui.add_space(8.0);
            });
            ui.add_space(4.0); // Reduced space after title
        }

        ui.add_space(4.0); // Add slight spacing to match the image

        // Create a container for the slider
        let height = 40.0;

        // Reserve space for the slider
        let available_width = ui.available_width();

        // Draw the slider directly with the icon built-in
        draw_colored_slider(
            ui,
            &mut self.value,
            available_width,
            colors,
            Some(&self.icon),
        );

        ui.add_space(4.0); // Add slight spacing after slider
    }
}
