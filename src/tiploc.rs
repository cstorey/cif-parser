use std::{borrow::Cow, fmt};

#[derive(Clone, Eq, PartialEq)]
pub struct Tiploc<'a>(Cow<'a, str>);

impl<'a> Tiploc<'a> {
    pub fn of_str(s: &'a str) -> Self {
        Tiploc(s.into())
    }
}
impl Tiploc<'static> {
    pub fn of_string(s: String) -> Self {
        Tiploc(s.into())
    }
}

impl fmt::Debug for Tiploc<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("Tiploc").field(&self.0).finish()
    }
}

impl fmt::Display for Tiploc<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl<'a> From<&'a str> for Tiploc<'a> {
    fn from(tl: &'a str) -> Self {
        Tiploc(tl.into())
    }
}

impl AsRef<str> for Tiploc<'_> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
