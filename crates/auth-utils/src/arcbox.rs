use std::sync::Arc;

pub type ArcBox<T> = Arc<Box<T>>;

#[macro_export]
macro_rules! arcbox {
    ($x:ident) => {
        std::sync::Arc::new(Box::new($x))
    };
}
