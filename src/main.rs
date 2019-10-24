#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::json::Json;
use rocket::State;
use std::sync::{Mutex};

fn reflme() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    #[serde(default="reflme", rename(serialize = "refl.me"))]
    reflme: bool,
    message: String,
}

type Messages = Mutex<Vec<Message>>;

#[post("/push", format = "json", data = "<input>")]
fn push(input: Json<Message>, state: State<Messages>) -> &'static str {
    let mut messages = state.lock().expect("state lock");
    messages.push(input.0);
    "saved"
}

#[get("/pop")]
fn pop(state: State<Messages>) -> Option<Json<Message>> {
    let mut messages = state.lock().expect("state lock");
    messages.pop().map(|m| Json(m))

}

fn main() {
    let state: Messages = Mutex::new(vec![]);
    rocket::ignite()
        .mount("/", routes![push, pop])
        .manage(state)
        .launch();
}
