use std::iter::Peekable;
use std::{fmt::Debug, iter::Enumerate};

use crate::pretty::{part_trait::PartTrait, Part};

pub enum Next<T> {
    Only(T),
    First(T),
    Middle(T),
    Last(T),
}

impl<T> Next<T> {
    pub fn of(iterator: &mut Peekable<Enumerate<impl Iterator<Item = T>>>) -> Option<Next<T>> {
        let (i, item) = iterator.next()?;
        let last = iterator.peek().is_none();

        let next = match (i, last) {
            (0, true) => Next::Only(item),
            (0, false) => Next::First(item),
            (_, true) => Next::Last(item),
            (_, false) => Next::Middle(item),
        };

        Some(next)
    }

    #[allow(unused)]
    pub fn item(self) -> T {
        match self {
            Next::Only(item) => item,
            Next::First(item) => item,
            Next::Middle(item) => item,
            Next::Last(item) => item,
        }
    }

    #[allow(unused)]
    pub fn is_only(&self) -> bool {
        match self {
            Next::Only(_) => true,
            _ => false,
        }
    }

    #[allow(unused)]
    pub fn is_first(&self) -> bool {
        match self {
            Next::First(_) | Next::Only(_) => true,
            _ => false,
        }
    }

    #[allow(unused)]
    pub fn is_last(&self) -> bool {
        match self {
            Next::Last(_) | Next::Only(_) => true,
            _ => false,
        }
    }
}

pub struct PartIterator {
    iterator: Peekable<Enumerate<Box<dyn Iterator<Item = Part> + 'static>>>,
}

impl Debug for PartIterator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PartIterator")
            .field("iterator", &"Peekable<Enumerate<{Part iterator}>>")
            .finish()
    }
}

#[allow(non_snake_case)]
pub fn PartIterator(
    iterator: impl IntoIterator<IntoIter = impl Iterator<Item = impl PartTrait> + 'static>,
) -> PartIterator {
    let iterator: Box<dyn Iterator<Item = Part> + 'static> =
        Box::new(iterator.into_iter().map(Part::of));

    PartIterator {
        iterator: iterator.enumerate().peekable(),
    }
}

impl Iterator for PartIterator {
    type Item = Next<Part>;

    fn next(&mut self) -> Option<Self::Item> {
        Next::of(&mut self.iterator)
    }
}

impl Debug for MapIterator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapIterator")
            .field("iterator", &"Peekable<Enumerate<{(Part, Part) iterator}>>")
            .finish()
    }
}

type InnerMapIterator = Peekable<Enumerate<Box<dyn Iterator<Item = (Part, Part)> + 'static>>>;

pub struct MapIterator {
    iterator: InnerMapIterator,
}

#[allow(non_snake_case)]
pub fn MapIterator(
    iterator: impl IntoIterator<IntoIter = impl Iterator<Item = (impl PartTrait, impl PartTrait)> + 'static>
        + 'static,
) -> MapIterator {
    let iterator: Box<dyn Iterator<Item = (Part, Part)>> = Box::new(
        iterator
            .into_iter()
            .map(|(k, v)| (Part::of(k), Part::of(v))),
    );
    let iterator = iterator.enumerate().peekable();

    MapIterator { iterator }
}

impl MapIterator {
    pub fn into_entries(self) -> impl Iterator<Item = (Part, Part)> {
        self.iterator.map(|(_, pair)| pair)
    }
}

impl Iterator for MapIterator {
    type Item = Next<(Part, Part)>;

    fn next(&mut self) -> Option<Self::Item> {
        Next::of(&mut self.iterator)
    }
}
