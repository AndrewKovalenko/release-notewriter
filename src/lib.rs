pub mod application {
    pub mod notes;

    pub mod dtos {
        pub mod commit;
        pub mod llm_request;
        pub mod release;
        pub mod repository;

        pub mod common;
    }
}

pub mod repositories {
    pub mod github;
    pub mod system;
}

pub mod services {
    pub mod jwt;
    pub mod release_notes_generator;
}

pub mod config;
