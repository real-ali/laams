use super::entity::Entity;

pub trait Aggregate<ID: Eq>: Entity<ID> {
    fn validate(&self) -> bool;
}
