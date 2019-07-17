/// Trait for types which represent something which has a distribution holding an expected value.
pub trait ExpectedValue {
    fn expected(&self) -> f64;
}
