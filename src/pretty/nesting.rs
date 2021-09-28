use crate::{
    defpart,
    pretty::{
        iterator::PartIterator,
        part::LowLevel,
        part_trait::{low_level_part, PartTrait},
        scalars::SP,
        sigils::Wrap,
        string, Part,
    },
};

use super::iterator::Next;

defpart! {
    struct Wrapped {
        wrap: (string, string),
        part: Part,
    }

    constructor(wrap: impl Wrap, part: impl PartTrait) {
        Wrapped {
            wrap: wrap.wrapper(),
            part: Part::of(part)
        }
    }

    impl Part(|Wrapped { wrap: (prefix, suffix), part }| Group(|g| g + prefix + Nest(part) + suffix))
}

defpart! {
    struct Nest {
        part: Part
    }

    constructor(part: impl PartTrait) {
        Nest {
            part: Part::of(part)
        }
    }

    impl Part(|Nest { part }| SP + LowLevel(|pretty| pretty.append(low_level_part(part)).nest(2)) + SP)
}

defpart! {
    struct Group {
        part: Part
    }

    constructor(callback: impl FnOnce(Part) -> Part) {
        Group { part: callback(Part::new()) }
    }

    impl Part(|Group { part }| part.low_level(|low_level| low_level.group()))
}

defpart! {
    struct Parts {
        parts: Part,
    }

    constructor(build: impl FnOnce(Part) -> Part) {
        Parts { parts: build(Part::new()) }
    }

    impl Part(|Parts { parts }| parts)
}

defpart! {
    struct Container {
        wrap: (string, string),
        body: PartIterator,
    }

    constructor(wrap: impl Wrap, body: impl Into<PartIterator>) {
        Container {
            wrap: wrap.wrapper(),
            body: body.into()
        }
    }

    impl Part(|Container { wrap, body }| {
        let list = Part::build(body, |part, next| match next {
            Next::Only(item) | Next::Last(item) => part + item,
            Next::First(item) | Next::Middle(item) => part + Group(|g| g + item + ",") + SP,
        });

        Wrapped(wrap, list)
    })
}
