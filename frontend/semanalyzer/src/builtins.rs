pub mod fns {
    pub mod typedef {
        use elise_shared_types::Name;

        pub const NAME: &str = Name::FN__TYPEDEF;
        pub const ARGS_LEN: usize = 2;
    }

    pub mod get {
        use elise_shared_types::Name;

        pub const NAME: &str = Name::FN__GET;
        pub const ARGS_LEN_MIN: usize = 1;
    }

    pub mod add {
        use elise_shared_types::Name;

        pub const NAME: &str = Name::FN__ADD;
        pub const ARGS_LEN_MIN: usize = 1;
    }
}
