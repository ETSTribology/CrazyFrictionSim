pub trait LinearSolver {
    fn solve(&self, A: &Matrix, b: &Vector) -> Vector;
}