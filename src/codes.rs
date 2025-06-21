use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RotatedSurfaceCode {
    pub d: usize,
    pub qubit_positions: Vec<(f64, f64)>,
}

impl RotatedSurfaceCode {
    pub fn new(d: usize) -> Self {
        Self { n, p }
    }
}
