mod builder;
fn main() {
    println!("Hello, world!");
    let mut base = builder::grid::Grid::new(30, 40);
    builder::grid::sidewinder(&mut base);
    println!("{}", base);
    //dbg!(base);
}
