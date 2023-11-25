pub trait Entity<ID: Eq> {
    fn validate(&self) -> bool;
}
