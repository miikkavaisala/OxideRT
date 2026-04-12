use rand_distr::{Normal, Distribution};

fn main() {
    let version = "0.1";
    let author = "Miikka Väisälä";

    // Print text to the console.
    println!("Starting Oxide, Version {version}", version = version);
    println!("Simple Rust based radiative transfer rendering tool");
    println!("Author: {name}", name = author);

    // Initialize a 3D array of zeroes.

    const NX: usize = 32;
    const NY: usize = 32;
    const NZ: usize = 32;
    const NGRID: usize = NX * NY * NZ;
    const DX: f64 = 1.0;
    const DY: f64 = 1.0;
    const DZ: f64 = 1.0;

    let mut density_grid: Vec<f64> = Vec::new();
    let mut xx_grid: Vec<f64> = Vec::new();
    let mut yy_grid: Vec<f64> = Vec::new();
    let mut zz_grid: Vec<f64> = Vec::new();
    density_grid.resize(NGRID, 0.0);
    xx_grid.resize(NGRID, 0.0);
    yy_grid.resize(NGRID, 0.0);
    zz_grid.resize(NGRID, 0.0);

    // For loop to set values for coordinate grids
    for k in 0..NZ {
        for j in 0..NY {
            for i in 0..NX {
                let ind = i + j * NX + k * NX * NY;
                xx_grid[ind] = (i as f64) * DX;
                yy_grid[ind] = (j as f64) * DY;
                zz_grid[ind] = (k as f64) * DZ;
            }
        }
    }

    let normal_dist = Normal::new(0.0, 1.0).unwrap(); // Mean = 0.0, Std Dev = 1.0
    let mut rng = rand::thread_rng();

    // For loop to set values for an array
    for k in 0..NZ {
        for j in 0..NY {
            for i in 0..NX {
                let ind = i + j * NX + k * NX * NY;
                density_grid[ind] = normal_dist.sample(&mut rng);
            }
        }
    }

    println!("Grid set successfully.");
}
