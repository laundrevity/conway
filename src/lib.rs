use eframe::egui::{self, Color32, Rect, Vec2};
use eframe::App;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(not(target_arch = "wasm32"))]
use once_cell::sync::Lazy;

#[cfg(not(target_arch = "wasm32"))]
static START_TIME: Lazy<Instant> = Lazy::new(Instant::now);

pub struct GameOfLifeApp {
    grid_length: usize,
    grid: Vec<Vec<bool>>, // true for alive, false for dead
    is_playing: bool, // track if the game is playing, e.g. evolving
    last_update: f64,
    update_frequency: f32,
}

impl GameOfLifeApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let grid_length = 32;
        let grid = vec![vec![false; grid_length + 2]; grid_length + 2];
        Self {
            grid_length,
            grid,
            is_playing: false,
            last_update: get_current_time(),
            update_frequency: 0.5,
        }
    }

    fn draw_grid(&mut self, ui: &mut egui::Ui) {
        let cell_size = 20.0; // size of each cell in the grid
        let grid_size = cell_size * (self.grid_length as f32); // total size of the grid
        let (response, painter) = ui.allocate_painter(Vec2::splat(grid_size), egui::Sense::click());
    
        // Check for the click and toggle cell state
        if response.clicked() {
            if let Some(mouse_pos) = response.interact_pointer_pos() {
                // Calculate which cell was clicked
                let x = ((mouse_pos.x - response.rect.left()) / cell_size).floor() as usize;
                let y = ((mouse_pos.y - response.rect.top()) / cell_size).floor() as usize;
                if x < self.grid_length && y < self.grid_length {
                    // Flip the state of the clicked cell
                    self.grid[x][y] = !self.grid[x][y];
                }
            }
        }

        // Define the stroke for the grid lines
        let grid_line_stroke = egui::Stroke::new(1.0, Color32::WHITE);

        // Draw only central part of grid
        for x in 1..(self.grid_length+1) {
            for y in 1..(self.grid_length+1) {
                let rect = Rect::from_min_size(
                    response.rect.min + Vec2::new(x as f32 * cell_size, y as f32 * cell_size), 
                    Vec2::splat(cell_size),
                );
                let color = if self.grid[x][y] {
                    Color32::RED // Alive
                } else {
                    Color32::BLACK // Dead
                };
                painter.rect_filled(rect, 0.0, color);
                painter.rect_stroke(rect, 0.0, grid_line_stroke); // Grid lines
            }
        }
    }

    fn update_game_state(&mut self) {
        let mut new_grid = self.grid.clone();

        for x in 0..(self.grid_length + 2) {
            for y in 0..(self.grid_length + 2) {
                let alive_neighbors = self.count_alive_neighbors(x, y);
                
                if self.grid[x][y] {
                    // Rule for alive cells
                    new_grid[x][y] = alive_neighbors == 2 || alive_neighbors == 3;
                } else {
                    // Rule for dead cells
                    new_grid[x][y] = alive_neighbors == 3;
                }
            }
        }

        self.grid = new_grid;
    }

    fn count_alive_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for i in 0..3 {
            for j in 0..3 {
                if i == 1 && j == 1 { continue; } // Skip the cell itself

                let ni = x as isize + i - 1; // x-index of i-th offset (so for i=0 is left , i=2 is right)
                let nj = y as isize + j - 1; // y-index of j-th offset (so for i=0 is above, j=2 is below)

                // Check if the neighbor is within grid bounds, including "buffer"
                if ni >= 0 && ni < (self.grid_length + 2) as isize && nj >= 0 && nj < (self.grid_length + 2) as isize {
                    if self.grid[ni as usize][nj as usize] {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn clear_grid(&mut self) {
        self.grid = vec![vec![false; self.grid_length + 2]; self.grid_length + 2];
    }
}

impl App for GameOfLifeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Conway's Game of Life");
            self.draw_grid(ui);

            // Play and Pause buttons
            ui.horizontal(|ui| {
                if ui.button("Play").clicked() {
                    self.is_playing = true;
                }
                if ui.button("Pause").clicked() {
                    self.is_playing = false;
                }
                if ui.button("Clear").clicked() {
                    self.clear_grid();
                    self.is_playing = false;
                }

                // Display game state
                ui.label(if self.is_playing { "Playing" } else { "Paused" });

                ui.add(egui::Slider::new(&mut self.update_frequency, 0.1..=2.0).text("Update frequency (s)"));
            });

            let now = get_current_time();
            // Check if more than one second has passed and game is playing
            if self.is_playing && (now - self.last_update) >= self.update_frequency as f64 {
                self.update_game_state();
                self.last_update = now; // Reset the timer
            }
        });

        // Request a repaint
        ctx.request_repaint();
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;
    use super::GameOfLifeApp; // Import our app
    
    #[wasm_bindgen]
    pub struct WebHandle {
        runner: eframe::WebRunner,
    }

    #[wasm_bindgen]
    impl WebHandle {
        // Installs a panic hook, then returns
        #[allow(clippy::new_without_default)]
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            // Redirect [`log`] message to `console.log` and friends:
            eframe::WebLogger::init(log::LevelFilter::Debug).ok();

            Self {
                runner: eframe::WebRunner::new(),
            }
        }

        // Call this once from JavaScript to start the app
        #[wasm_bindgen]
        pub async fn start(&self, canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
            self.runner
                .start(
                    canvas_id,
                    eframe::WebOptions::default(),
                    Box::new(|cc| Box::new(GameOfLifeApp::new(cc))),
                )
                .await
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn get_current_time() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now() / 1000.0
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_current_time() -> f64 {
    START_TIME.elapsed().as_secs_f64()
}