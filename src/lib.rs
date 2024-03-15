pub mod application {
    pub mod notes;

    pub mod dtos {
        pub mod commit;
        pub mod release;

        pub(crate) mod common;
    }
}

pub mod repositories {
    pub mod github;
    pub mod system;
}

pub mod services {
    pub mod jwt;
}

pub mod config;
