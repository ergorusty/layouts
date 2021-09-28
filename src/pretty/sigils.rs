use crate::pretty::{scalars::Whitespace, string};
use getset::{CopyGetters, Getters};

#[derive(Debug, Copy, Clone)]
pub struct Sep {
    pub(crate) separator: string,
    pub(crate) space_before: Option<Whitespace>,
    pub(crate) space_after: Option<Whitespace>,
}

pub trait Separator: Copy + 'static {
    fn sep(self) -> Sep;
}

impl Separator for Sep {
    fn sep(self) -> Sep {
        self
    }
}

impl Separator for string {
    fn sep(self) -> Sep {
        Sep {
            separator: self,
            space_before: None,
            space_after: None,
        }
    }
}

impl Separator for (Whitespace, string) {
    fn sep(self) -> Sep {
        Sep {
            separator: self.1,
            space_before: Some(self.0),
            space_after: None,
        }
    }
}

impl Separator for (string, Whitespace) {
    fn sep(self) -> Sep {
        Sep {
            separator: self.0,
            space_before: None,
            space_after: Some(self.1),
        }
    }
}

impl Separator for (Whitespace, string, Whitespace) {
    fn sep(self) -> Sep {
        Sep {
            separator: self.1,
            space_before: Some(self.0),
            space_after: Some(self.2),
        }
    }
}

pub trait Wrap: Copy + 'static {
    fn wrapper(self) -> (string, string);
}

impl Wrap for (string, string) {
    fn wrapper(self) -> (string, string) {
        self
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StdWrappers {
    #[allow(unused)]
    Brackets,
    Curlies,
    Parens,
}

impl Wrap for StdWrappers {
    fn wrapper(self) -> (string, string) {
        match self {
            StdWrappers::Brackets => ("[", "]"),
            StdWrappers::Curlies => ("{", "}"),
            StdWrappers::Parens => ("(", ")"),
        }
    }
}

#[derive(Debug, Getters, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct MapSigils {
    pub(crate) wrap: (string, string),
    pub(crate) sep: Sep,
}

impl<W, S> Into<MapSigils> for (W, S)
where
    W: Wrap,
    S: Separator,
{
    fn into(self) -> MapSigils {
        MapSigils::new(self.0, self.1)
    }
}

impl MapSigils {
    pub fn new(wrap: impl Wrap, sep: impl Separator) -> MapSigils {
        MapSigils {
            wrap: wrap.wrapper(),
            sep: sep.sep(),
        }
    }
}
