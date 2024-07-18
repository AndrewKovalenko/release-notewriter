pub mod application {
    pub mod notes;

    pub mod dtos {
        pub mod access_token;
        pub mod commit;
        pub mod gpt_response;
        pub mod installation;
        pub mod llm_request;
        pub mod release;
        pub mod repository;

        pub mod common;
    }
}

pub mod repositories {
    pub mod github;
    pub mod gpt;
    pub mod system;
}

pub mod services {
    pub mod jwt;
    pub mod release_notes_generator;
}

pub mod config;
