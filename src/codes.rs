use mwpf::util::{HyperEdge, SolverInitializer};
use mwpf::visualize::VisualizePosition;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCodeInfo {
    pub client_info: ClientCodeInfo,
    pub solver_initializer: SolverInitializer,
    pub edge_errors: Vec<(usize, String)>, // (data_qubit_index, check_type)
    pub visualize_positions: Vec<VisualizePosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCodeInfo {
    pub id: String,
    pub name: String,
    pub d: usize,
    pub data_qubit_positions: Vec<(f64, f64)>,
    pub data_qubit_actions: Vec<HashMap<String, Vec<usize>>>,
    pub stabilizer_positions: Vec<(f64, f64)>,
    pub stabilizer_shapes: Vec<Vec<(f64, f64)>>,
    pub stabilizer_checks: Vec<Vec<(usize, String)>>, // (data_qubit_index, check_type)
    pub stabilizer_colors: Vec<String>,
    pub logical_observables: Vec<Vec<(usize, String)>>, // (data_qubit_index, check_type)
}

impl ClientCodeInfo {
    pub fn construct_graph(&self) -> (SolverInitializer, Vec<(usize, String)>) {
        let vertex_num = self.stabilizer_positions.len();
        // deduplicate hyperedges
        let mut hyperedges: HashMap<BTreeSet<usize>, (usize, String)> = HashMap::new();
        for (data_index, actions) in self.data_qubit_actions.iter().enumerate() {
            for (error_type, syndrome) in actions.iter() {
                let hyperedge = BTreeSet::from_iter(syndrome.iter().cloned());
                hyperedges
                    .entry(hyperedge)
                    .or_insert_with(|| (data_index, error_type.clone()));
            }
        }
        // construct weighted edges
        let mut weighted_edges = vec![];
        let mut edge_errors = vec![];
        for (hyperedge, (data_index, error_type)) in hyperedges.iter() {
            weighted_edges.push(HyperEdge::new(
                hyperedge.iter().cloned().collect(),
                1.0.into(),
            ));
            edge_errors.push((*data_index, error_type.clone()));
        }
        (
            SolverInitializer::new(vertex_num, weighted_edges),
            edge_errors,
        )
    }
}

#[derive(Default, Clone)]
pub enum NoiseType {
    #[default]
    Depolarize,
    BitFlip,
    OnlyY,
}

const RED: &str = "#ffe8e8";
const GREEN: &str = "#e8f5e8";
const BLUE: &str = "#e8f0ff";

impl NoiseType {
    pub fn has_error(&self, error_type: &str) -> bool {
        match self {
            NoiseType::Depolarize => error_type == "X" || error_type == "Y" || error_type == "Z",
            NoiseType::BitFlip => error_type == "X",
            NoiseType::OnlyY => error_type == "Y",
        }
    }
}

impl std::fmt::Display for NoiseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NoiseType::Depolarize => "depolarize".to_string(),
                NoiseType::BitFlip => "bit-flip".to_string(),
                NoiseType::OnlyY => "only-Y".to_string(),
            }
        )
    }
}

impl std::fmt::Debug for NoiseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NoiseType::Depolarize => "Depolarize".to_string(),
                NoiseType::BitFlip => "Bit-Flip".to_string(),
                NoiseType::OnlyY => "Only Y".to_string(),
            }
        )
    }
}

const RSC_SCALE: f64 = 1.0;
const VISUALIZE_SCALE: f64 = 0.5;
const RSC_ROUND_SEGMENTS: usize = 36; // how many points on the round

#[derive(Debug, Default, Clone)]
pub struct RotatedSurfaceCode {
    pub d: usize,
    pub noise_type: NoiseType,
    pub data_qubit_positions: Vec<(usize, usize)>,
    pub position_to_data_qubit: HashMap<(usize, usize), usize>,
    pub stabilizer_positions: Vec<(usize, usize)>,
    pub stabilizer_types: Vec<String>,
    pub position_to_stabilizer: HashMap<(usize, usize), usize>,
    pub data_qubit_actions: Vec<HashMap<String, Vec<usize>>>,
}

impl RotatedSurfaceCode {
    pub fn new(d: usize, noise_type: NoiseType) -> Self {
        let mut code = Self {
            d,
            noise_type,
            ..Default::default()
        };
        code.init_data_qubit_positions();
        code.init_stabilizer_positions();
        code.init_data_qubit_actions();
        code
    }

