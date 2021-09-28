use std::{fmt::Debug, ops::Add};

use pretty::{BoxAllocator, BoxDoc, DocAllocator};

use crate::pretty::{doc::Doc, iterator::Next, part_trait::PartTrait};

pub(crate) type PrettyBuilder = pretty::DocBuilder<'static, BoxAllocator, ()>;
pub(crate) type PrettyDoc = BoxDoc<'static>;

pub struct Part {
    pub(crate) inner: PrettyBuilder,
}

#[allow(non_snake_case)]
pub fn Part(part: impl PartTrait) -> Part {
    Part::of(part)
}

impl Debug for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Part")
            .field("inner", &"{pretty::DocBuilder}")
            .finish()
    }
}

impl<P> Add<P> for Part
where
    P: PartTrait,
{
    type Output = Part;

    fn add(self, rhs: P) -> Part {
        self.append(rhs)
    }
}

impl PartTrait for Part {
    fn append_to(self, left_part: Part) -> Part {
        let left = left_part.inner;
        let right = self.inner;

        let pretty = left.append(right);

        Part { inner: pretty }
    }
}

impl Part {
    pub fn of(item: impl PartTrait) -> Part {
        Part::new().initialize(item)
    }

    pub fn build(
        iterator: impl Iterator<Item = Next<Part>>,
        callback: impl Fn(Part, Next<Part>) -> Part,
    ) -> Part {
        iterator.fold(Part::new(), |list, next| callback(list, next))
    }

    pub(crate) fn new() -> Part {
        Part {
            inner: BoxAllocator.nil(),
        }
    }

    pub(crate) fn low_level(self, callback: impl FnOnce(PrettyBuilder) -> PrettyBuilder) -> Part {
        let inner = self.inner;
        let inner = callback(inner);
        Part { inner }
    }

    pub fn into_doc(self) -> Doc {
        Doc(self.inner.into_doc())
    }

    pub(crate) fn initialize(self, item: impl PartTrait) -> Part {
        item.append_to(self)
    }

    pub fn append(self, item: impl PartTrait) -> Part {
        item.append_to(self)
    }
}

pub struct LowLevel<F>
where
    F: FnOnce(PrettyBuilder) -> PrettyBuilder + 'static,
{
    callback: F,
}

impl<F> PartTrait for LowLevel<F>
where
    F: FnOnce(PrettyBuilder) -> PrettyBuilder,
{
    fn append_to(self, left: Part) -> Part {
        let inner = left.inner;
        let inner = (self.callback)(inner);
        Part { inner }
    }
}

#[allow(non_snake_case)]
pub fn LowLevel<F>(callback: F) -> LowLevel<F>
where
    F: FnOnce(PrettyBuilder) -> PrettyBuilder,
{
    LowLevel { callback }
}

impl Add<PrettyDoc> for Part {
    type Output = Part;

    fn add(self, rhs: PrettyDoc) -> Self::Output {
        Part {
            inner: self.inner.append(rhs),
        }
    }
}

impl Into<PrettyDoc> for Part {
    fn into(self) -> PrettyDoc {
        self.inner.into_doc()
    }
}
