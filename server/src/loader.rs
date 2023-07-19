use crate::models::ending::NewEnding;
use crate::models::episode::NewEpisode;
use crate::models::movie::NewMovie;
use crate::models::opening::NewOpening;
use crate::models::show::{NewShow, Show};
use crate::schema::{endings, episodes, movies, openings, shows};
use diesel::dsl::exists;
use diesel::dsl::select;
use diesel::prelude::*;
use std::env;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::process;
use uuid::Uuid;

const SHOW_BANNER_FILE_NAME_START: &str = "banner";
const SHOW_IMAGE_FILE_NAME_START: &str = "image";
const EPISODES_DIR_NAME: &str = "episodes";
const MOVIES_DIR_NAME: &str = "movies";
const OPENINGS_DIR_NAME: &str = "openings";
const ENDINGS_DIR_NAME: &str = "endings";

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
        match process::Command::new(&ffmpeg_binary)
            .arg("-version")
            .output()
        {
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
                process::exit(1);
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

            process::Command::new(&self.ffmpeg_binary)
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

    /// Check if a show exists
    fn show_exists(&mut self, title: &String) -> bool {
        select(exists(shows::dsl::shows.filter(shows::title.eq(title))))
            .get_result::<bool>(self.db_connection)
            .expect("Failed to check if show exists")
    }

    pub fn run(mut self) {
        // Load Shows

        // check if anime_directory exists
        if self.anime_directory.exists() == false {
            error!(
                "Anime directory '{}' does not exists!",
                self.anime_directory.to_str().unwrap()
            );

            process::exit(1);
        }

        for show_dir in self
            .anime_directory
            .read_dir()
            .expect("read_dir anime_directory failed")
        {
            let show_dir = show_dir.unwrap();
            let show_name: String = match show_dir.file_name().into_string() {
                Ok(v) => v,
                Err(_) => continue,
            };

            if show_dir.path().is_file() {
                // Skip files in the root of the anime dir
                continue;
            }

            let show_exists = self.show_exists(&show_name);

            if show_exists {
                // check if theres a new episode/movies/openings/endings
            } else {
                // add episodes/movies/openings/endings
            }

            info!("\"{show_name}\" Loading Episodes");
            self.load_episodes(&show_dir, show_exists);

            info!("\"{show_name}\" Loading Movies");
            self.load_movies(&show_dir, show_exists);

            info!("\"{show_name}\" Loading Openings");
            self.load_openings(&show_dir, show_exists);

            info!("\"{show_name}\" Loading Endings");
            self.load_endings(&show_dir, show_exists);
        }
    }

    fn load_episodes(&mut self, show_path: &DirEntry, check_new: bool) {}

    fn load_movies(&mut self, show_path: &DirEntry, check_new: bool) {}

    fn load_openings(&mut self, show_path: &DirEntry, check_new: bool) {}

    fn load_endings(&mut self, show_path: &DirEntry, check_new: bool) {}
}
