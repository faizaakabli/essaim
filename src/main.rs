extern crate noise;
extern crate rand; // Importation de la bibliothèque rand

use noise::{Perlin, NoiseFn};
use rand::{SeedableRng, RngCore}; // Importation de SeedableRng et RngCore

fn main() {
    let width = 10;
    let height = 10;
    let seed = 42;
    let map = generate_obstacles(width, height, seed);
    display_map(&map);
}


struct Cell {
    obstacle: bool,
    energy: u32,
    minerals: u32,
    scientific_interest: bool,
}

struct Map {
    cells: Vec<Vec<Cell>>,
}

fn generate_obstacles(width: usize, height: usize, seed: u32) -> Vec<Vec<bool>> {
    let perlin = Perlin::new();
    let seed_bytes = seed.to_be_bytes();
    let mut seed_array = [0; 32]; // Initialisation d'un tableau de 32 éléments avec des zéros

    // Copie des 4 premiers bytes de seed_bytes dans seed_array
    seed_array[..4].copy_from_slice(&seed_bytes);

    let mut rng = rand::rngs::StdRng::from_seed(seed_array);

    let mut obstacles = Vec::with_capacity(height);
    for y in 0..height {
        let mut row = Vec::with_capacity(width);
        for x in 0..width {
            let noise_value = perlin.get([x as f64 / 10.0, y as f64 / 10.0, 0.0]);
            let obstacle_probability = (noise_value + 1.0) / 2.0; // Valeur entre 0 et 1
            let obstacle = (rng.next_u64() as f64 / std::u64::MAX as f64) > obstacle_probability;
            row.push(obstacle);
        }
        obstacles.push(row);
    }
    obstacles
}

fn display_map(map: &Vec<Vec<bool>>) {
    for row in map {
        for &cell in row {
            if cell {
                print!("#"); // Caractère représentant un obstacle
            } else {
                print!("."); // Caractère représentant une zone libre
            }
        }
        println!(); // Aller à la ligne après chaque ligne de la carte
    }
}


