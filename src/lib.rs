pub mod impls {
    include!("./generated.rs");
}

pub trait ToClonedClosureNoArgs: Sized {
    fn with_cloned<R, B>(&self, builder: B) -> Box<dyn (Fn() -> R)>
    where
        B: (Fn(Self) -> R) + Copy + 'static;
}
