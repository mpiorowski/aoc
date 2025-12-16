use crate::app::App;
use color_eyre::eyre::Result;

mod app;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Setup color-eyre
    color_eyre::install()?;

    // 2. Setup TUI Panic Hook
    // Ensure we restore the terminal if the app panics
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        ratatui::restore();
        original_hook(panic_info);
    }));

    // 3. Initialize Terminal
    let mut terminal = ratatui::init();

    // 4. Run the app
    let app_result = App::new().run(&mut terminal).await;

    // 5. Restore Terminal (Normal Exit)
    ratatui::restore();

    // 6. Print errors if any
    app_result
}
