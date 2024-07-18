#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::response::content;
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

#[get("/")]
fn page_index() -> content::RawHtml<String> {
    let file_result = &fs::read_to_string("./web/index.html");
    match file_result {
        Ok(file) => content::RawHtml(String::from(file)),
        Err(error) => content::RawHtml(format!("error: {}", error)),
    }
}

#[get("/style")]
fn css() -> content::RawCss<String> {
    let file_result = &fs::read_to_string("./web/style.css");
    match file_result {
        Ok(file) => content::RawCss(String::from(file)),
        Err(error) => content::RawCss(format!("{}", error)),
    }
}

#[get("/write/<name>")]
fn write_name(name: &str) -> String {
    let mut options = OpenOptions::new();
    let file = options.append(true).create(true).open("names.txt");
    match file {
        Err(error) => format!("error: {}", error),
        Ok(mut writable) => match writable.write_all(format!("{},\n", name).as_bytes()) {
            Err(error) => format!("error {}", error),
            Ok(_) => "name added!! the list of names is viewable at /names".to_owned(),
        },
    }
}

#[catch(404)]
fn error404() -> String {
    "404!! nya~ :3".to_owned()
}

#[get("/names")]
fn names() -> String {
    match fs::read_to_string("./names.txt") {
        Err(error) => format!("error: {}", error),
        Ok(file) => file,
    }
}

#[get("/script")]
fn script() -> content::RawJavaScript<String> {
    let file_result = &fs::read_to_string("./web/script.js");
    match file_result {
        Ok(file) => content::RawJavaScript(String::from(file)),
        Err(error) => content::RawJavaScript(format!("{}", error)),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![css, names, page_index, script, write_name])
        .mount("/fonts", FileServer::from("./web/fonts/"))
        .register("/", catchers![error404])
}
