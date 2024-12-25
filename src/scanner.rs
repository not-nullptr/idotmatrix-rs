use std::{error::Error, time::Duration};

use btleplug::{
    api::{Central as _, Peripheral as _, ScanFilter},
    platform::{Adapter, Peripheral},
};
use tokio::time;

use crate::constants::MATRIX_PREFIX;

pub struct Scanner {
    adapter: Adapter,
}

impl Scanner {
    pub fn new(adapter: Adapter) -> Self {
        Self { adapter }
    }

    pub async fn scan(&self, duration: Duration) -> Result<Vec<Peripheral>, Box<dyn Error>> {
        self.adapter.start_scan(ScanFilter::default()).await?;
        time::sleep(duration).await;
        Ok(Self::find_idotmatrix(&self.adapter).await)
    }

    async fn find_idotmatrix(central: &Adapter) -> Vec<Peripheral> {
        let mut devices = Vec::new();
        for p in central.peripherals().await.unwrap() {
            let props = p.properties().await.unwrap().unwrap();
            // println!("{}", props.local_name.unwrap());
            if let Some(name) = props.local_name {
                if name.starts_with(MATRIX_PREFIX) {
                    devices.push(p);
                }
            }
        }

        devices
    }
}
