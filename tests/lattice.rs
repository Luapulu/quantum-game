use quantum_game::LatticeSpec;

#[test]
fn lattice() {
    let lat = LatticeSpec::new([5, 7], [0.2, 2.5]);

    assert_eq!(lat.grid(), [5, 7]);
    assert_eq!(lat.cell(), [0.2, 2.5]);
    assert_eq!(lat.size(), [1.0, 17.5]);
    assert_eq!(lat.nsites(), 35);

    assert_eq!(lat.nx(), 5);
    assert_eq!(lat.ny(), 7);

    assert_eq!(lat.dx(), 0.2);
    assert_eq!(lat.dy(), 2.5);

    assert_eq!(lat.lx(), 1.0);
    assert_eq!(lat.ly(), 17.5);

    assert_eq!(LatticeSpec::new([1, 1], [0.1, 0.2]).grid(), [1, 1]);
    assert_eq!(LatticeSpec::new([1, 2], [1e-200, 1e-200]).cell(), [1e-200, 1e-200]);
}

#[test]
#[should_panic]
fn bad_grid1() {LatticeSpec::new([0, 7], [0.1, 0.02]);}

#[test]
#[should_panic]
fn bad_grid2() {LatticeSpec::new([3, 0], [5.1, 1.123]);}

#[test]
#[should_panic]
fn bad_cell1() {LatticeSpec::new([2, 3], [0.0, 1.123]);}

#[test]
#[should_panic]
fn bad_cell2() {LatticeSpec::new([2, 1], [3.21, -0.5]);}
