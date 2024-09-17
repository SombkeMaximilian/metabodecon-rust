pub trait MovingAverage<Type, const WINDOW_SIZE: usize> {
    fn new(value: Type) -> Self;
    fn add_value(&mut self, value: Type);
    fn compute_average(&self) -> Type;
}
