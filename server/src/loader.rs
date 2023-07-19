use crate::models::ending::NewEnding;
use crate::models::episode::NewEpisode;
use crate::models::movie::NewMovie;
use crate::models::opening::NewOpening;
use crate::models::show::{NewShow, Show};
use crate::schema::{endings, episodes, movies, openings, shows};
use diesel::prelude::*;
use std::env;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::process::Command;
use uuid::Uuid;

pub struct List {
    shows: Vec<NewShow>,
    openings: Vec<NewOpening>,
    endings: Vec<NewEnding>,
    episodes: Vec<NewEpisode>,
    movies: Vec<NewMovie>,
}

pub struct Loader<'a> {
    ffmpeg_binary: String,
    anime_directory: &'a Path,
    db_connection: &'a mut PgConnection,
    lists: List,
}

impl<'a> Loader<'a> {
    pub fn new(anime_directory: &'a Path, db_connection: &'a mut PgConnection) -> Self {
        let ffmpeg_binary = env::var("FFMPEG_BINARY").unwrap_or("ffmpeg".into());

        // Check if ffmpeg exists
        match Command::new(&ffmpeg_binary).arg("-version").output() {
            Ok(_) => {}
            Err(e) => {
                if let std::io::ErrorKind::NotFound = e.kind() {
                    error!(
                        "`{}` was not found! Can't generate thumbnails.",
                        &ffmpeg_binary
                    )
                } else {
                    error!("Some error occurred when checking for ffmpeg {e}");
                }

                error!("Exiting..");
                std::process::exit(1);
            }
        }

        Self {
            ffmpeg_binary,
            anime_directory,
            db_connection,
            lists: List {
                shows: Vec::new(),
                openings: Vec::new(),
                endings: Vec::new(),
                episodes: Vec::new(),
                movies: Vec::new(),
            },
        }
    }

    fn generate_thumbnail(&self, show_title: &str, file: DirEntry) -> Vec<u8> {
        let cache_dir: PathBuf = dirs::cache_dir().unwrap();
        let thumbnails_dir = cache_dir.join("sakugahd_thumbnails");

        if thumbnails_dir.exists() == false {
            match std::fs::create_dir_all(&thumbnails_dir) {
                Ok(_) => {
                    info!(
                        "Created thumbnail cache directory in '{}'",
                        &thumbnails_dir.to_str().unwrap()
                    )
                }
                Err(e) => {
                    error!("Can't create '{}', {e}", thumbnails_dir.to_str().unwrap());
                    std::process::exit(1);
                }
            };
        }

        let thumbnail_file = thumbnails_dir.join(format!(
            "{}_{}.jpg",
            show_title,
            file.path().file_stem().unwrap().to_str().unwrap()
        ));

        if thumbnail_file.exists() == true {
            info!(
                "Thumbnail Found for ({show_title}) {}!",
                file.file_name().to_str().unwrap()
            )
        } else {
            info!(
                "Generating thumbnail for ({show_title}) '{}'",
                file.file_name().to_str().unwrap()
            );

            Command::new(&self.ffmpeg_binary)
                .args([
                    "-nostdin",
                    "-y",
                    "-i",
                    file.path().to_str().unwrap(),
                    "-vf",
                    "thumbnail",
                    "-frames:v",
                    "1",
                    thumbnail_file.to_str().unwrap(),
                ])
                .output()
                .expect("Failed to generate thumbnail!");
        }

        let thumbnail: Vec<u8> = match std::fs::read(&thumbnail_file) {
            Ok(bytes) => bytes,
            Err(e) => {
                error!(
                    "Failed to read thumbnail file (video file might be broken) '{}', {e}",
                    thumbnail_file.to_str().unwrap()
                );
                return Vec::new();
            }
        };

        return thumbnail;
    }
    fn insert_into_database(&mut self) {
        diesel::insert_into(shows::dsl::shows)
            .values(&self.lists.shows)
            .execute(self.db_connection)
            .expect("Error saving shows");

        diesel::insert_into(openings::dsl::openings)
            .values(&self.lists.openings)
            .execute(self.db_connection)
            .expect("Error saving openings");

        diesel::insert_into(endings::dsl::endings)
            .values(&self.lists.endings)
            .execute(self.db_connection)
            .expect("Error saving endings");

        diesel::insert_into(episodes::dsl::episodes)
            .values(&self.lists.episodes)
            .execute(self.db_connection)
            .expect("Error saving episodes");

        diesel::insert_into(movies::dsl::movies)
            .values(&self.lists.movies)
            .execute(self.db_connection)
            .expect("Error saving movies");
    }

    pub fn run(self) {
        // Load Shows
    }
}
