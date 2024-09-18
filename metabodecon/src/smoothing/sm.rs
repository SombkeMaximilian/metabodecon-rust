pub trait Smoother<Type, const WINDOW_SIZE: usize> {
    fn compute_smoothed(&mut self, values: Vec<Type>) -> Vec<Type>;
}
