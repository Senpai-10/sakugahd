use common::models::anime::NewAnime;
use common::models::ending::NewEnding;
use common::models::episode::NewEpisode;
use common::models::movie::NewMovie;
use common::models::opening::NewOpening;
use common::schema::{anime, endings, episodes, movies, openings};
use diesel::dsl::exists;
use diesel::dsl::select;
use diesel::prelude::*;
use nanoid::nanoid;
use std::env;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::process;

pub const THUMBNAILS_CACHE_DIR: &str = "sakugahd_thumbnails";
const EPISODES_DIR_NAME: &str = "episodes";
const MOVIES_DIR_NAME: &str = "movies";
const OPENINGS_DIR_NAME: &str = "openings";
const ENDINGS_DIR_NAME: &str = "endings";

pub struct List {
    anime: Vec<NewAnime>,
    openings: Vec<NewOpening>,
    endings: Vec<NewEnding>,
    episodes: Vec<NewEpisode>,
    movies: Vec<NewMovie>,
}

pub struct Loader<'a> {
    ffmpegthumbnailer_binary: String,
    anime_directory: &'a Path,
    db_connection: &'a mut PgConnection,
    current_anime: String,
    lists: List,
}

impl<'a> Loader<'a> {
    pub fn new(anime_directory: &'a Path, db_connection: &'a mut PgConnection) -> Self {
        let ffmpegthumbnailer_binary =
            env::var("FFMPEGTHUMBNAILER_BINARY").unwrap_or("ffmpegthumbnailer".into());

        // Check if ffmpegthumbnailer exists
        match process::Command::new(&ffmpegthumbnailer_binary)
            .arg("-version")
            .output()
        {
            Ok(_) => {}
            Err(e) => {
                if let std::io::ErrorKind::NotFound = e.kind() {
                    error!(
                        "`{}` was not found! Can't generate thumbnails.",
                        &ffmpegthumbnailer_binary
                    )
                } else {
                    error!("Some error occurred when checking for ffmpegthumbnailer {e}");
                }

                error!("Exiting..");
                process::exit(1);
            }
        }

        Self {
            ffmpegthumbnailer_binary,
            anime_directory,
            db_connection,
            current_anime: String::new(),
            lists: List {
                anime: Vec::new(),
                openings: Vec::new(),
                endings: Vec::new(),
                episodes: Vec::new(),
                movies: Vec::new(),
            },
        }
    }

