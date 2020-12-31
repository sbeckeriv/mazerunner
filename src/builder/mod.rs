pub mod grid;

use grid::{Grid, LinkDirections};
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::collections::HashSet;

pub fn wilsons(base: &mut Grid) {
    let mut rng = rand::thread_rng();
    let seed: u64 = rng.gen();
    wilsions_seeded(base, seed)
}

pub fn wilsions_seeded(base: &mut Grid, seed: u64) {
    let mut rng = Pcg64::seed_from_u64(seed);

    let mut unvisited = base.cell_locations();

    let random_index: usize = rng.gen_range(0..unvisited.len() - 1);
    let first = unvisited.remove(random_index);
    while !unvisited.is_empty() {
        let cell_index: usize = if unvisited.len() > 1 {
            rng.gen_range(0..unvisited.len() - 1)
        } else {
            0
        };
        let mut cell = unvisited[cell_index].clone();
        let mut path = vec![cell.clone()];

        let mut directions: Vec<LinkDirections> = vec![LinkDirections::Other];

        while unvisited.contains(&cell) {
            //    println!("_____{}-{}____", cell.0, cell.1);

            let neighbors = base.neighbors(cell);

            let rindex: u8 = rng.gen();
            let rindex = rindex as usize % neighbors.len();
            let neighbor = neighbors[rindex].clone();
            cell = neighbor.0;
            let position = path.iter().position(|&x| x == cell);
            if let Some(index) = position {
                let index = if index == 0 { 1 } else { index };
                //println!("************");
                path.truncate(index);
                directions.truncate(index);
                cell = path.last().unwrap().clone();
            } else {
                path.push(cell.clone());
                directions.push(neighbor.1.clone());
            }
        }

        if path.len() == 2 {
            base.link(path[0].0, path[0].1, directions[1].clone());
            unvisited.retain(|x| x != &path[0]);
        } else {
            for index in 0..path.len() - 1 {
                base.link(path[index].0, path[index].1, directions[index + 1].clone());
                unvisited.retain(|x| x != &path[index]);
            }
        }
    }
}

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
