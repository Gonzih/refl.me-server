#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::json::Json;
use rocket::State;
use std::sync::{Mutex};
use std::collections::HashMap;

fn default_reflme() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    #[serde(default="default_reflme", rename(serialize = "refl.me"))]
    reflme: bool,
    message: String,
    title: Option<String>,
    image: Option<String>,
    go_link: Option<String>,
}

fn empty_msg() -> Message {
    Message {
        reflme: false,
        message: "".to_owned(),
        title: None,
        image: None,
        go_link: None,
    }
}

type Messages = Mutex<HashMap<String,Vec<Message>>>;

#[post("/<id>/push", format = "json", data = "<input>")]
fn push(id: String, input: Json<Message>, state: State<Messages>) -> &'static str {
    let mut messages = state.lock().expect("state lock");
    let v = messages.entry(id).or_insert(vec![]);
    v.push(input.0);
    "saved"
}

#[get("/<id>/pop")]
fn pop(id: String, state: State<Messages>) -> Json<Message> {
    let mut messages = state.lock().expect("state lock");
    let v = messages.entry(id).or_insert(vec![]);
    Json(v.pop().unwrap_or(empty_msg()))
}

fn main() {
    let state: Messages = Mutex::new(HashMap::new());
    rocket::ignite()
        .mount("/", routes![push, pop])
        .manage(state)
        .launch();
}
