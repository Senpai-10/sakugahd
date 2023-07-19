use crate::models::ending::NewEnding;
use crate::models::episode::NewEpisode;
use crate::models::movie::NewMovie;
use crate::models::opening::NewOpening;
use crate::models::show::{NewShow, Show};
use crate::schema::{endings, episodes, movies, openings, shows};
use crate::thumbnail::generate_thumbnail;
use diesel::prelude::*;
use std::env;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub struct Loader<'a> {
    ffmpeg_binary: String,
    anime_directory: &'a Path,
    db_connection: &'a mut PgConnection,
}

impl<'a> Loader<'a> {
    pub fn new(anime_directory: &'a Path, db_connection: &'a mut PgConnection) -> Self {
        let ffmpeg_binary = env::var("FFMPEG_BINARY").unwrap_or("ffmpeg".into());

        // Check if ffmpeg exists
        match std::process::Command::new(&ffmpeg_binary)
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
                std::process::exit(1);
            }
        }

        Self {
            ffmpeg_binary,
            anime_directory,
            db_connection,
        }
    }

    pub fn run(&self) {
        // Load Shows
    }
}