    pub fn is_qubit(&self, i: usize, j: usize) -> bool {
        if i > 2 * self.d || j > 2 * self.d {
            return false;
        }
        if i == 0 {
            return j > 0 && j < 2 * self.d && j % 4 == 0;
        }
        if i == 2 * self.d {
            return j > 0 && j < 2 * self.d && j % 4 == 2;
        }
        if j == 0 {
            return i % 4 == 2;
        }
        if j == 2 * self.d {
            return i % 4 == 0;
        }
        (i + j) % 2 == 0
    }

    pub fn is_data_qubit(&self, i: usize, j: usize) -> bool {
        self.is_qubit(i, j) && i % 2 == 1
    }

    pub fn is_stabilizer(&self, i: usize, j: usize) -> bool {
        let is_stabilizer = self.is_qubit(i, j) && i % 2 == 0;
        if matches!(self.noise_type, NoiseType::BitFlip) {
            // only add Z stabilizers for bit-flip noise
            return is_stabilizer && (i + j) % 4 == 0;
        }
        is_stabilizer
    }

    pub fn is_z_stabilizer(&self, i: usize, j: usize) -> bool {
        self.is_stabilizer(i, j) && (i + j) % 4 == 0
    }

    pub fn is_x_stabilizer(&self, i: usize, j: usize) -> bool {
        self.is_stabilizer(i, j) && (i + j) % 4 == 2
    }

    fn init_data_qubit_positions(&mut self) {
        for i in 0..(2 * self.d + 1) {
            for j in 0..(2 * self.d + 1) {
                if self.is_data_qubit(i, j) {
                    self.position_to_data_qubit
                        .insert((i, j), self.data_qubit_positions.len());
                    self.data_qubit_positions.push((i, j));
                }
            }
        }
    }

    fn init_stabilizer_positions(&mut self) {
        for i in 0..(2 * self.d + 1) {
            for j in 0..(2 * self.d + 1) {
                if self.is_stabilizer(i, j) {
                    self.position_to_stabilizer
                        .insert((i, j), self.stabilizer_positions.len());
                    self.stabilizer_positions.push((i, j));
                    self.stabilizer_types.push(if self.is_z_stabilizer(i, j) {
                        "Z".to_string()
                    } else {
                        "X".to_string()
                    });
                }
            }
        }
    }

    fn init_data_qubit_actions(&mut self) {
        for (i, j) in self.data_qubit_positions.iter().cloned() {
            let mut actions = HashMap::new();
            for error_type in ["X", "Y", "Z"] {
                if !self.noise_type.has_error(error_type) {
                    continue;
                }
                let mut flipped_stabilizers = vec![];
                for (di, dj) in [(-1, -1), (-1, 1), (1, 1), (1, -1)] {
                    let i2 = (i as isize + di) as usize;
                    let j2 = (j as isize + dj) as usize;
                    if self.is_stabilizer(i2, j2) {
                        let stabilizer_idx = self.position_to_stabilizer[&(i2, j2)];
                        if self.stabilizer_types[stabilizer_idx] != error_type {
                            flipped_stabilizers.push(stabilizer_idx);
                        }
                    }
                }
                actions.insert(error_type.to_string(), flipped_stabilizers);
            }
            self.data_qubit_actions.push(actions);
        }
    }

    fn stabilizer_checks(&self) -> Vec<Vec<(usize, String)>> {
        let mut checks = vec![];
        for (stabilizer_idx, (i, j)) in self.stabilizer_positions.iter().cloned().enumerate() {
            let mut check = vec![];
            for (di, dj) in [(-1, -1), (-1, 1), (1, 1), (1, -1)] {
                if (i as isize + di) < 0 || (j as isize + dj) < 0 {
                    continue;
                }
                let i2 = (i as isize + di) as usize;
                let j2 = (j as isize + dj) as usize;
                if self.is_data_qubit(i2, j2) {
                    check.push((
                        self.position_to_data_qubit[&(i2, j2)],
                        self.stabilizer_types[stabilizer_idx].clone(),
                    ));
                }
            }
            checks.push(check);
        }
        checks
    }

    fn data_qubit_f64_position(&self, data_index: usize) -> (f64, f64) {
        let (i, j) = self.data_qubit_positions[data_index];
        (i as f64 * RSC_SCALE, j as f64 * RSC_SCALE)
    }

    fn stabilizer_f64_position(&self, stabilizer_index: usize) -> (f64, f64) {
        let (i, j) = self.stabilizer_positions[stabilizer_index];
        (i as f64 * RSC_SCALE, j as f64 * RSC_SCALE)
    }

