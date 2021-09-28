use crate::pretty::{part::PrettyBuilder, Part};

pub trait PartTrait: 'static {
    fn append_to(self, left: Part) -> Part;
}

pub(crate) fn low_level_part(part: impl PartTrait) -> PrettyBuilder {
    Part::of(part).inner
}

impl<P> PartTrait for Option<P>
where
    P: PartTrait,
{
    fn append_to(self, builder: Part) -> Part {
        match self {
            Some(part) => builder.append(part),
            None => builder,
        }
    }
}
