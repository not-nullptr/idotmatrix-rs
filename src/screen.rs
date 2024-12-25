use std::{error::Error, time::Duration};

use btleplug::{
    api::{Characteristic, Peripheral as _},
    platform::Peripheral,
};
use tokio::time;

use crate::constants::MATRIX_WRITE;

pub struct Screen {
    peripheral: Peripheral,
    write_char: Option<Characteristic>,
}

impl Screen {
    pub fn new(peripheral: Peripheral) -> Self {
        Self {
            peripheral,
            write_char: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        self.peripheral.connect().await?;
        self.peripheral.discover_services().await?;
        self.write_char = self
            .peripheral
            .characteristics()
            .iter()
            .find(|x| x.uuid == MATRIX_WRITE)
            .cloned();
        println!("{:?}", self.write_char);
        // let bytes: [u8; 10] = [10, 0, 5, 1, 0, 255, 0, 0, 8, 8];
        // self.peripheral.write(characteristic, data, write_type)
        Ok(())
    }
}