    fn stabilizer_shapes(&self) -> Vec<Vec<(f64, f64)>> {
        let mut shapes = vec![];
        let checks = self.stabilizer_checks();
        for (stabilizer_idx, (i, j)) in self.stabilizer_positions.iter().cloned().enumerate() {
            let mut shape = vec![];
            let check = &checks[stabilizer_idx];
            if i > 0 && j > 0 && i < 2 * self.d && j < 2 * self.d {
                // regular rectangle shape
                assert_eq!(check.len(), 4);
                for &(data_index, _) in check.iter() {
                    shape.push(self.data_qubit_f64_position(data_index));
                }
            } else {
                // draw a half-circle
                let start_theta = match (i == 0, j == 0, i == 2 * self.d, j == 2 * self.d) {
                    (true, _, _, _) => 0.0,
                    (_, true, _, _) => std::f64::consts::PI / 2.0,
                    (_, _, true, _) => std::f64::consts::PI,
                    (_, _, _, true) => 3.0 * std::f64::consts::PI / 2.0,
                    _ => unreachable!(),
                };
                assert_eq!(check.len(), 2);
                let center_i = (self.data_qubit_f64_position(check[0].0).0
                    + self.data_qubit_f64_position(check[1].0).0)
                    / 2.0;
                let center_j = (self.data_qubit_f64_position(check[0].0).1
                    + self.data_qubit_f64_position(check[1].0).1)
                    / 2.0;
                let radius = RSC_SCALE;
                for theta in (0..RSC_ROUND_SEGMENTS).map(|i| {
                    start_theta + std::f64::consts::PI * i as f64 / (RSC_ROUND_SEGMENTS - 1) as f64
                }) {
                    shape.push((
                        center_i - radius * theta.sin(),
                        center_j + radius * theta.cos(),
                    ));
                }
            }
            shapes.push(shape);
        }
        shapes
    }

    fn stabilizer_colors(&self) -> Vec<String> {
        let mut colors = vec![];
        for (i, j) in self.stabilizer_positions.iter().cloned() {
            colors.push(if self.is_z_stabilizer(i, j) {
                GREEN.to_string()
            } else {
                BLUE.to_string()
            });
        }
        colors
    }
}

