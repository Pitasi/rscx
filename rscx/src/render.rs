use std::{
    borrow::Cow,
    convert::Infallible,
    env::VarError,
    fmt,
    io::ErrorKind,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    rc::Rc,
    sync::{
        mpsc::{RecvTimeoutError, TryRecvError},
        Arc,
    },
};

pub trait Render {
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

macro_rules! impl_render_for_basic_types {
    ($($t:ty)*) => ($(
        impl Render for $t {
            #[inline(always)]
            fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(self, f)
            }
        }
    )*)
}

impl_render_for_basic_types! {
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
    str
}

impl<B: ?Sized> Render for Cow<'_, B>
where
    B: Render + ToOwned,
    <B as ToOwned>::Owned: Render,
{
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Cow::Borrowed(ref b) => Render::render(b, f),
            Cow::Owned(ref o) => Render::render(o, f),
        }
    }
}

impl<T: Render> Render for Box<T> {
    #[inline(always)]
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Render::render(&**self, f)
    }
}

impl<T: Render + ?Sized> Render for &T {
    #[inline(always)]
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Render::render(&**self, f)
    }
}

impl<T: Render> Render for &mut T {
    #[inline(always)]
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Render::render(&**self, f)
    }
}

impl<T: Render> Render for Rc<T> {
    #[inline(always)]
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Render::render(&**self, f)
    }
}

impl<T: Render> Render for Arc<T> {
    #[inline(always)]
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Render::render(&**self, f)
    }
}

impl<T: Render> Render for Option<T> {
    #[inline(always)]
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(inner) = self {
            inner.render(f)
        } else {
            Ok(())
        }
    }
}

impl Render for () {
    #[inline(always)]
    fn render(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
