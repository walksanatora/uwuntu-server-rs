#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use chrono::prelude::*;
use once_cell::sync::OnceCell;
use rocket::fs::NamedFile;
use rocket::http::{ContentType, Status};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use serde::Serialize;
use std::fs::read_dir;
use std::sync::Mutex;


struct State {
    last_fetch: NaiveTime,
    last_build: NaiveTime
}

static STATE: OnceCell<Mutex<State>> = OnceCell::new();

#[derive(Serialize)]
struct Message{
    message: String,
    code: u16
}

#[get("/trigger/<trig>")]
async fn trigger(trig: String) -> (Status,Json<Message>) {
    match trig.as_str() {
        "rebuild" => {
            (Status::NotImplemented,Json(Message { message: "Not implemented yet".into(), code: 501 }))
        }
        _ => {(Status::BadRequest,Json(Message { message: "Invalid Trigger".into(), code: 400 }))}
    }
}

#[get("/download/<target>")]
async fn branches(target: String) -> Result<Json<Vec<String>>,NotFound<String>>  {
    let branches = read_dir(format!("files/{}",target));
    if let Ok(files) = branches {
        let output = files.map(|f|{
            let mut s = f.unwrap()
            .file_name()
            .into_string()
            .unwrap();
            s.pop();
            s.pop();
            s.pop();
            s.pop(); //fuck unicode making have to do this
            s
        }).collect();
        Ok(Json(output))
    } else {
        Err(NotFound("Invalid target".into()))
    }
    
}

#[get("/download/<target>/<branch>")]
async fn download(target: String,branch: String) -> Result<(ContentType,NamedFile), NotFound<String>> {
    let ext = match target.as_str() {
        "installer" => "lua",
        "orangebox" => "vgz",
        "yellowbox" => "vfs",
        _ => ""
    };
    let ct = match target.as_str() {
        "installer" => ContentType::parse_flexible("application/x-lua").unwrap(),
        "orangebox" => ContentType::parse_flexible("application/gzip+txt").unwrap(),
        "yellowbox" => ContentType::parse_flexible("application/octet-stream").unwrap(),
        _ => ContentType::default()
    };
    let file = format!("files/{}/{}.{}",target,branch,ext);
    let nf= NamedFile::open(file).await.map_err(|e| NotFound(e.to_string()));
    match nf {
        Ok(named) => {Ok((ct,named))},
        Err(e) => {Err(e)},
    }
}

#[launch]
fn rocket() -> _ {
    let _ = STATE.set(Mutex::new(State { last_fetch: NaiveTime::default(), last_build: NaiveTime::default() }));
    rocket::build().mount("/", routes![branches,download,trigger])
}