use common::models::{chapter::NewChapter, manga::NewManga, page::NewPage};
use common::schema::{chapters, manga, pages};
use diesel::dsl::exists;
use diesel::dsl::select;
use diesel::prelude::*;
use nanoid::nanoid;
use std::fs::DirEntry;
use std::path::Path;
use std::process;

struct Lists {
    manga: Vec<NewManga>,
    chapters: Vec<NewChapter>,
    pages: Vec<NewPage>,
}

pub struct MangaLoader<'a> {
    manga_directory: &'a Path,
    db_connection: &'a mut PgConnection,
    current_manga: String,
    lists: Lists,
}

impl<'a> MangaLoader<'a> {
    pub fn new(manga_directory: &'a Path, db_connection: &'a mut PgConnection) -> Self {
        Self {
            manga_directory,
            db_connection,
            current_manga: String::new(),
            lists: Lists {
                manga: Vec::new(),
                chapters: Vec::new(),
                pages: Vec::new(),
            },
        }
    }

    fn manga_exists(&mut self) -> bool {
        select(exists(
            manga::dsl::manga.filter(manga::title.eq(&self.current_manga)),
        ))
        .get_result::<bool>(self.db_connection)
        .expect("Failed to check if manga exists")
    }

    fn chapter_exists(&mut self, title: &String, num: &String) -> bool {
        select(exists(
            chapters::dsl::chapters
                .filter(chapters::manga_title.eq(&self.current_manga))
                .filter(chapters::title.eq(title))
                .filter(chapters::number.eq(num)),
        ))
        .get_result::<bool>(self.db_connection)
        .expect("Failed to check if chapter exists")
    }

    fn page_exists(&mut self, chapter_id: &String, page_number: &i32) -> bool {
        select(exists(
            pages::dsl::pages
                .filter(pages::manga_title.eq(&self.current_manga))
                .filter(pages::chapter_id.eq(chapter_id))
                .filter(pages::number.eq(page_number)),
        ))
        .get_result::<bool>(self.db_connection)
        .expect("Failed to check if page exists")
    }

    fn insert_into_database(&mut self) {
        diesel::insert_into(manga::dsl::manga)
            .values(&self.lists.manga)
            .execute(self.db_connection)
            .expect("Error saving manga");

        diesel::insert_into(chapters::dsl::chapters)
            .values(&self.lists.chapters)
            .execute(self.db_connection)
            .expect("Error saving chapters");

        diesel::insert_into(pages::dsl::pages)
            .values(&self.lists.pages)
            .execute(self.db_connection)
            .expect("Error saving pages");
    }

    pub fn run(mut self) {
        if !self.manga_directory.exists() {
            error!(
                "Manga directory '{}' does not exists!",
                self.manga_directory.to_str().unwrap()
            );

            process::exit(1);
        }

        println!("Loading manga!");

        for manga_dir in self
            .manga_directory
            .read_dir()
            .expect("read_dir manga_directory failed")
        {
            let manga_dir = manga_dir.unwrap();
            let manga_name: String = match manga_dir.file_name().into_string() {
                Ok(v) => v,
                Err(_) => continue,
            };

            if manga_dir.path().is_file() {
                // Skip files in the root of the manga dir
                continue;
            }

            self.current_manga = manga_name;

            let manga_exists = self.manga_exists();

            if !manga_exists {
                let cover = manga_dir.path().join("cover.png");

                let mut new_manga = NewManga {
                    title: self.current_manga.clone(),
                    description: String::from("no description."),
                    cover: None,
                };

                if cover.exists() {
                    new_manga.cover = Some(cover.file_name().unwrap().to_str().unwrap().into());
                }

                self.lists.manga.push(new_manga);
            }

            // Load chapters
            // Load pages of chapters
            self.load_chapters(&manga_dir);
        }

        self.insert_into_database();
    }

    fn load_chapters(&mut self, manga_path: &DirEntry) {
        println!("Loading chapters");

        let chapters_directory = manga_path.path().join("chapters");

        for chapter_dir in chapters_directory
            .read_dir()
            .expect("Failed to read chapters dir")
        {
            let chapter = chapter_dir.unwrap();
            let chapter_name: String = match chapter.file_name().into_string() {
                Ok(v) => v,
                Err(_) => continue,
            };

            let parsed_title = parse_chapter_title(chapter_name);

            let (num, title) = match parsed_title {
                Some(v) => v,
                None => {
                    continue;
                }
            };

            let mut chapter_id: Option<String> = None;

            if self.chapter_exists(&title, &num) {
                chapter_id = Some(
                    chapters::dsl::chapters
                        .filter(chapters::manga_title.eq(&self.current_manga))
                        .filter(chapters::title.eq(&title))
                        .filter(chapters::number.eq(&num))
                        .select(chapters::id)
                        .first(self.db_connection)
                        .expect("Failed to get chapter id"),
                );
            } else {
                let id = nanoid!();
                chapter_id = Some(id.clone());

                let new_chapter = NewChapter {
                    id,
                    manga_title: self.current_manga.clone(),
                    title,
                    number: num,
                };

                self.lists.chapters.push(new_chapter)
            }

            // Load pages
            if let Some(ch_id) = chapter_id {
                self.load_pages(chapter, ch_id);
            }
        }
    }

    fn load_pages(&mut self, chapter_dir: DirEntry, chapter_id: String) {
        for page in chapter_dir
            .path()
            .read_dir()
            .expect("Failed to read chapter dir")
        {
            let page_number: i32 = page
                .as_ref()
                .unwrap()
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            if self.page_exists(&chapter_id, &page_number) {
                continue;
            }

            let new_page = NewPage {
                id: nanoid!(),
                manga_title: self.current_manga.clone(),
                chapter_id: chapter_id.clone(),
                number: page_number,
                file_name: String::from(
                    page.unwrap().path().file_name().unwrap().to_str().unwrap(),
                ),
            };

            self.lists.pages.push(new_page)
        }
    }
}

// Just a helper function
fn parse_chapter_title(chapter_name: String) -> Option<(String, String)> {
    let mut d: Vec<&str> = chapter_name.split(" ").collect();

    // Because the first item is going to be the chapter number
    // and the rest is the title
    if d.len() < 2 {
        return None;
    }

    let chapter_number = d[0].to_string();
    d.remove(0);
    let chapter_title = d.join(" ");

    Some((chapter_number, chapter_title))
}
