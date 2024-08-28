use eframe::egui;
use eframe::emath::Align;
use eframe::epaint::Color32;
use egui::Layout;

use egui_comps::tabbar::*;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "TabBar Example",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    selected_tab: usize,
    selected_tab2: usize,
    underline: bool,
    heading: bool,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            selected_tab: 0,
            selected_tab2: 0,
            underline: false,
            heading: false,
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("TabBar Example");
                ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui);
                });
            });

            // Add the first TabBar
            let tabs = vec!{"Tab0", "Tab1", "Tab2"};
            if ui
                .add(TabBar::new(tabs, &mut self.selected_tab, &ui.visuals()))
                .clicked()
            {
                println!("clicked tab={}", self.selected_tab);
            }

            match self.selected_tab {
                0 => {
                    ui.colored_label(Color32::RED, "Content for Tab0");
                }
                1 => {
                    ui.colored_label(Color32::BLUE, "Content for Tab1");
                }
                2 => {
                    ui.colored_label(Color32::GREEN, "Content for Tab2");
                }
                _ => (),
            }

            ui.add_space(20.0);
            ui.separator();
            ui.checkbox(&mut self.underline, "underline");
            ui.checkbox(&mut self.heading, "heading");
            ui.add_space(10.0);

            // Add another TabBar
            let tabs = vec!{"TabA", "TabB", "TabC"};
            ui.add(
                TabBar::new(tabs, &mut self.selected_tab2, &ui.visuals())
                    .selected_bg(
                        Color32::from_rgb(0xf6, 0xb1, 0x7a),
                        Color32::from_rgb(0x6e, 0x85, 0xb7),
                    )
                    .selected_fg(Color32::BLACK, Color32::WHITE)
                    .hover_bg(
                        Color32::from_rgb(0x70, 0x77, 0xa1),
                        Color32::from_rgb(218, 207, 181),
                    )
                    .hover_fg(Color32::WHITE, Color32::BLACK)
                    .bg(
                        Color32::from_rgb(0x42, 0x47, 0x69),
                        Color32::from_rgb(226, 221, 213),
                    )
                    .fg(Color32::LIGHT_GRAY, Color32::DARK_GRAY)
                    .heading(self.heading)
                    .underline(self.underline),
            );
            match self.selected_tab2 {
                0 => {
                    ui.colored_label(Color32::RED, "Content for TabA");
                }
                1 => {
                    ui.colored_label(Color32::BLUE, "Content for TabB");
                }
                2 => {
                    ui.colored_label(Color32::GREEN, "Content for TabC");
                }
                _ => (),
            }
        });
    }
}