impl From<&RotatedSurfaceCode> for ServerCodeInfo {
    fn from(code: &RotatedSurfaceCode) -> Self {
        // Z logical observables
        let mut z_observable = vec![];
        for i in 0..(2 * code.d + 1) {
            if code.is_data_qubit(i, 1) {
                z_observable.push((code.position_to_data_qubit[&(i, 1)], "Z".to_string()));
            }
        }
        // X logical observables
        let mut x_observable: Vec<(usize, String)> = vec![];
        for j in 0..(2 * code.d + 1) {
            if code.is_data_qubit(j, 1) {
                x_observable.push((code.position_to_data_qubit[&(1, j)], "X".to_string()));
            }
        }
        let noise_str = match code.noise_type {
            NoiseType::Depolarize => "".to_string(),
            NoiseType::BitFlip => "Bit-Flip, ".to_string(),
            NoiseType::OnlyY => "Y-Flip, ".to_string(),
        };
        let client_info = ClientCodeInfo {
            id: format!("rsc-{}-d-{}", code.noise_type, code.d),
            name: format!("Surface Code ({}d={})", noise_str, code.d),
            d: code.d,
            data_qubit_positions: (0..code.data_qubit_positions.len())
                .map(|data_idx| code.data_qubit_f64_position(data_idx))
                .collect(),
            stabilizer_positions: (0..code.stabilizer_positions.len())
                .map(|stabilizer_idx| code.stabilizer_f64_position(stabilizer_idx))
                .collect(),
            stabilizer_shapes: code.stabilizer_shapes(),
            stabilizer_checks: code.stabilizer_checks(),
            stabilizer_colors: code.stabilizer_colors(),
            data_qubit_actions: code.data_qubit_actions.clone(),
            logical_observables: vec![z_observable, x_observable],
        };
        let (solver_initializer, edge_errors) = client_info.construct_graph();
        let visualize_positions = client_info
            .stabilizer_positions
            .iter()
            .map(|(i, j)| VisualizePosition::new(*i * VISUALIZE_SCALE, *j * VISUALIZE_SCALE, 0.0))
            .collect();
        Self {
            client_info,
            edge_errors,
            solver_initializer,
            visualize_positions,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct TriangularColorCodeBitFlip {
    pub d: usize,
    pub data_qubit_positions: Vec<(usize, usize)>,
    pub position_to_data_qubit: HashMap<(usize, usize), usize>,
    pub stabilizer_positions: Vec<(usize, usize)>,
    pub stabilizer_types: Vec<String>,
    pub position_to_stabilizer: HashMap<(usize, usize), usize>,
    pub data_qubit_actions: Vec<HashMap<String, Vec<usize>>>,
}

impl TriangularColorCodeBitFlip {
    pub fn new(d: usize) -> Self {
        let mut code = Self {
            d,
            ..Default::default()
        };
        code.init_data_qubit_positions();
        code.init_stabilizer_positions();
        code.init_data_qubit_actions();
        code
    }

    pub fn rows(&self) -> usize {
        3 * (self.d - 1) / 2 + 1
    }

    pub fn columns(&self) -> usize {
        2 * self.d - 1
    }

    pub fn range_for_row(&self, r: usize) -> std::ops::Range<usize> {
        let cr = r / 3;
        let start = 2 * cr + (if r % 3 == 0 { 0 } else { 1 });
        let length = self.columns() - 4 * cr - (r % 3);
        start..(start + length)
    }

    pub fn is_qubit(&self, i: usize, j: usize) -> bool {
        if i >= self.rows() {
            return false;
        }
        self.range_for_row(i).contains(&j)
    }

    pub fn is_data_qubit(&self, i: usize, j: usize) -> bool {
        if !self.is_qubit(i, j) {
            return false;
        }
        if i % 2 == 0 {
            (j % 4) == 0 || (j % 4) == 3
        } else {
            (j % 4) == 1 || (j % 4) == 2
        }
    }

    pub fn is_z_stabilizer(&self, i: usize, j: usize) -> bool {
        if !self.is_qubit(i, j) {
            return false;
        }
        if i % 2 == 0 {
            j % 4 == 1
        } else {
            j % 4 == 3
        }
    }

    fn init_data_qubit_positions(&mut self) {
        for i in 0..self.rows() {
            for j in self.range_for_row(i) {
                if self.is_data_qubit(i, j) {
                    self.position_to_data_qubit
                        .insert((i, j), self.data_qubit_positions.len());
                    self.data_qubit_positions.push((i, j));
                }
            }
        }
    }

    fn init_stabilizer_positions(&mut self) {
        for i in 0..self.rows() {
            for j in self.range_for_row(i) {
                if self.is_z_stabilizer(i, j) {
                    self.position_to_stabilizer
                        .insert((i, j), self.stabilizer_positions.len());
                    self.stabilizer_positions.push((i, j));
                    self.stabilizer_types.push("Z".to_string());
                }
            }
        }
    }

    fn init_data_qubit_actions(&mut self) {
        for (i, j) in self.data_qubit_positions.iter().cloned() {
            let mut actions = HashMap::new();
            let error_type = "X";
            let mut flipped_stabilizers = vec![];
            for (di, dj) in [(0, 1), (0, -2), (-1, 0), (1, 0), (-1, -1), (1, -1)] {
                let i2 = (i as isize + di) as usize;
                let j2 = (j as isize + dj) as usize;
                if self.is_z_stabilizer(i2, j2) {
                    let stabilizer_idx = self.position_to_stabilizer[&(i2, j2)];
                    if self.stabilizer_types[stabilizer_idx] != error_type {
                        flipped_stabilizers.push(stabilizer_idx);
                    }
                }
            }
            actions.insert(error_type.to_string(), flipped_stabilizers);
            self.data_qubit_actions.push(actions);
        }
    }

    fn stabilizer_checks(&self) -> Vec<Vec<(usize, String)>> {
        let mut checks = vec![];
        for (stabilizer_idx, (i, j)) in self.stabilizer_positions.iter().cloned().enumerate() {
            let mut check = vec![];
            for (di, dj) in [(-1, 0), (-1, 1), (0, 2), (1, 1), (1, 0), (0, -1)] {
                let i2 = (i as isize + di) as usize;
                let j2 = (j as isize + dj) as usize;
                if self.is_data_qubit(i2, j2) {
                    check.push((
                        self.position_to_data_qubit[&(i2, j2)],
                        self.stabilizer_types[stabilizer_idx].clone(),
                    ));
                }
            }
            checks.push(check);
        }
        checks
    }

    fn qubit_f64_position(&self, i: usize, j: usize) -> (f64, f64) {
        let row_j = match i % 3 {
            0 => {
                let idx = j - (i / 3) * 2 + 1;
                idx / 4 * 3 + idx % 4 - 1
            }
            1 => {
                let idx = j - 1 - (i / 3) * 2;
                idx / 4 * 3 + idx % 4
            }
            2 => {
                let idx = j + 1 - (i / 3) * 2;
                idx / 4 * 3 + idx % 4 - 2
            }
            _ => unreachable!(),
        };
        let interval = 2f64.sqrt() * RSC_SCALE;
        let row_bias = match i % 3 {
            0 => 0.0,
            1 => 0.5,
            2 => 1.0,
            _ => unreachable!(),
        } + (i / 3) as f64 * 1.5;
        let j_f64 = interval * (row_bias + row_j as f64);
        let i_f64 = i as f64 * interval / 2.0 * 3f64.sqrt();
        (-i_f64, j_f64)
    }

    fn data_qubit_f64_position(&self, data_index: usize) -> (f64, f64) {
        let (i, j) = self.data_qubit_positions[data_index];
        self.qubit_f64_position(i, j)
    }

    fn stabilizer_f64_position(&self, stabilizer_index: usize) -> (f64, f64) {
        let (i, j) = self.stabilizer_positions[stabilizer_index];
        self.qubit_f64_position(i, j)
    }

    fn stabilizer_shapes(&self) -> Vec<Vec<(f64, f64)>> {
        let mut shapes = vec![];
        let checks = self.stabilizer_checks();
        for check in checks.iter() {
            let mut shape = vec![];
            for &(data_index, _) in check.iter() {
                shape.push(self.data_qubit_f64_position(data_index));
            }
            shapes.push(shape);
        }
        shapes
    }

    fn stabilizer_colors(&self) -> Vec<String> {
        let mut colors = vec![];
        for (i, _j) in self.stabilizer_positions.iter().cloned() {
            colors.push(if i % 3 == 0 {
                RED.to_string()
            } else if i % 3 == 1 {
                GREEN.to_string()
            } else {
                BLUE.to_string()
            });
        }
        colors
    }
}

impl From<&TriangularColorCodeBitFlip> for ServerCodeInfo {
    fn from(code: &TriangularColorCodeBitFlip) -> Self {
        // Z logical observables
        let mut z_observable = vec![];
        for j in code.range_for_row(0) {
            if code.is_data_qubit(0, j) {
                z_observable.push((code.position_to_data_qubit[&(0, j)], "Z".to_string()));
            }
        }
        let client_info = ClientCodeInfo {
            id: format!("color-d-{}", code.d),
            name: format!("Color Code (Bit-Flip, d={})", code.d),
            d: code.d,
            data_qubit_positions: (0..code.data_qubit_positions.len())
                .map(|data_idx| code.data_qubit_f64_position(data_idx))
                .collect(),
            stabilizer_positions: (0..code.stabilizer_positions.len())
                .map(|stabilizer_idx| code.stabilizer_f64_position(stabilizer_idx))
                .collect(),
            stabilizer_shapes: code.stabilizer_shapes(),
            stabilizer_checks: code.stabilizer_checks(),
            stabilizer_colors: code.stabilizer_colors(),
            data_qubit_actions: code.data_qubit_actions.clone(),
            logical_observables: vec![z_observable],
        };
        let (solver_initializer, edge_errors) = client_info.construct_graph();
        let visualize_positions = client_info
            .stabilizer_positions
            .iter()
            .map(|(i, j)| VisualizePosition::new(*i * VISUALIZE_SCALE, *j * VISUALIZE_SCALE, 0.0))
            .collect();
        Self {
            client_info,
            edge_errors,
            solver_initializer,
            visualize_positions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotated_surface_code() {
        // cargo test -- test_rotated_surface_code --nocapture
        let code = RotatedSurfaceCode::new(3, NoiseType::Depolarize);
        println!("{:?}\n{:?}\n\n", code, ServerCodeInfo::from(&code));
        println!(
            "{}\n",
            serde_json::to_string(&ServerCodeInfo::from(&code)).unwrap()
        );
        // let code = RotatedSurfaceCode::new(3, NoiseType::BitFlip);
        // println!("{:?}\n{:?}\n\n", code, ServerCodeInfo::from(code));
        // let code = RotatedSurfaceCode::new(3, NoiseType::OnlyY);
        // println!("{:?}\n{:?}\n\n", code, ServerCodeInfo::from(code));
    }

    #[test]
    fn test_triangular_color_code_bit_flip() {
        // cargo test -- test_triangular_color_code_bit_flip --nocapture
        let code = TriangularColorCodeBitFlip::new(3);
        println!("{:?}\n{:?}\n\n", code, ServerCodeInfo::from(&code));
        println!(
            "{}\n",
            serde_json::to_string(&ServerCodeInfo::from(&code)).unwrap()
        );
    }
}
