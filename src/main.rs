fn main() {
    let static_var:&'static str = "Hello";
    print!("{}", static_var);
    let a: &str = static_var;
    print!("{}", static_var);
}
