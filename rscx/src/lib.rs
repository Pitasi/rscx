pub mod context;

pub extern crate rscx_macros;
pub use rscx_macros::*;

pub trait CollectFragment<I>
where
    I: Iterator,
    Vec<String>: FromIterator<<I as Iterator>::Item>,
{
    fn collect_fragment(self) -> String;
}

impl<I> CollectFragment<I> for I
where
    I: Iterator,
    Vec<String>: FromIterator<<I as Iterator>::Item>,
{
    fn collect_fragment(self) -> String {
        self.collect::<Vec<String>>().join("")
    }
}
