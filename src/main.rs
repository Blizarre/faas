#[macro_use]
extern crate rocket;

use rand::prelude::{thread_rng, RngCore};
use rocket::State;
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Fortunes {
    fortunes: Vec<String>,
}

#[get("/")]
fn shuffle(fortunes: &State<Fortunes>) -> &str {
    fortunes.random()
}

#[get("/<id>")]
fn specific(id: usize, fortunes: &State<Fortunes>) -> &str {
    fortunes.get(id)
}

impl Fortunes {
    fn load(path: &str) -> io::Result<Fortunes> {
        let mut f = File::open(path)?;
        let mut str = String::new();
        f.read_to_string(&mut str)?;
        Ok(Fortunes {
            fortunes: str
                .split('%')
                .map(|x| x.trim())
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        })
    }

    fn random(&self) -> &str {
        self.get(thread_rng().next_u32() as usize)
    }

    fn get(&self, index: usize) -> &str {
        let len = self.fortunes.len();
        &self.fortunes[index % len]
    }
}

#[launch]
fn rocket() -> _ {
    let fortunes = Fortunes::load("fortunes").expect("Could not load fortune file");
    rocket::build()
        .manage(fortunes)
        .mount("/", routes![shuffle, specific])
}
