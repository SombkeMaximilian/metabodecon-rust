pub trait MovingAverage<Type, const WINDOW_SIZE: usize> {
    fn new(value: Type) -> Self;
    fn add_value(&mut self, value: Type);
    fn pop_last(&mut self) -> Option<Type>;
    fn compute_average(&self) -> Type;
    fn clear(&mut self);
}
