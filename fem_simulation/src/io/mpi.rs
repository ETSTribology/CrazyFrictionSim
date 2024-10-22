use mpi::traits::*;
use mpi::environment::Universe;

pub fn initialize_mpi() -> Universe {
    let universe = mpi::initialize().unwrap();
    universe
}
