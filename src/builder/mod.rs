pub mod grid;

use grid::{Grid, LinkDirections};
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::collections::HashSet;
pub fn aldous_broder(base: &mut Grid) {
    let mut rng = rand::thread_rng();
    let seed: u64 = rng.gen();
    aldous_broder_seeded(base, seed)
}

pub fn aldous_broder_seeded(base: &mut Grid, seed: u64) {
    let mut rng = Pcg64::seed_from_u64(seed);
    let mut cell_list = base.cell_locations();
    let random_index: usize = rng.gen_range(0..cell_list.len() - 1);
    let mut cell = cell_list.remove(random_index);
    let mut visited = HashSet::new();
    visited.insert(cell);
    while !cell_list.is_empty() {
        let mut neighbors = base.neighbors(cell);
        let rindex: u8 = rng.gen();
        let rindex = rindex as usize % neighbors.len();
        let link_neighbor = neighbors.remove(rindex as usize);
        if !visited.contains(&link_neighbor.0) {
            base.link(cell.0, cell.1, link_neighbor.1.clone());
            // add visited
            visited.insert(link_neighbor.0);
            // remove cell

            cell_list.retain(|x| x != &link_neighbor.0);
        }
        //dbg!(&cell_list);
        cell = link_neighbor.0;
    }
}

pub fn sidewinder(base: &mut Grid) {
    let mut rng = rand::thread_rng();
    let seed: u64 = rng.gen();
    sidewinder_seeded(base, seed)
}

pub fn sidewinder_seeded(base: &mut Grid, seed: u64) {
    let mut rng = Pcg64::seed_from_u64(seed);
    for row in 0..base.rows {
        let mut run = vec![];
        for column in 0..base.columns {
            run.push((row, column));
            let far_east = column == base.columns - 1;
            let far_north = row == base.rows - 1;
            let coin: u8 = rng.gen();
            let close_out = far_east || (!far_north && coin % 2 == 0);
            if close_out {
                let member = run.choose(&mut rng).unwrap();
                if member.0 < base.rows - 1 {
                    // is not at the top
                    base.link(member.0, member.1, LinkDirections::North);
                }
                run.clear();
            } else {
                base.link(row, column, LinkDirections::East);
            }
        }
    }
}
