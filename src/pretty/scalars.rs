use std::fmt::{Debug, Display};

use pretty::{BoxAllocator, DocAllocator};

use crate::{
    defpart,
    pretty::{string, Part},
};

defpart! {
    #[lowlevel]
    for string;

    impl Part(|low_level, s| low_level.append(s))
}

defpart! {
    #[lowlevel]
    for String;

    impl Part(|low_level, s| low_level.append(s))
}

defpart! {
    #[lowlevel]
    struct Text {
        text: String,
    }

    constructor(text: impl Display) {
        Text { text: text.to_string() }
    }

    impl Part(|low_level, Text { text }| low_level.append(text))
}

#[allow(non_snake_case)]
pub fn DebugText(text: impl Debug + Display) -> Text {
    Text {
        text: format!("{:?}", text),
    }
}

pub const SP: Whitespace = Whitespace::Space;
#[allow(unused)]
pub const BR: Whitespace = Whitespace::Break;

defpart! {
    #[lowlevel]
    struct Opaque {
        tag: String,
    }

    constructor(text: impl Display) {
        Opaque { tag: text.to_string() }
    }

    impl Part(|low_level, Opaque { tag }| low_level.append(tag))
}

#[derive(Debug, Copy, Clone)]
pub enum Whitespace {
    /// A space is a newline when the surrounding group is broken, or a single
    /// space when it's not.
    Space,
    #[allow(unused)]
    /// A break is a newline when the surrounding group is broken, or nothing
    /// when it's not.
    Break,
}

// impl PartTrait for Whitespace {
//     fn append_to(self, builder: Part) -> Part {
//         builder.low_level(|pretty| match self {
//             Whitespace::Break => pretty.append(BoxAllocator.line_()),
//             Whitespace::Space => pretty.append(BoxAllocator.line()),
//         })
//     }
// }

defpart! {
    #[lowlevel]
    for Whitespace;

    impl Part(|low_level, whitespace| match whitespace {
        Whitespace::Space => low_level.append(BoxAllocator.line()),
        Whitespace::Break => low_level.append(BoxAllocator.line_()),
    })
}
