mod fem;
mod config;
mod io;
mod linear_solvers;

use config::yaml_config;
use fem::solver::FEMSolver;
use linear_solvers::LinearSolver;

fn main() {
    // Load configuration
    let config = yaml_config::load_config("config.yaml");

    // Initialize MPI
    let universe = io::mpi::initialize_mpi();
    let world = universe.world();
    let rank = world.rank();

    // Initialize FEM solver
    let solver = FEMSolver::new(&config);

    // Choose linear solver based on config
    let linear_solver: Box<dyn LinearSolver> = match config.solver_type.as_str() {
        "AMG" => Box::new(linear_solvers::amg::AMG::new()),
        "Direct" => Box::new(linear_solvers::direct::DirectSolver::new()),
        _ => Box::new(linear_solvers::iterative::IterativeSolver::new()),
    };

    // Run simulation
    solver.run(linear_solver);

    // Connect to Redis if needed
    // io::redis::connect_redis().await;
}