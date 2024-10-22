use mpi::traits::*;
use mpi::environment::Universe;

pub fn initialize_mpi() -> Universe {
    let universe = mpi::initialize().unwrap();
    universe
}

// In main.rs
mod io;
use io::mpi::initialize_mpi;

fn main() {
    let universe = initialize_mpi();
    let world = universe.world();
    let rank = world.rank();
    // MPI code here
}