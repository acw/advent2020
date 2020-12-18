pub enum Math {
    Constant(usize),
    Multiply(Box<Math>, Box<Math>),
    Add(Box<Math>, Box<Math>),
}

impl Math {
    pub fn compute(&self) -> usize {
        match self {
            Math::Constant(x) => *x,
            Math::Multiply(a, b) => a.compute() * b.compute(),
            Math::Add(a, b) => a.compute() + b.compute(),
        }
    }
}
