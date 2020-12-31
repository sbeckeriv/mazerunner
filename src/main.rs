mod builder;
fn main() {
    println!("Hello, world!");
    let mut base = builder::grid::Grid::new(30, 40);
    builder::sidewinder_seeded(&mut base, 2);
    println!("Sidewinder\n{}", base);
    let mut base = builder::grid::Grid::new(30, 40);
    builder::aldous_broder_seeded(&mut base, 2);
    println!("Aldous Broder\n{}", base);

    let mut base = builder::grid::Grid::new(30, 40);
    builder::wilsions_seeded(&mut base, 2);
    println!("Wilsions:\n{}", base);

    //dbg!(base);
}
