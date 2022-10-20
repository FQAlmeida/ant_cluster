pub trait Object {
    fn is_empty(&self) -> bool;
    fn clone_empty() -> Self;
    fn get_distance(&self, other_obj: &Self) -> f64;
}
