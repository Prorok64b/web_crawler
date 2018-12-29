pub mod configs;
pub mod modules;

extern crate cdrs;
extern crate reqwest;
extern crate scraper;
extern crate url;
extern crate yaml_rust;

use scraper::{Html, Selector};
use url::Url;

use cdrs::authenticators::PasswordAuthenticator;
use cdrs::client::{ Credentials, Session, CDRS };
use cdrs::compression::Compression;
use cdrs::consistency::Consistency;
use cdrs::error as cdrs_error;

fn main() {
    //    let link = "https://rust-lang.org";
    //    let url = Url::parse(&link).expect("cannot parse url");

    //    println!("{0}", url.host().unwrap());
    //    get_links(link);

    let mut addr:String = String::new();
    let mut username:String = String::new();
    let mut password:String = String::new();
    configs::configs::db_session_set(&mut addr, &mut username, &mut password);

    let mut authenticator:PasswordAuthenticator = PasswordAuthenticator::new(username.as_str(), password.as_str());
    let mut session = configs::configs::new_session(addr.as_str(), authenticator).unwrap();

    let name = "vasia2";
    let email = "test@mail.com";

    let insert_struct_cql = format!(
        "INSERT INTO test_space.users \
         (id, email, name) VALUES (uuid(), '{0}', '{1}')",
        email.to_string(),
        name
    );

    session
        .query(
            insert_struct_cql.to_string(),
            Consistency::One,
            None,
            None, // with names
            None, // page size
            None, // paging state
            None, // serial consistency
            None, // timestamp
        )
        .expect("insert");

    session.end();
}

fn get_body(link: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(link);
    let body = resp?.text();

    body
}

fn get_links(link: &str) -> Vec<String> {
    let url = Url::parse(&link).expect("cannot parse url");
    let body = get_body(&link).expect("cannot parse body");

    let mut links = Vec::new();
    let document = Html::parse_fragment(&body[..]);
    let selector = Selector::parse(r#"[href]"#).unwrap();

    let repair = |link: &str| -> String {
        if !link.is_empty() && !link.contains("http") {
            return format!(
                "{0}://{1}{2}",
                url.scheme(),
                url.host().unwrap().clone(),
                link
            );
        }

        link.to_string()
    };

    for element in document.select(&selector) {
        let link = element.value().attr("href").unwrap().to_string();
        links.push(link.clone());
        println!("{0}", repair(&link.clone()[..]));
    }

    links
}
