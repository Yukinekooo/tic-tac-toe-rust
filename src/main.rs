#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::{
    Align, Align2, Button, CentralPanel, Color32, Layout, RichText, TextStyle, Vec2, Window,
};

mod utils;

use utils::{components, display_grid};

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(305.0, 320.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Tic Tac Toe",
        options,
        Box::new(|_cc| Box::new(Board::default())),
    )
}

pub struct Board {
    grid: [[&'static str; 3]; 3],
    current_player: u8,
    winner: u8,
    game_ended: bool,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            grid: [[""; 3]; 3],
            current_player: 1,
            winner: 0,
            game_ended: false,
        }
    }
}

impl eframe::App for Board {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            display_grid::display_grid(ui, self);
            components::new_label(
                ui,
                format!("Current player: {}", self.current_player).as_str(),
            );
            if self.game_ended {
                Window::new("Game ended")
                    .resizable(false)
                    .collapsible(false)
                    .fixed_size(Vec2::new(200.0, 100.0))
                    .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
                    .show(ui.ctx(), |ui| {
                        ui.with_layout(Layout::top_down(Align::Center), |ui| {
                            ui.spacing_mut().item_spacing.y = 10.0;
                            components::new_label(
                                ui,
                                format!("Player {} won!", self.winner).as_str(),
                            );
                            if ui
                                .add(
                                    Button::new(
                                        RichText::new("Restart")
                                            .color(Color32::from_rgb(255, 255, 255))
                                            .strong()
                                            .size(15.0)
                                            .text_style(TextStyle::Button),
                                    )
                                    .min_size(Vec2::new(180.0, 20.0)),
                                )
                                .clicked()
                            {
                                *self = Board::default();
                            }
                        });
                    });
            }
        });
    }
}
