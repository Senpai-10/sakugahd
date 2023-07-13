use crate::models::{NewOpening, NewShow, Show};
use crate::schema::{openings, shows};
use diesel::prelude::*;
use std::env;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub fn loader(conn: &mut PgConnection) {
    let env_anime_directory = env::var("ANIME_DIRECTORY").expect("ANIME_DIRECTORY must be set");

    let anime_directory = Path::new(&env_anime_directory);

    let mut list_of_shows: Vec<NewShow> = Vec::new();
    let mut list_of_openings: Vec<NewOpening> = Vec::new();

    for show_dir in anime_directory
        .read_dir()
        .expect("read_dir anime_directory failed")
    {
        let show_dir = show_dir.unwrap();
        let show_name: String = match show_dir.file_name().into_string() {
            Ok(v) => v,
            Err(_) => continue,
        };

        // Check if show already exists
        match shows::dsl::shows
            .filter(shows::title.eq(&show_name))
            .first::<Show>(conn)
        {
            Ok(_) => continue,
            Err(_) => {}
        };

        let mut new_show = NewShow {
            id: Uuid::new_v4(),
            title: show_name.clone(),
            description: String::from("no description."),
            format: None,
            status: None,
            season: None,
            season_year: None,
            directory_name: show_name.clone(),
            banner: vec![],
            image: vec![],
        };
        println!("Loading show: '{}'", &show_name);
        let show_directory = show_dir.path();

        for show_entry in show_directory
            .read_dir()
            .expect("read_dir show_directory failed")
        {
            let show_entry = show_entry.unwrap();
            let file_name: String = match show_entry.file_name().into_string() {
                Ok(v) => v,
                Err(_) => continue,
            };

            if file_name.starts_with("banner") {
                println!("Found banner! '{}'", file_name);
                let bytes = match std::fs::read(show_entry.path()) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        eprintln!("Failed to read {file_name}, {e}");
                        continue;
                    }
                };
                new_show.banner = bytes;
            } else if file_name.starts_with("image") {
                println!("Found image! '{}'", file_name);
                let bytes = match std::fs::read(show_entry.path()) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        eprintln!("Failed to read {file_name}, {e}");
                        continue;
                    }
                };
                new_show.image = bytes;
            } else if file_name == "openings" {
                println!("openings directory");
                load_openings(show_entry.path(), new_show.id, &mut list_of_openings);
            } else if file_name == "endings" {
                println!("endings directory");
                load_endings(show_entry.path());
            } else if file_name == "movies" {
                println!("movies directory");
                load_movies(show_entry.path());
            } else if file_name == "eps" {
                println!("eps directory");
                load_eps(show_entry.path());
            }
        }
        list_of_shows.push(new_show);
    }

    // Push list_of_shows to shows table

    diesel::insert_into(shows::dsl::shows)
        .values(&list_of_shows)
        .execute(conn)
        .expect("Error saving shows");

    diesel::insert_into(openings::dsl::openings)
        .values(&list_of_openings)
        .execute(conn)
        .expect("Error saving shows");

    // let new_show = NewShow {
    //     id: Uuid::new_v4(),
    //     title: "bleach",
    //     description: "some description.",
    //     format: None,
    //     status: None,
    //     season: None,
    //     season_year: None,
    //     directory_name: "Bleach (English SUB)",
    //     image: vec![0],
    //     banner: vec![0],
    // };

    // diesel::insert_into(shows)
    //     .values(&new_show)
    //     .execute(&mut connection)
    //     .expect("Error saving new show");
}

fn load_openings(dir: PathBuf, show_id_: Uuid, list: &mut Vec<NewOpening>) {
    for opening in dir.read_dir().expect("read_dir openings failed") {
        let opening = opening.unwrap();

        // let file_without_extension: String = match opening.path().file_stem() {
        //     Some(v) => v.into(),
        //     None => continue,
        // };

        // TODO: rewrite this!
        let file_without_ext: String = opening
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let file_name_: String = match opening.file_name().into_string() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let new_opening = NewOpening {
            id: Uuid::new_v4(),
            show_id: show_id_,
            title: file_without_ext,
            file_name: file_name_.clone(),
            thumbnail: vec![],
        };

        // TODO: Find a way to genrate a thumbnail

        list.push(new_opening);
    }
}
fn load_endings(dir: PathBuf) {}
fn load_movies(dir: PathBuf) {}
fn load_eps(dir: PathBuf) {}
