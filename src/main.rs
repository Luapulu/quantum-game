use num_complex::Complex64;

fn main() {
    let c1 = Complex64::new(3.0, 4.5);
    let c2 = Complex64::new(1.5, 2.3);
    let v = vec![c1, c2];
    println!("{:?}", v);
}
