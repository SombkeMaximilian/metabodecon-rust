pub trait Smoother<Type, Algo, const WINDOW_SIZE: usize> {
    fn new(value: Type) -> Self;
    fn compute_smoothed(&mut self, values: Vec<Type>) -> Vec<Type>;
}
