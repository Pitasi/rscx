pub trait Component<P> {}

impl<P, F, R> Component<P> for F
where
    F: FnOnce(P) -> R,
    P: Props,
{
}

pub fn props_builder<C, P>(_: C) -> P::Builder
where
    C: Component<P>,
    P: Props,
{
    P::builder()
}

pub trait Props {
    type Builder;
    fn builder() -> Self::Builder;
}
