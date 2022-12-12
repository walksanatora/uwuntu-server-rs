#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;
use rocket::http::ContentType;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use std::fs::read_dir;


#[get("/download/<t>")]
async fn branches(t: String) -> Result<Json<Vec<String>>,NotFound<String>>  {
    let branches = read_dir(format!("files/{}",t));
    if let Ok(files) = branches {
        let output = files.map(|f|{
            let s = f.unwrap()
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
        Err(NotFound("Invalid type".into()))
    }
    
}

#[get("/download/<t>/<branch>")]
async fn download(t: String,branch: String) -> Result<(ContentType,NamedFile), NotFound<String>> {
    let ext = match t.as_str() {
        "installer" => "lua",
        "orangebox" => "vgz",
        "yellowbox" => "vfs",
        _ => ""
    };
    let ct = match t.as_str() {
        "installer" => ContentType::parse_flexible("application/x-lua").unwrap(),
        "orangebox" => ContentType::parse_flexible("application/gzip+txt").unwrap(),
        "yellowbox" => ContentType::parse_flexible("application/octet-stream").unwrap(),
        _ => ContentType::default()
    };
    let file = format!("files/{}/{}.{}",t,branch,ext);
    let nf= NamedFile::open(file).await.map_err(|e| NotFound(e.to_string()));
    match nf {
        Ok(named) => {Ok((ct,named))},
        Err(e) => {Err(e)},
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![branches,download])
}