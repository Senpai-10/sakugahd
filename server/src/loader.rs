use crate::models::ending::NewEnding;
use crate::models::episode::NewEpisode;
use crate::models::movie::NewMovie;
use crate::models::opening::NewOpening;
use crate::models::show::{NewShow, Show};
use crate::schema::{endings, episodes, movies, openings, shows};
use diesel::prelude::*;
use std::env;
use std::path::{Path, PathBuf};
use uuid::Uuid;

struct Lists {
    shows: Vec<NewShow>,
    openings: Vec<NewOpening>,
    endings: Vec<NewEnding>,
    episodes: Vec<NewEpisode>,
    movies: Vec<NewMovie>,
}

pub fn loader(conn: &mut PgConnection) {
    let env_anime_directory = env::var("ANIME_DIRECTORY").expect("ANIME_DIRECTORY must be set");

    let anime_directory = Path::new(&env_anime_directory);

    let mut lists = Lists {
        shows: Vec::new(),
        openings: Vec::new(),
        endings: Vec::new(),
        episodes: Vec::new(),
        movies: Vec::new(),
    };

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
            banner: None,
            image: None,
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
                new_show.banner = Some(bytes);
            } else if file_name.starts_with("image") {
                println!("Found image! '{}'", file_name);
                let bytes = match std::fs::read(show_entry.path()) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        eprintln!("Failed to read {file_name}, {e}");
                        continue;
                    }
                };
                new_show.image = Some(bytes);
            } else if file_name == "openings" {
                println!("openings directory");
                load_openings(show_entry.path(), new_show.id, &mut lists.openings);
            } else if file_name == "endings" {
                println!("endings directory");
                load_endings(show_entry.path(), new_show.id, &mut lists.endings);
            } else if file_name == "movies" {
                println!("movies directory");
                load_movies(show_entry.path(), new_show.id, &mut lists.movies);
            } else if file_name == "eps" {
                println!("eps directory");
                load_eps(show_entry.path(), new_show.id, &mut lists.episodes);
            }
        }
        lists.shows.push(new_show);
    }

    diesel::insert_into(shows::dsl::shows)
        .values(&lists.shows)
        .execute(conn)
        .expect("Error saving shows");

    diesel::insert_into(openings::dsl::openings)
        .values(&lists.openings)
        .execute(conn)
        .expect("Error saving openings");

    diesel::insert_into(endings::dsl::endings)
        .values(&lists.endings)
        .execute(conn)
        .expect("Error saving endings");

    diesel::insert_into(episodes::dsl::episodes)
        .values(&lists.episodes)
        .execute(conn)
        .expect("Error saving episodes");

    diesel::insert_into(movies::dsl::movies)
        .values(&lists.movies)
        .execute(conn)
        .expect("Error saving movies");
}

fn load_openings(dir: PathBuf, show_id_: Uuid, list: &mut Vec<NewOpening>) {
    for opening in dir.read_dir().expect("read_dir openings failed") {
        let opening = opening.unwrap();

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
            thumbnail: None,
        };

        // TODO: Find a way to genrate a thumbnail

        list.push(new_opening);
    }
}

fn load_endings(dir: PathBuf, show_id_: Uuid, list: &mut Vec<NewEnding>) {
    for ending in dir.read_dir().expect("read_dir endings failed") {
        let ending = ending.unwrap();

        // TODO: rewrite this!
        let file_without_ext: String = ending
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let file_name_: String = match ending.file_name().into_string() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let new_ending = NewEnding {
            id: Uuid::new_v4(),
            show_id: show_id_,
            title: file_without_ext,
            file_name: file_name_.clone(),
            thumbnail: None,
        };

        // TODO: Find a way to genrate a thumbnail

        list.push(new_ending);
    }
}
fn load_movies(dir: PathBuf, show_id_: Uuid, list: &mut Vec<NewMovie>) {
    for movie in dir.read_dir().expect("read_dir movies failed") {
        let movie = movie.unwrap();

        // TODO: rewrite this!
        let file_without_ext: String = movie
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let file_name_: String = match movie.file_name().into_string() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let new_movie = NewMovie {
            id: Uuid::new_v4(),
            show_id: show_id_,
            watch_after: 0,
            title: file_without_ext,
            file_name: file_name_.clone(),
            thumbnail: None,
        };

        // TODO: Find a way to genrate a thumbnail

        list.push(new_movie);
    }
}
fn load_eps(dir: PathBuf, show_id_: Uuid, list: &mut Vec<NewEpisode>) {
    for ep in dir.read_dir().expect("read_dir movies failed") {
        let ep = ep.unwrap();

        // TODO: rewrite this!
        let file_without_ext: String = ep.path().file_stem().unwrap().to_str().unwrap().to_string();

        let file_name_: String = match ep.file_name().into_string() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let mut ep_number = match file_without_ext.parse::<i32>() {
            Ok(n) => n,
            Err(_) => continue,
        };
        let mut is_filler = false;
        let mut title = file_without_ext.clone();

        if file_without_ext.contains(" ") {
            let split: Vec<&str> = file_without_ext.split(" ").collect();

            for string in split {
                if string.chars().all(char::is_numeric) {
                    title = string.into();
                    ep_number = string.parse::<i32>().unwrap();
                } else {
                    if string == "(Filler)" {
                        is_filler = true
                    }
                }
            }
        }

        let new_episode = NewEpisode {
            id: Uuid::new_v4(),
            show_id: show_id_,
            title,
            number: ep_number,
            is_filler,
            file_name: file_name_.clone(),
            thumbnail: None,
        };

        // TODO: Find a way to genrate a thumbnail

        list.push(new_episode);
    }
}