    fn generate_thumbnail(&self, file: DirEntry) -> String {
        let cache_dir: PathBuf = dirs::cache_dir().unwrap();
        let thumbnails_dir = cache_dir.join(THUMBNAILS_CACHE_DIR);

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

        let thumbnail_file_name = format!(
            "{}_{}_{}",
            self.current_anime,
            file.path().parent().unwrap().to_str().unwrap(),
            file.file_name().to_str().unwrap()
        );

        let hash_file_name = sha256::digest(thumbnail_file_name);
        let thumbnail_file = thumbnails_dir.join(format!("{}.jpeg", hash_file_name));

        if thumbnail_file.exists() {
            info!(
                "Thumbnail Found for ({}) {}!",
                self.current_anime,
                file.file_name().to_str().unwrap()
            )
        } else {
            info!(
                "Generating thumbnail for ({}) '{}'",
                self.current_anime,
                file.file_name().to_str().unwrap()
            );
            let thumbnail_size = "256";
            let time_to_seek = "10%";
            let quality = "8";

            process::Command::new(&self.ffmpegthumbnailer_binary)
                .args([
                    "-i",
                    file.path().to_str().unwrap(),
                    "-o",
                    thumbnail_file.to_str().unwrap(),
                    "-s",
                    thumbnail_size,
                    "-t",
                    time_to_seek,
                    "-q",
                    quality,
                ])
                .output()
                .expect("Failed to generate thumbnail!");
        }

        thumbnail_file
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    fn insert_into_database(&mut self) {
        diesel::insert_into(anime::dsl::anime)
            .values(&self.lists.anime)
            .execute(self.db_connection)
            .expect("Error saving anime");

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

    /// Check if a anime exists
    fn anime_exists(&mut self) -> bool {
        select(exists(
            anime::dsl::anime.filter(anime::title.eq(&self.current_anime)),
        ))
        .get_result::<bool>(self.db_connection)
        .expect("Failed to check if anime exists")
    }

    pub fn run(mut self) {
        // Load anime

        // check if anime_directory exists
        if !self.anime_directory.exists() {
            error!(
                "Anime directory '{}' does not exists!",
                self.anime_directory.to_str().unwrap()
            );

            process::exit(1);
        }

        for anime_dir in self
            .anime_directory
            .read_dir()
            .expect("read_dir anime_directory failed")
        {
            let anime_dir = anime_dir.unwrap();
            let anime_name: String = match anime_dir.file_name().into_string() {
                Ok(v) => v,
                Err(_) => continue,
            };

            if anime_dir.path().is_file() {
                // Skip files in the root of the anime dir
                continue;
            }

            self.current_anime = anime_name;

            let anime_exists = self.anime_exists();

            if !anime_exists {
                let cover = anime_dir.path().join("cover.png");

                let mut new_anime = NewAnime {
                    title: self.current_anime.clone(),
                    description: String::from("no description."),
                    format: None,
                    status: None,
                    season: None,
                    season_year: None,
                    cover: None,
                };

                if cover.exists() {
                    new_anime.cover = Some(cover.file_name().unwrap().to_str().unwrap().into());
                }

                self.lists.anime.push(new_anime);
            }

            self.load_episodes(&anime_dir, anime_exists);
            self.load_movies(&anime_dir, anime_exists);
            self.load_openings(&anime_dir, anime_exists);
            self.load_endings(&anime_dir, anime_exists);
        }
        self.insert_into_database();
    }

    fn load_episodes(&mut self, anime_path: &DirEntry, check_new: bool) {
        let episodes_directory = anime_path.path().join(EPISODES_DIR_NAME);

        if !episodes_directory.exists() {
            warn!(
                "'{}' Does not exists'",
                episodes_directory.to_str().unwrap()
            );
            return;
        }

        if check_new {
            info!("({}) Checking for new episodes", self.current_anime);
        } else {
            info!("({}) Loading Episodes", self.current_anime);
        }

        let file_names: Vec<String> = episodes::dsl::episodes
            .filter(episodes::anime_title.eq(&self.current_anime))
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

            let thumbnail_file_name = self.generate_thumbnail(episode);

            let new_episode = NewEpisode {
                id: nanoid!(),
                anime_title: self.current_anime.clone(),
                title,
                number: episode_number,
                is_filler,
                file_name: file_name.clone(),
                thumbnail_file_name,
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

    fn load_movies(&mut self, anime_path: &DirEntry, check_new: bool) {
        let movies_directory = anime_path.path().join(MOVIES_DIR_NAME);

        if !movies_directory.exists() {
            warn!("'{}' Does not exists'", movies_directory.to_str().unwrap());
            return;
        }

        if check_new {
            info!("({}) Checking for new movies", self.current_anime);
        } else {
            info!("({}) Loading Movies", self.current_anime);
        }

        let file_names: Vec<String> = movies::dsl::movies
            .filter(movies::anime_title.eq(&self.current_anime))
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

            let thumbnail_file_name = self.generate_thumbnail(movie);

            let new_movie = NewMovie {
                id: nanoid!(),
                anime_title: self.current_anime.clone(),
                title,
                watch_after: 0,
                number,
                file_name: file_name.clone(),
                thumbnail_file_name,
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

    fn load_openings(&mut self, anime_path: &DirEntry, check_new: bool) {
        let openings_directory = anime_path.path().join(OPENINGS_DIR_NAME);

        if !openings_directory.exists() {
            warn!(
                "'{}' Does not exists'",
                openings_directory.to_str().unwrap()
            );
            return;
        }

        if check_new {
            info!("({}) Checking for new openings", self.current_anime);
        } else {
            info!("({}) Loading Openings", self.current_anime);
        }

        let file_names: Vec<String> = openings::dsl::openings
            .filter(openings::anime_title.eq(&self.current_anime))
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

            let thumbnail_file_name = self.generate_thumbnail(opening);

            let new_opening = NewOpening {
                id: nanoid!(),
                anime_title: self.current_anime.clone(),
                title,
                number,
                file_name: file_name.clone(),
                thumbnail_file_name,
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

    fn load_endings(&mut self, anime_path: &DirEntry, check_new: bool) {
        let endings_directory = anime_path.path().join(ENDINGS_DIR_NAME);

        if !endings_directory.exists() {
            warn!("'{}' Does not exists'", endings_directory.to_str().unwrap());
            return;
        }

        if check_new {
            info!("({}) Checking for new endings", self.current_anime);
        } else {
            info!("({}) Loading endings", self.current_anime);
        }

        let file_names: Vec<String> = endings::dsl::endings
            .filter(endings::anime_title.eq(&self.current_anime))
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

            let thumbnail_file_name = self.generate_thumbnail(ending);

            let new_ending = NewEnding {
                id: nanoid!(),
                anime_title: self.current_anime.clone(),
                title,
                number,
                file_name: file_name.clone(),
                thumbnail_file_name,
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
