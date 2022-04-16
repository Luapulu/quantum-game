#[derive(Debug, Copy, Clone)]
pub struct LatticeSpec {
    grid: [usize; 2],
    cell: [f64; 2]
}

impl LatticeSpec {
    pub fn new(grid: [usize; 2], cell: [f64; 2]) -> LatticeSpec {
        if grid[0] <= 0 || grid[1] <= 0 {
            panic!("Grid must have at least one site per dimension, got specification {:?}", grid)
        } else if cell[0] <= 0.0 || cell[1] <= 0.0 {
            panic!("cell size must be specified by strictly positive numbers, got {:?}", cell)
        } else {
            LatticeSpec{grid, cell}
        }
    }

    pub fn grid(&self) -> [usize; 2] {self.grid}
    pub fn cell(&self) -> [f64; 2] {self.cell}

    pub fn nx(&self) -> usize {self.grid[0]}
    pub fn ny(&self) -> usize {self.grid[1]}

    pub fn dx(&self) -> f64 {self.cell[0]}
    pub fn dy(&self) -> f64 {self.cell[1]}

    pub fn lx(&self) -> f64 {(self.grid[0] as f64) * self.cell[0]}
    pub fn ly(&self) -> f64 {(self.grid[1] as f64) * self.cell[1]}

    pub fn size(&self) -> [f64; 2] {[self.lx(), self.ly()]}
    pub fn nsites(&self) -> usize {self.grid[0] * self.grid[1]}
}
