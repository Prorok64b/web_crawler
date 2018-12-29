pub mod configs {
    use std::io::Error;
    use std::fs;
    use yaml_rust::{ YamlLoader, YamlEmitter };

    use cdrs::client::{CDRS, Credentials, Session};
    use cdrs::{error as cdrs_error};
    use cdrs::authenticators::{Authenticator, PasswordAuthenticator};
    use cdrs::compression::Compression;

    pub fn db_session_set(addr: &mut String, username: &mut String, password: &mut String) {
        let all_configs = &self::configs()[0].clone();

        let host = String::from(all_configs["configs"]["cassandra"]["host"].as_str().expect("Cannot find db host in configs.yml"));
        let port = String::from(all_configs["configs"]["cassandra"]["port"].as_str().expect("Cannot find db port in configs.yml"));

        let username_conf = String::from(all_configs["configs"]["cassandra"]["username"].as_str().expect("Cannot find db username in configs.yml"));
        let password_conf = String::from(all_configs["configs"]["cassandra"]["password"].as_str().expect("Cannot find db password in configs.yml"));

//        let auth = PasswordAuthenticator::new(&username[..], &password[..]);
//        authenticator.clone_from(&auth);
//        *authenticator = auth;
        *addr = format!("{0}:{1}", host, port).to_string();
        *username = username_conf;
        *password = password_conf;
    }

    pub fn new_session<A: Authenticator>(addr: &str, authenticator: A) -> cdrs_error::Result<Session<A>> {
        let cdrs = CDRS::new(addr, authenticator);

        cdrs?.start(Compression::None)
    }

    fn configs() -> Vec<yaml_rust::Yaml> {
        let file_data = self::conf_file_data();

        let unwraped = YamlLoader::load_from_str(&file_data[..]).unwrap();
        unwraped
    }

    fn conf_file_data() -> String {
        let file_data = fs::read_to_string("configs/configs.yml")
            .expect("Something went wrong reading the file");

        file_data
    }
}