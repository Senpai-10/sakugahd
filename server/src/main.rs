#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_seek_stream::SeekStream;

#[get("/")]
fn home() -> String {
    String::from("Home page")
}

#[get("/video/<file_name>")]
fn video<'a>(file_name: String) -> std::io::Result<SeekStream<'a>> {
    let video_path = format!(
        "/run/media/senpai/Toshiba external hard drive/anime/Bleach (English SUB)/{}",
        file_name
    );

    SeekStream::from_path(video_path)
}

#[rocket::main]
async fn main() {
    match rocket::build()
        .mount("/api", routes![home, video])
        .launch()
        .await
    {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Rocket stopped unexpectedly. (Error {})", e);
        }
    };
}
