use std::error::Error;

mod app;

use app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new()?;
    app.run();

    Ok(())
}
