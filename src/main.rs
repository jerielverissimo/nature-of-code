use std::error::Error;

mod app;
mod walker;

use app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new()?;
    app.run();

    Ok(())
}
