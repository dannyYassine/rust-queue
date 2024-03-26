pub trait Resolvable {
    fn resolve() -> Self;
}

pub fn resolve<S>() -> S
where
    S: Resolvable,
{
    S::resolve()
}
