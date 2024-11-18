pub trait MovingAverage<Type> {
    fn add_value(&mut self, value: Type);
    fn pop_last(&mut self) -> Option<Type>;
    fn compute_average(&self) -> Type;
    fn clear(&mut self);
}
