use std::str::FromStr;

use uuid::Uuid;

pub const MATRIX_PREFIX: &str = "IDM-";
pub const MATRIX_WRITE: Uuid = Uuid::from_u128(0x0000fa02_0000_1000_8000_00805f9b34fb);
