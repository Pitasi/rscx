use std::{fmt, convert::Infallible, env::VarError, io::ErrorKind, net::{IpAddr, SocketAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6}, sync::{mpsc::{RecvTimeoutError, TryRecvError}, Arc}, num::{NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize}, borrow::Cow, ops::Deref, rc::Rc};

pub trait Renderable {
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

macro_rules! impl_renderable_for_basic_types {
    ($($t:ty)*) => ($(
        impl Renderable for $t {
            fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self)
            }
        }
    )*)
}

impl_renderable_for_basic_types! {
    Infallible
    VarError
    ErrorKind
    IpAddr
    SocketAddr
    RecvTimeoutError
    TryRecvError
    bool
    char
    f32
    f64
    i8
    i16
    i32
    i64
    i128
    isize
    u8
    u16
    u32
    u64
    u128
    usize
    Ipv4Addr
    Ipv6Addr
    SocketAddrV4
    SocketAddrV6
    NonZeroI8
    NonZeroI16
    NonZeroI32
    NonZeroI64
    NonZeroI128
    NonZeroIsize
    NonZeroU8
    NonZeroU16
    NonZeroU32
    NonZeroU64
    NonZeroU128
    NonZeroUsize
    String
    &str
    str
}

impl<T: Renderable + ToOwned + ?Sized> Renderable  for Cow<'_, T>  {
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().render(f)
    }
}

impl<T: Renderable> Renderable for Box<T> {
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().render(f)
    }
}

impl<T: Renderable> Renderable for Vec<T> {
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter().map(|x| x.render(f)).collect::<fmt::Result>()
    }
}

impl<T: Renderable> Renderable for [T] {
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter().map(|x| x.render(f)).collect::<fmt::Result>()
    }
}

impl<T: Renderable> Renderable for &T {
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).render(f)
    }
}

impl<T: Renderable> Renderable for &mut T {
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).render(f)
    }
}

impl<T: Renderable> Renderable for Rc<T> {
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().render(f)
    }
}

impl<T: Renderable> Renderable for Arc<T> {
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().render(f)
    }
}

impl<T: Renderable> Renderable for Option<T> {
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(inner) = self {
            inner.render(f)
        } else {
            Ok(())
        }
    }
}

impl Renderable for () {
    fn render(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
