use egui::{Button, Color32, Grid, RichText, TextStyle, Ui, Vec2};

use crate::utils::game_handlers;
use crate::Board;

pub fn display_grid(ui: &mut Ui, board: &mut Board) {
    let button_size = Vec2::new(90.0, 80.0);
    let mut x = 0;
    let mut y = 0;
    Grid::new("main_grid").show(ui, |ui| {
        for _ in 0..3 {
            x = 0;
            for _ in 0..3 {
                if ui
                    .add(
                        Button::new(
                            RichText::new(board.grid[x][y])
                                .text_style(TextStyle::Button)
                                .color(Color32::from_rgb(255, 255, 255))
                                .size(40.0),
                        )
                        .min_size(button_size),
                    )
                    .clicked()
                {
                    game_handlers::handle_clicked_button(board, x, y);
                };

                x += 1;
            }
            y += 1;
            ui.end_row();
        }
    });
}
