use game_of_life::GameOfLifeApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Game of Life",
        native_options,
        Box::new(|cc| Box::new(GameOfLifeApp::new(cc)))
    );
}
