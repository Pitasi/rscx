use std::borrow::Cow;

use html_escape::encode_unquoted_attribute;

pub trait EscapeAttribute {
    fn escape_attribute(&self) -> Cow<str>;
}

impl EscapeAttribute for &str {
    fn escape_attribute(&self) -> Cow<str> {
        encode_unquoted_attribute(self)
    }
}

impl EscapeAttribute for str {
    fn escape_attribute(&self) -> Cow<str> {
        encode_unquoted_attribute(self)
    }
}

impl EscapeAttribute for String {
    fn escape_attribute(&self) -> Cow<str> {
        encode_unquoted_attribute(self)
    }
}

impl EscapeAttribute for &String {
    fn escape_attribute(&self) -> Cow<str> {
        encode_unquoted_attribute(self)
    }
}

macro_rules! impl_escape_attribute_literal {
    ($($t:ty),*) => {
        $(
            impl EscapeAttribute for $t {
                fn escape_attribute(&self) -> Cow<str> {
                    Cow::Owned(self.to_string())
                }
            }
        )*
    };
}

impl_escape_attribute_literal!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool
);
