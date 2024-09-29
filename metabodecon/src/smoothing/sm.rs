pub trait Smoother<Type> {
    fn compute_smoothed(&mut self, values: &[Type]) -> Vec<Type>;
}
