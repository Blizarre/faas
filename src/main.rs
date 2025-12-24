#[macro_use]
extern crate rocket;

use rocket::serde::Deserialize;
use rocket::{fairing::AdHoc, State};

use rand::prelude::{ThreadRng, RngCore};
use std::fs::File;
use std::io;
use std::io::prelude::*;

use rocket::response::content;

#[derive(Deserialize)]
struct AppConfig {
    fortune_path: String,
    url_prefix: String,
}

struct Fortunes {
    fortunes: Vec<String>,
}

fn template(id: usize, message: &str, url_prefix: &str) -> content::RawHtml<String> {
    content::RawHtml(
        "<!doctype html>
<html lang=\"en\">
    <head>
        <title>Faas - {{id}}</title>
    </head>
    <body>
        <pre>{{message}}</pre>
        <a href=\"{{url_prefix}}{{id}}\">🔗</a>
    </body>
</html>"
            .replace("{{id}}", &id.to_string())
            .replace("{{message}}", message)
            .replace("{{url_prefix}}", url_prefix),
    )
}

#[get("/")]
fn shuffle(fortunes: &State<Fortunes>, config: &State<AppConfig>) -> content::RawHtml<String> {
    let (id, content) = fortunes.random();
    template(id, content, &config.url_prefix)
}

#[get("/<id>")]
fn specific(
    id: usize,
    fortunes: &State<Fortunes>,
    config: &State<AppConfig>,
) -> content::RawHtml<String> {
    template(id, fortunes.get(id), &config.url_prefix)
}

#[get("/txt")]
fn shuffle_txt(fortunes: &State<Fortunes>) -> &str {
    let (_, content) = fortunes.random();
    content
}

#[get("/txt/<id>")]
fn specific_txt(id: usize, fortunes: &State<Fortunes>) -> &str {
    fortunes.get(id)
}

impl Fortunes {
    fn load(path: &str) -> io::Result<Fortunes> {
        let mut f = File::open(path)?;
        let mut str = String::new();
        f.read_to_string(&mut str)?;
        Ok(Fortunes {
            fortunes: str
                .split("\n%\n")
                .map(|x| x.trim())
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        })
    }

    fn random(&self) -> (usize, &str) {
        let id = ThreadRng::default().next_u32() as usize;
        (id, self.get(id))
    }

    fn len(&self) -> usize {
        self.fortunes.len()
    }

    fn get(&self, index: usize) -> &str {
        let len = self.fortunes.len();
        &self.fortunes[index % len]
    }
}

#[launch]
fn rocket() -> _ {
    let config = rocket::Config::figment()
        .extract::<AppConfig>()
        .expect("Could not read config");
    let fortunes = Fortunes::load(&config.fortune_path).expect("Could not load fortune file");
    println!("Loaded {} fortunes", fortunes.len());
    rocket::build()
        .attach(AdHoc::config::<AppConfig>())
        .manage(fortunes)
        .mount("/", routes![shuffle, specific, shuffle_txt, specific_txt])
}
