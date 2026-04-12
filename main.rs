use rand::thread_rng;
use rand_distr::{Normal, Distribution};

fn initialize_coordinate_grid(xx_grid: &mut Vec<f64>, yy_grid: &mut Vec<f64>, 
                              zz_grid: &mut Vec<f64>, 
                              nx: usize, ny: usize, nz: usize,  
                              dx: f64, dy: f64, dz: f64) {
    // For loop to set values for coordinate grids
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let ind = i + j * nx + k * nx * ny;
                xx_grid[ind] = (i as f64) * dx;
                yy_grid[ind] = (j as f64) * dy;
                zz_grid[ind] = (k as f64) * dz;
            }
        }
    }
}


fn initialize_density_grid(density_grid: &mut Vec<f64>, nx: usize, ny: usize, nz: usize) {
    let normal_dist = Normal::new(0.0, 1.0).unwrap(); // Mean = 0.0, Std Dev = 1.0
    let mut rng = thread_rng();

    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let ind = i + j * nx + k * nx * ny;
                density_grid[ind] = normal_dist.sample(&mut rng);
            }
        }
    }
}

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

    initialize_coordinate_grid(&mut xx_grid, &mut yy_grid, &mut zz_grid, 
                               NX, NY, NZ, DX, DY, DZ);
    
    // Initialize density_grid with Gaussian random numbers
    initialize_density_grid(&mut density_grid, NX, NY, NZ);

    println!("Grid set successfully.");
}
