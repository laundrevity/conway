use eframe::egui::{self, Color32, Rect, Vec2};
use eframe::App;
use std::time::{Duration, Instant};

struct GameOfLifeApp {
    grid: Vec<Vec<bool>>, // true for alive, false for dead
    is_playing: bool, // track if the game is playing, e.g. evolving
    last_update: Instant,
    update_frequency: f32,
}

impl GameOfLifeApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            grid: vec![vec![false; 18]; 18],
            is_playing: false,
            last_update: Instant::now(), // Initialize the timer
            update_frequency: 0.5,
        }
    }

    fn draw_grid(&mut self, ui: &mut egui::Ui) {
        let cell_size = 20.0; // size of each cell in the grid
        let grid_size = cell_size * 16.0; // total size of the grid
        let (response, painter) = ui.allocate_painter(Vec2::splat(grid_size), egui::Sense::click());
    
        // Check for the click and toggle cell state
        if response.clicked() {
            if let Some(mouse_pos) = response.interact_pointer_pos() {
                // Calculate which cell was clicked
                let x = ((mouse_pos.x - response.rect.left()) / cell_size).floor() as usize;
                let y = ((mouse_pos.y - response.rect.top()) / cell_size).floor() as usize;
                if x < 16 && y < 16 {
                    // Flip the state of the clicked cell
                    self.grid[x][y] = !self.grid[x][y];
                }
            }
        }

        // Define the stroke for the grid lines
        let grid_line_stroke = egui::Stroke::new(1.0, Color32::WHITE);

        // Draw only central part of grid
        for x in 1..17 {
            for y in 1..17 {
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

        for x in 0..18 {
            for y in 0..18 {
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
                if ni >= 0 && ni < 18 as isize && nj >= 0 && nj < 18 as isize {
                    if self.grid[ni as usize][nj as usize] {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn clear_grid(&mut self) {
        self.grid = vec![vec![false; 18]; 18];
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

            let now = Instant::now();
            // Check if more than one second has passed and game is playing
            if self.is_playing && now.duration_since(self.last_update) >= Duration::from_secs_f32(self.update_frequency){
                self.update_game_state();
                self.last_update = now; // Reset the timer
            }
        });

        // Request a repaint
        ctx.request_repaint();
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Game of Life", 
        native_options, 
        Box::new(|cc| Box::new(GameOfLifeApp::new(cc)))
    );
}
