use eframe::{App, egui::{Grid, Button, Ui}, egui, epaint::{Rgba, Vec2, Rounding}};
use std::time;
use crate::SudokuApp;

impl SudokuApp {
    fn draw_board(&mut self, ui: &mut Ui) {
        Grid::new("board_grid")
            .spacing([0.0, 0.0])
            .min_col_width(60.0)
            .min_row_height(60.0)
            .show(ui, |ui: &mut Ui| {
                for y in 0..9 {
                    for x in 0..9 {
                        self.draw_tile(ui, x, y);
                    }
                    ui.end_row();
                }
            });
    }

    fn draw_tile(&mut self, ui: &mut Ui, x: usize, y: usize) {
        let color = if (x, y) == self.selected_tile {
            Rgba::RED
        } else {
            Rgba::BLUE
        };

        let tile = &self.board[x][y];
        let text = tile.map_or(String::from(""), |v| format!("{}", v));

        let button = ui.add(
            Button::new(text)
                .min_size(Vec2::new(58.0, 58.0))
                .rounding(Rounding::ZERO)
                .fill(color)
        );

        if button.clicked() {
            self.selected_tile = (x, y);
        }
    }

    fn draw_numpad(&mut self, ui: &mut Ui) {
        Grid::new("numpad")
            .spacing(Vec2::new(0.0, 0.0))
            .min_col_width(60.0)
            .min_row_height(60.0)
            .show(ui, |ui| {
                for i in 1..=9 {
                    self.draw_numpad_button(ui, i);
                    if i % 3 == 0 {
                        ui.end_row();
                    }
                }
            });
    }

    fn draw_numpad_button(&mut self, ui: &mut Ui, i: u8) {
        if ui.add(
            Button::new(format!("{}", i))
                .rounding(Rounding::ZERO)
                .fill(Rgba::WHITE)
                .min_size(Vec2::new(58.0, 58.0))
        ).clicked() {
            let (x, y) = self.selected_tile;
            let old_val = self.board[x][y];
            if old_val.is_none() && self.solvable(self.selected_tile, i) {
                self.collapse((x, y), i);
            }
        }
    }
}

impl App for SudokuApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Grid::new("main grid").show(ui, |ui| {
                self.draw_board(ui);
                self.draw_numpad(ui);
            });

            if ui.button("solve").clicked() {
                let t = time::SystemTime::now();
                self.solve();
                println!("{:?}", t.elapsed());
            }
        });
    }
}
