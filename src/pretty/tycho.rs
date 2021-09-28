use crate::{
    defpart,
    pretty::{
        containers::{Dictionary, List, Map},
        iterator::{MapIterator, PartIterator},
        nesting::Wrapped,
        part::Part,
        part_trait::PartTrait,
        scalars::{DebugText, Opaque, Text},
        sigils::StdWrappers,
    },
};

defpart! {
    for tycho::Element;

    impl Part(|part, element| match element {
        tycho::Element::Unit => part + "unit",
        tycho::Element::Value(value) => part + value,
        tycho::Element::Option(option) => match option {
            Some(element) => part + element,
            None => part + "None",
        },
        tycho::Element::Variant(tag, element) => {
            part + tag + Wrapped(StdWrappers::Parens, element)
        }
        tycho::Element::Struct(s) => part + Dictionary(MapIterator(s.into_iter())),
        tycho::Element::List(list) => part + List(PartIterator(list)),
        tycho::Element::Array(_id, _list) => todo!("Element::Array"),
        tycho::Element::Map(_id, map) => part + Map(MapIterator(map.into_iter())),
        tycho::Element::Compression(_) => todo!("Element::Compression"),
    })
}

// impl PartTrait for tycho::Element {
//     fn append_to(self, left: Part) -> Part {
//         match self {
//             tycho::Element::Unit => left + "unit",
//             tycho::Element::Value(value) => left + value,
//             tycho::Element::Option(option) => match option {
//                 Some(element) => left + element,
//                 None => left + "None",
//             },
//             tycho::Element::Variant(tag, element) => {
//                 left + tag + Wrapped(StdWrappers::Parens, element)
//             }
//             tycho::Element::Struct(s) => left + Dictionary(MapIterator(s.into_iter())),
//             tycho::Element::List(list) => left.append(List(PartIterator(list))),
//             tycho::Element::Array(_id, _list) => todo!("Element::Array"),
//             tycho::Element::Map(_id, map) => left.append(Map(MapIterator(map.into_iter()))),
//             tycho::Element::Compression(_) => todo!("Element::Compression"),
//         }
//     }
// }

impl PartTrait for Box<tycho::Element> {
    fn append_to(self, builder: Part) -> Part {
        (*self).append_to(builder)
    }
}

impl PartTrait for tycho::Value {
    fn append_to(self, left: Part) -> Part {
        match self {
            tycho::Value::Null => left + "null",
            tycho::Value::Boolean(bool) => left + Text(bool),
            tycho::Value::String(string) => left + DebugText(string),
            tycho::Value::Char(char) => left + DebugText(char),
            tycho::Value::Number(num) => left + Text(num),
            tycho::Value::Bytes(_bytes) => {
                // TODO actually display bytes
                left + Opaque("bytes")
            }
            tycho::Value::UUID(uuid) => left + Text(uuid),
        }
    }
}
