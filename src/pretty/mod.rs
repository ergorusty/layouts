mod internal_macros;

mod containers;
mod doc;
mod iterator;
mod nesting;
mod part;
mod part_trait;
mod scalars;
mod sigils;
mod tycho;

pub use part::Part;
pub use part_trait::PartTrait;

use crate::pretty::doc::Doc;

#[allow(non_camel_case_types)]
pub type string = &'static str;

pub struct TychoElement(pub ::tycho::Element);

impl TychoElement {
    pub fn into_doc(self) -> Doc {
        Part::of(self.0).into_doc()
    }
}
