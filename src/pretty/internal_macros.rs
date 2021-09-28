#[macro_export]
macro_rules! defpart {
    (
        for $name:ty;

        impl Part(|$right:pat $(,)?| $($body:tt)*)

    ) => {
        impl $crate::PartTrait for $name {
            fn append_to(self, left: Part) -> Part {
                let $right = self;

                left + $($body)*
            }
        }
    };

    (
        for $name:ty;

        impl Part(|$left:ident, $right:pat $(,)?| $($body:tt)*)

    ) => {
        impl $crate::PartTrait for $name {
            fn append_to(self, left: Part) -> Part {
                let $right = self;
                let $left = left;

                $($body)*
            }
        }
    };

    (
        #[lowlevel]
        for $name:ident;

        impl Part(|$pretty:ident, $right:pat $(,)?| $($body:tt)*)

    ) => {
        impl $crate::PartTrait for $name {
            fn append_to(self, left: Part) -> Part {
                let $right = self;

                left.low_level(|pretty| {
                    let $pretty = pretty;

                    $($body)*
                })
            }
        }
    };

    // defpart! {
    //     #[lowlevel]
    //     for Whitespace;

    //     impl Part(|low_level, whitespace| match whitespace {
    //         Whitespace::Space => low_level.append(BoxAllocator.line_()),
    //         Whitespace::Break => low_level.append(BoxAllocator.line()),
    //     })
    // }


    // impl PartTrait for Whitespace {
    //     fn append_to(self, builder: Part) -> Part {
    //         builder.low_level(|pretty| match self {
    //             Whitespace::Break => pretty.append(BoxAllocator.line_()),
    //             Whitespace::Space => pretty.append(BoxAllocator.line()),
    //         })
    //     }
    // }

    (
        #[lowlevel]
        struct $name:ident {
            $($field_name:ident : $field_ty:ty),* $(,)?
        }

        constructor($($constructor_param:ident : $constructor_ty:ty),* $(,)?) {
            $($constructor_body:tt)*
        }

        impl Part(|$pretty:ident, $right:pat $(,)?| $($body:tt)*)
    ) => {
        #[derive(Debug)]
        pub struct $name {
            $(
                $field_name : $field_ty,
            )*
        }

        impl $crate::PartTrait for $name {
            fn append_to(self, left: Part) -> Part {
                let $right = self;

                left.low_level(|pretty| {
                    let $pretty = pretty;

                    $($body)*
                })
            }
        }

        #[allow(non_snake_case)]
        pub fn $name( $($constructor_param : $constructor_ty,)* ) -> $name {
            $($constructor_body)*
        }
    };


    (
        struct $name:ident {
            $($field_name:ident : $field_ty:ty),* $(,)?
        }

        constructor($($constructor_param:ident : $constructor_ty:ty),* $(,)?) {
            $($constructor_body:tt)*
        }

        impl Part(|$right:pat $(,)?| $($body:tt)*)
    ) => {
        #[derive(Debug)]
        pub struct $name {
            $(
                $field_name : $field_ty,
            )*
        }

        impl $crate::PartTrait for $name {
            fn append_to(self, left: Part) -> Part {
                let $right = self;

                left + $($body)*
            }
        }

        #[allow(non_snake_case, unused)]
        pub fn $name( $($constructor_param : $constructor_ty,)* ) -> $name {
            $($constructor_body)*
        }
    };



    (struct $name:ident { $($field_name:ident : $field_ty:ty),* $(,)? } impl Part(|$right:pat| $($body:tt)*)) => {
        #[derive(Debug)]
        pub struct $name {
            $(
                $field_name : $field_ty,
            )*
        }

        impl $crate::PartTrait for $name {
            fn append_to(self, left: Part) -> Part {
                let $right = self;

                left + $($body)*
            }
        }

        #[allow(non_snake_case)]
        pub fn $name( $($field_name : impl Into<$field_ty>, )* ) -> $name {
            $name {
                $(
                    $field_name: $field_name.into(),
                )*
            }
        }
    }
}
