pub trait Object {
    fn is_empty(&self) -> bool;
    fn clone_empty() -> Self;
}
