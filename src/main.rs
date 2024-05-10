#[macro_use]
extern crate rocket;
use handlebars::Handlebars;
use rocket::response::content;
use rocket_dyn_templates::handlebars;
use rocket_dyn_templates::{Template, context};
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;
use rocket::fs::NamedFile;
use std::path::PathBuf;

#[derive(serde::Serialize)]
struct Posting {
   title: String,
   author: String,
   image_path: String,
}

#[get("/static/<file..>")]
pub async fn servefiles(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[get("/")]
fn home() -> Template {
    let mut context = HashMap::new();
    let posts = vec![
        Posting  
            {
                title: "Blustery day at the Beach".to_string(),
                author: "LCrossman".to_string(),
                image_path: "static/images/beach.jpg".to_string(),
            },
        Posting
            {
                title: "Trip to the woods".to_string(),
                author: "Wild garlic".to_string(),
                image_path: "static/images/wood.jpg".to_string(),
            },
        Posting
            {
                title: "Famous weathervane".to_string(),
                author: "Church tower".to_string(),
                image_path: "static/images/weathervane.jpg".to_string(),
            },
        ];
    context.insert("posts", &posts);
    Template::render("posts", &context)
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
       .attach(Template::fairing())
       .mount("/", routes![home, servefiles]);
    Ok(rocket.into())
}

