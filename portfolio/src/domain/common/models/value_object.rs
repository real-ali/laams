pub trait ValueObject: Eq {
    fn validate(&self) -> bool;
}
