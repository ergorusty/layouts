use crate::{
    defpart,
    pretty::{
        iterator::{MapIterator, PartIterator},
        nesting::{Container, Group},
        part_trait::PartTrait,
        scalars::SP,
        sigils::{MapSigils, Sep, Separator, StdWrappers},
        Part,
    },
};

defpart! {
    struct List {
        iterator: PartIterator
    }

    impl Part(|list| Container(("[", "]"), list.iterator))
}

defpart! {
    struct Dictionary {
        iterator: MapIterator
    }

    impl Part(|dict| MapLike(dict.iterator,
        MapSigils::new(StdWrappers::Curlies, (":", SP))))
}

defpart! {
    struct Map {
        iterator: MapIterator
    }

    impl Part(|map| MapLike(map.iterator, (StdWrappers::Curlies, (SP, "=>", SP))))
}

defpart! {
    struct MapLike {
        map: MapIterator,
        sigils: MapSigils,
    }

    impl Part(|MapLike { map, sigils: MapSigils { wrap, sep } }| {
        Container(wrap, PartIterator(
            map.into_entries()
                .map(move |(k, v)| Part::of(MapEntry(k, v, sep)))))
    })
}

defpart! {
    struct MapEntry {
        key: Part,
        value: Part,
        sep: Sep
    }

    constructor(key: impl PartTrait, value: impl PartTrait, sep: impl Separator) {
        MapEntry { key: Part::of(key), value: Part::of(value), sep: sep.sep() }
    }

    impl Part(|MapEntry { key, value, sep: Sep { separator, space_before, space_after } }| {
        Group(|g| g + Group(|g| g + key + space_before + separator) + space_after + value)
    })
}
