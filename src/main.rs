fn main() {
    let mut greet = String::from("Hello ");
    let name = "Philip";
    greet.push_str(name);
    println!("{}", greet);
}
