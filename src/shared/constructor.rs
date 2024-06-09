#[macro_export]
macro_rules! constructor {
    (
        <$($generic:tt),*> $s:ident where $($generic2:ident: $($generic_constraint:ty),*),* {
            $(
                $constructor_name:ident($($arg:ident: $arg_ty:ty),*) $constructor_block:block
            ),*
        }
    ) => {
        paste::paste! {
            pub enum [<$s Constructor>]<$($generic),*> where $($generic2: $($generic_constraint),*),* {
                $(
                    $constructor_name {
                        $(
                            $arg: $arg_ty
                        ),*
                    }
                ),*
            }

            impl <$($generic),*> [<$s Constructor>]<$($generic),*> where $($generic2: $($generic_constraint),*),* {
                pub fn construct(self) -> $s<$($generic),*> {
                    match self {
                        $(
                            [<$s Constructor>]::$constructor_name { $($arg),* } => $constructor_block
                        ),*
                    }
                }
            }

            impl <$($generic),*> $s <$($generic),*> where $($generic2: $($generic_constraint),*),* {
                pub fn new(constructor: [<$s Constructor>]<$($generic),*>) -> Self {
                    constructor.construct()
                }
            }
        }
    };

    (
        <$($generic:tt),*> $s:ident {
            $(
                $constructor_name:ident($($arg:ident: $arg_ty:ty),*) $constructor_block:block
            ),*
        }
    ) => {
        paste::paste! {
            pub enum [<$s Constructor>]<$($generic),*> {
                $(
                    $constructor_name {
                        $(
                            $arg: $arg_ty
                        ),*
                    }
                ),*
            }

            impl <$($generic),*> [<$s Constructor>]<$($generic),*> {
                pub fn construct(self) -> $s<$($generic),*> {
                    match self {
                        $(
                            [<$s Constructor>]::$constructor_name { $($arg),* } => $constructor_block
                        ),*
                    }
                }
            }

            impl <$($generic),*> $s <$($generic),*> {
                pub fn new(constructor: [<$s Constructor>]<$($generic),*>) -> Self {
                    constructor.construct()
                }
            }
        }
    };
    (
        $s:ident {
            $(
                $constructor_name:ident($($arg:ident: $arg_ty:ty),*) $constructor_block:block
            ),*
        }
    ) => {
        paste::paste! {
            pub enum [<$s Constructor>] {
                $(
                    $constructor_name {
                        $(
                            $arg: $arg_ty
                        ),*
                    }
                ),*
            }

            impl [<$s Constructor>] {
                pub fn construct(self) -> $s {
                    match self {
                        $(
                            [<$s Constructor>]::$constructor_name { $($arg),* } => $constructor_block
                        ),*
                    }
                }
            }

            impl $s {
                pub fn new(constructor: [<$s Constructor>]) -> Self {
                    constructor.construct()
                }
            }
        }
    };
}