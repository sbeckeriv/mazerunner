mod builder;
fn main() {
    println!("Hello, world!");
    let mut base = builder::grid::Grid::new(30, 40);
    builder::sidewinder_seeded(&mut base, 2);
    println!("{}", base);
    let mut base = builder::grid::Grid::new(30, 40);
    builder::aldous_broder_seeded(&mut base, 2);
    println!("{}", base);

    //dbg!(base);
}
