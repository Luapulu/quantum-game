use num_complex::Complex;
use std::ops::Index;

use crate::lattice::LatticeSpec;

struct OneBodyState<T> {
    wave_function: Vec<Complex<T>>,
    lattice: LatticeSpec
}

impl<T> OneBodyState<T> {
    fn new_uninit(lattice: LatticeSpec) -> OneBodyState<T> {
        let wave_function = Vec::<Complex<T>>::with_capacity(lattice.nsites());
        OneBodyState{wave_function, lattice}
    }

    pub fn lattice(&self) -> LatticeSpec {self.lattice}

    // fn new_at(site: [usize; 2], lattice: LatticeSpec) {
    //     let mut state = Self::new_uninit(lattice);
    //     let [ix, iy] = site;
    //     let sitei = ix * lattice.nx() + lattice.ny();

    //     // for i in 0:
    //     // state.wave_function
    // }
}

impl<T> Index<[usize; 2]> for OneBodyState<T> {
    type Output = Complex<T>;

    fn index(&self, site: [usize; 2]) -> &Self::Output {
        let [ix, iy] = site;
        let lattice = self.lattice();

        // This guarantees our index is inbounds
        if ix >= lattice.nx() || iy >= lattice.ny() {
            panic!("site {:?} is not on lattice of size {:?}", site, lattice.grid())
        }

        // use row major order with 0 based indexing
        let i = iy * self.lattice().nx() + ix;

        debug_assert!(i < self.lattice().nsites());
        debug_assert!(i < self.wave_function.len());

        unsafe {
            self.wave_function.get_unchecked(i)
        }
    }
}
