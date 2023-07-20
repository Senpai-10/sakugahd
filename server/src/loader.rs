use common::models::ending::NewEnding;
use common::models::episode::NewEpisode;
use common::models::movie::NewMovie;
use common::models::opening::NewOpening;
use common::models::show::NewShow;
use common::schema::{endings, episodes, movies, openings, shows};
use diesel::dsl::exists;
use diesel::dsl::select;
use diesel::prelude::*;
use std::env;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::process;
use uuid::Uuid;

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
    current_show: String,
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
            current_show: String::new(),
            lists: List {
                shows: Vec::new(),
                openings: Vec::new(),
                endings: Vec::new(),
                episodes: Vec::new(),
                movies: Vec::new(),
            },
        }
    }

    fn generate_thumbnail(&self, file: DirEntry) -> Vec<u8> {
        let cache_dir: PathBuf = dirs::cache_dir().unwrap();
        let thumbnails_dir = cache_dir.join("sakugahd_thumbnails");

        if !thumbnails_dir.exists() {
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
            self.current_show,
            file.path().file_stem().unwrap().to_str().unwrap()
        ));

        if thumbnail_file.exists() {
            info!(
                "Thumbnail Found for ({}) {}!",
                self.current_show,
                file.file_name().to_str().unwrap()
            )
        } else {
            info!(
                "Generating thumbnail for ({}) '{}'",
                self.current_show,
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

        thumbnail
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
    fn show_exists(&mut self) -> bool {
        select(exists(
            shows::dsl::shows.filter(shows::title.eq(&self.current_show)),
        ))
        .get_result::<bool>(self.db_connection)
        .expect("Failed to check if show exists")
    }

    pub fn run(mut self) {
        // Load Shows

        // check if anime_directory exists
        if !self.anime_directory.exists() {
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

            self.current_show = show_name;

            let show_exists = self.show_exists();

            if !show_exists {
                let image = show_dir.path().join("image.png");
                let banner = show_dir.path().join("banner.png");

                let mut new_show = NewShow {
                    title: self.current_show.clone(),
                    description: String::from("no description."),
                    format: None,
                    status: None,
                    season: None,
                    season_year: None,
                    banner: None,
                    image: None,
                };

                new_show.image = match std::fs::read(image) {
                    Ok(bytes) => {
                        info!("Detected show image");
                        Some(bytes)
                    }
                    Err(_) => Some(Vec::new()),
                };

                new_show.banner = match std::fs::read(banner) {
                    Ok(bytes) => {
                        info!("Detected show banner image");
                        Some(bytes)
                    }
                    Err(_) => Some(Vec::new()),
                };

                self.lists.shows.push(new_show);
            }

            self.load_episodes(&show_dir, show_exists);
            self.load_movies(&show_dir, show_exists);
            self.load_openings(&show_dir, show_exists);
            self.load_endings(&show_dir, show_exists);
        }
        self.insert_into_database();
    }

    fn load_episodes(&mut self, show_path: &DirEntry, check_new: bool) {
        let episodes_directory = show_path.path().join(EPISODES_DIR_NAME);

        if !episodes_directory.exists() {
            warn!(
                "'{}' Does not exists'",
                episodes_directory.to_str().unwrap()
            );
            return;
        }

        if check_new {
            info!("({}) Checking for new episodes", self.current_show);
        } else {
            info!("({}) Loading Episodes", self.current_show);
        }

        let file_names: Vec<String> = episodes::dsl::episodes
            .filter(episodes::show_title.eq(&self.current_show))
            .select(episodes::file_name)
            .load(self.db_connection)
            .expect("Can't load episode file_names");

        for episode in episodes_directory
            .read_dir()
            .expect("Failed to read episodes directory")
        {
            let episode = episode.unwrap();

            if episode.path().is_dir() {
                // skip this entry. we only need files
                continue;
            }

            let file_name: String = episode.file_name().into_string().unwrap();

            if check_new && file_names.contains(&file_name) {
                continue;
            }

            let file_name_without_extension: String = episode
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            if !file_name.ends_with(".mp4") {
                warn!("'{}' is not a .mp4", &file_name);
                continue;
            }

            // Parse episode file name
            let mut episode_number = 0;
            let mut is_filler = false;
            let mut title = file_name_without_extension.clone();

            if file_name_without_extension.contains(' ') {
                let parts: Vec<_> = file_name_without_extension.split_whitespace().collect();

                for part in parts {
                    if part.chars().all(char::is_numeric) {
                        title = part.to_string();
                        episode_number = part.parse::<i32>().unwrap();
                    } else if part == "(Filler)" {
                        is_filler = true
                    }
                }
            } else {
                episode_number = file_name_without_extension.parse::<i32>().unwrap();
            }
            // end of parsing

            let thumbnail = self.generate_thumbnail(episode);

            let new_episode = NewEpisode {
                id: Uuid::new_v4(),
                show_title: self.current_show.clone(),
                title,
                number: episode_number,
                is_filler,
                file_name: file_name.clone(),
                thumbnail,
            };

            if check_new && !file_names.contains(&file_name) {
                info!("New episode detected: '{}'", file_name);
            }

            self.lists.episodes.push(new_episode);
        }

        if check_new && self.lists.episodes.is_empty() {
            info!("Nothing new.")
        }
    }

    fn load_movies(&mut self, show_path: &DirEntry, check_new: bool) {
        let movies_directory = show_path.path().join(MOVIES_DIR_NAME);

        if !movies_directory.exists() {
            warn!("'{}' Does not exists'", movies_directory.to_str().unwrap());
            return;
        }

        if check_new {
            info!("({}) Checking for new movies", self.current_show);
        } else {
            info!("({}) Loading Movies", self.current_show);
        }

        let file_names: Vec<String> = movies::dsl::movies
            .filter(movies::show_title.eq(&self.current_show))
            .select(movies::file_name)
            .load(self.db_connection)
            .expect("Can't load movies file_names");

        for movie in movies_directory
            .read_dir()
            .expect("Failed to read movies directory")
        {
            let movie = movie.unwrap();

            if movie.path().is_dir() {
                // skip this entry. we only need files
                continue;
            }

            let file_name: String = movie.file_name().into_string().unwrap();

            if check_new && file_names.contains(&file_name) {
                continue;
            }

            let file_name_without_extension: String = movie
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            if !file_name.ends_with(".mp4") {
                warn!("'{}' is not a .mp4", &file_name);
                continue;
            }

            let mut title = file_name_without_extension.clone();
            let number;

            if file_name_without_extension.contains(' ') {
                let mut split: Vec<&str> = file_name_without_extension.split(' ').collect();

                number = split[0].parse::<i32>().unwrap();
                split.remove(0);
                title = split.join(" ");
            } else {
                number = file_name_without_extension.parse::<i32>().unwrap()
            }

            let thumbnail = self.generate_thumbnail(movie);

            let new_movie = NewMovie {
                id: Uuid::new_v4(),
                show_title: self.current_show.clone(),
                title,
                watch_after: 0,
                number,
                file_name: file_name.clone(),
                thumbnail,
            };

            if check_new && !file_names.contains(&file_name) {
                info!("New movie detected: '{}'", file_name);
            }

            self.lists.movies.push(new_movie);
        }

        if check_new && self.lists.movies.is_empty() {
            info!("Nothing new.")
        }
    }

    fn load_openings(&mut self, show_path: &DirEntry, check_new: bool) {
        let openings_directory = show_path.path().join(OPENINGS_DIR_NAME);

        if !openings_directory.exists() {
            warn!(
                "'{}' Does not exists'",
                openings_directory.to_str().unwrap()
            );
            return;
        }

        if check_new {
            info!("({}) Checking for new openings", self.current_show);
        } else {
            info!("({}) Loading Openings", self.current_show);
        }

        let file_names: Vec<String> = openings::dsl::openings
            .filter(openings::show_title.eq(&self.current_show))
            .select(openings::file_name)
            .load(self.db_connection)
            .expect("Can't load openings file_names");

        for opening in openings_directory
            .read_dir()
            .expect("Failed to read openings directory")
        {
            let opening = opening.unwrap();

            if opening.path().is_dir() {
                // skip this entry. we only need files
                continue;
            }

            let file_name: String = opening.file_name().into_string().unwrap();

            if check_new && file_names.contains(&file_name) {
                continue;
            }

            let file_name_without_extension: String = opening
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            if !file_name.ends_with(".mp4") {
                warn!("'{}' is not a .mp4", &file_name);
                continue;
            }

            let mut title = file_name_without_extension.clone();
            let number;

            if file_name_without_extension.contains(' ') {
                let mut split: Vec<&str> = file_name_without_extension.split(' ').collect();

                number = split[0].parse::<i32>().unwrap();
                split.remove(0);
                title = split.join(" ");
            } else {
                number = file_name_without_extension.parse::<i32>().unwrap()
            }

            let thumbnail = self.generate_thumbnail(opening);

            let new_opening = NewOpening {
                id: Uuid::new_v4(),
                show_title: self.current_show.clone(),
                title,
                number,
                file_name: file_name.clone(),
                thumbnail,
            };

            if check_new && !file_names.contains(&file_name) {
                info!("New opening detected: '{}'", file_name);
            }

            self.lists.openings.push(new_opening);
        }

        if check_new && self.lists.openings.is_empty() {
            info!("Nothing new.")
        }
    }

    fn load_endings(&mut self, show_path: &DirEntry, check_new: bool) {
        let endings_directory = show_path.path().join(ENDINGS_DIR_NAME);

        if !endings_directory.exists() {
            warn!("'{}' Does not exists'", endings_directory.to_str().unwrap());
            return;
        }

        if check_new {
            info!("({}) Checking for new endings", self.current_show);
        } else {
            info!("({}) Loading endings", self.current_show);
        }

        let file_names: Vec<String> = endings::dsl::endings
            .filter(endings::show_title.eq(&self.current_show))
            .select(endings::file_name)
            .load(self.db_connection)
            .expect("Can't load endings file_names");

        for ending in endings_directory
            .read_dir()
            .expect("Failed to read endings directory")
        {
            let ending = ending.unwrap();

            if ending.path().is_dir() {
                // skip this entry. we only need files
                continue;
            }

            let file_name: String = ending.file_name().into_string().unwrap();

            if check_new && file_names.contains(&file_name) {
                continue;
            }

            let file_name_without_extension: String = ending
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            if !file_name.ends_with(".mp4") {
                warn!("'{}' is not a .mp4", &file_name);
                continue;
            }

            let mut title = file_name_without_extension.clone();
            let number;

            if file_name_without_extension.contains(' ') {
                let mut split: Vec<&str> = file_name_without_extension.split(' ').collect();

                number = split[0].parse::<i32>().unwrap();
                split.remove(0);
                title = split.join(" ");
            } else {
                number = file_name_without_extension.parse::<i32>().unwrap()
            }

            let thumbnail = self.generate_thumbnail(ending);

            let new_ending = NewEnding {
                id: Uuid::new_v4(),
                show_title: self.current_show.clone(),
                title,
                number,
                file_name: file_name.clone(),
                thumbnail,
            };

            if check_new && !file_names.contains(&file_name) {
                info!("New ending detected: '{}'", file_name);
            }

            self.lists.endings.push(new_ending);
        }

        if check_new && self.lists.endings.is_empty() {
            info!("Nothing new.")
        }
    }
}
