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

fn edge_sum_xy(colden_slice_xy: &mut Vec<f64>, density_grid: &Vec<f64>, nx: usize, ny: usize, nz: usize) {
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let ind_slice = i + j * nx;
                let ind = i + j * nx + k * nx * ny;
                colden_slice_xy[ind_slice] = colden_slice_xy[ind_slice] + density_grid[ind];
            }
        }
    }
}

fn edge_sum_xz(colden_slice_xz: &mut Vec<f64>, density_grid: &Vec<f64>, nx: usize, ny: usize, nz: usize) {
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let ind_slice = i + k * nx;
                let ind = i + j * nx + k * nx * ny;
                colden_slice_xz[ind_slice] = colden_slice_xz[ind_slice] + density_grid[ind];
            }
        }
    }
}

fn edge_sum_yz(colden_slice_yz: &mut Vec<f64>, density_grid: &Vec<f64>, nx: usize, ny: usize, nz: usize) {
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let ind_slice = j + k * ny;
                let ind = i + j * nx + k * nx * ny;
                colden_slice_yz[ind_slice] = colden_slice_yz[ind_slice] + density_grid[ind];
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

    // Grid dimensions
    const NX: usize = 32;
    const NY: usize = 32;
    const NZ: usize = 32;
    const NGRID: usize = NX * NY * NZ;
    const NSLICEXY: usize = NX * NY;
    const NSLICEXZ: usize = NX * NZ;
    const NSLICEYZ: usize = NY * NZ;
    
    // Physical cell spacing
    const DX: f64 = 1.0;
    const DY: f64 = 1.0;
    const DZ: f64 = 1.0;
    
    // Volume grids
    let mut density_grid = vec![0.0_f64; NGRID];
    let mut xx_grid      = vec![0.0_f64; NGRID];
    let mut yy_grid      = vec![0.0_f64; NGRID];
    let mut zz_grid      = vec![0.0_f64; NGRID];
    
    // Column density projection slices
    let mut colden_slice_xy = vec![0.0_f64; NSLICEXY];
    let mut colden_slice_xz = vec![0.0_f64; NSLICEXZ];
    let mut colden_slice_yz = vec![0.0_f64; NSLICEYZ];

    initialize_coordinate_grid(&mut xx_grid, &mut yy_grid, &mut zz_grid, 
                               NX, NY, NZ, DX, DY, DZ);
    
    // Initialize density_grid with Gaussian random numbers
    initialize_density_grid(&mut density_grid, NX, NY, NZ);

    println!("Grid set successfully.");

    // Calculate simple sum for edge wise column density. 
    edge_sum_xy(&mut colden_slice_xy, &density_grid, NX, NY, NZ);
    edge_sum_xz(&mut colden_slice_xz, &density_grid, NX, NY, NZ);
    edge_sum_yz(&mut colden_slice_yz, &density_grid, NX, NY, NZ);

    println!("Edge sum column density calculated successfully.");
}
