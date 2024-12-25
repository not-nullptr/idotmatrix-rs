mod constants;
mod scanner;
mod screen;
mod util;

use btleplug::api::{Manager as _, Peripheral as _};
use btleplug::platform::{Manager, Peripheral};
use scanner::Scanner;
use screen::Screen;
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();
    let scanner = Scanner::new(central);
    let mut peripherals = scanner.scan(Duration::from_secs(2)).await?;
    if let Some(peripheral) = peripherals.first() {
        let peripheral = peripherals.remove(0);
        handle_device(peripheral).await?;
    }
    println!("No iDotMatrix devices found.");
    Ok(())
}

async fn handle_device(peripheral: Peripheral) -> Result<(), Box<dyn Error>> {
    let mut screen = Screen::new(peripheral);
    screen.connect().await?;
    Ok(())
}
