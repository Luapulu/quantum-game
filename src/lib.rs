// use rand::Rng;
// use num_complex::Complex;
// use ndarray::{Array, Array2, Dim};

pub mod lattice;
pub use crate::lattice::LatticeSpec;

pub mod state;

// pub trait QuantumState<T> {

//     const WAVE_FUNCTION_DIM: u32 = 2 * Self::PARTICLE_NUM;

//     // Array<Complex<Self::T>, Self::PARTICLE_NUM>

//     // immutable
//     fn wave_function(self: &Self) -> View;

//     // mutable
//     fn wave_function_mut(self: &mut Self) -> MutView;
// }

// Defines system
// pub trait Hamiltonian {
//     fn time_step(dt, state, &self, t);
// }

// struct OneBodyZeroPotential {}

// struct OneBodyPosPotential {}

// struct OneBodyStaticPosPotential {}

// pub trait Measurement {
//     type State;
//     type Setup;
//     type Observation;

//     fn setup(state: Self::State) -> Self::Setup;

//     fn measure(setup: Self::Setup, rng: &mut impl Rng) -> Self::Observation;

//     fn apply(state: Self::State);
// }

// pub struct OnePosObs {

// }

// pub struct PosMeasurement {

// }

// impl Measurement for PosMeasurement {
//     type State =
// }

pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_two(2), 4);
    }
}
