use crate::db::establish_connection;
use common::models::chapter::Chapter;
use common::models::manga::Manga;
use common::models::page::Page;
use common::schema;
use diesel::prelude::*;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use std::path::Path;
use urlencoding::decode;

#[get("/manga")]
pub fn manga() -> Json<Vec<Manga>> {
    let mut conn = establish_connection();

    Json(
        schema::manga::dsl::manga
            .order(schema::manga::title)
            .load(&mut conn)
            .expect("Can't load manga"),
    )
}

#[get("/manga/<title>")]
pub fn manga_one(title: String) -> Json<Manga> {
    let mut conn = establish_connection();

    Json(
        schema::manga::dsl::manga
            .filter(schema::manga::title.eq(&title))
            .first(&mut conn)
            .expect("Can't load anime"),
    )
}

#[get("/manga/<title>/genres")]
pub fn manga_genres(title: String) -> Json<Vec<String>> {
    let mut conn = establish_connection();

    Json(
        schema::manga_genres::dsl::manga_genres
            .filter(schema::manga_genres::manga_title.eq(&title))
            .select(schema::manga_genres::genre_name)
            .load(&mut conn)
            .expect("Can't load genres"),
    )
}

#[get("/manga/<title>/themes")]
pub fn manga_themes(title: String) -> Json<Vec<String>> {
    let mut conn = establish_connection();

    Json(
        schema::manga_themes::dsl::manga_themes
            .filter(schema::manga_themes::manga_title.eq(&title))
            .select(schema::manga_themes::theme_name)
            .load(&mut conn)
            .expect("Can't load themes"),
    )
}

#[get("/manga/<title>/cover/<file_name>")]
pub async fn get_cover(title: String, file_name: String) -> NamedFile {
    let decoded_title: String = decode(&title).expect("UTF-8").to_string();
    let decoded_file_name: String = decode(&file_name).expect("UTF-8").to_string();

    let env_manga_directory =
        std::env::var("MANGA_DIRECTORY").expect("MANGA_DIRECTORY must be set");
    let manga_directory = Path::new(&env_manga_directory);

    let file_path = Path::new(&manga_directory)
        .join(decoded_title)
        .join(decoded_file_name);
    NamedFile::open(&file_path)
        .await
        .expect("Failed to get cover image")
}

#[get("/manga/<title>/chapters")]
pub fn manga_chapters(title: String) -> Json<Vec<Chapter>> {
    let mut conn = establish_connection();

    Json(
        schema::chapters::dsl::chapters
            .filter(schema::chapters::manga_title.eq(&title))
            .order(schema::chapters::number)
            .load(&mut conn)
            .expect("Can't load chapters"),
    )
}

#[get("/manga/<title>/chapters/<id>")]
pub fn manga_chapter_pages(title: String, id: String) -> Json<Vec<Page>> {
    let mut conn = establish_connection();

    Json(
        schema::pages::dsl::pages
            .filter(schema::pages::chapter_id.eq(&id))
            .order(schema::pages::number)
            .load(&mut conn)
            .expect("Can't load pages"),
    )
}

#[get("/page/<id>")]
pub async fn manga_page(id: String) -> Option<NamedFile> {
    let mut conn = establish_connection();

    let page_query: Result<Page, _> = schema::pages::table
        .filter(schema::pages::dsl::id.eq(&id))
        .select(Page::as_select())
        .get_result(&mut conn);

    if let Ok(page) = page_query {
        let chapter_query: Result<Chapter, _> = schema::chapters::table
            .filter(schema::chapters::dsl::id.eq(&page.chapter_id))
            .select(Chapter::as_select())
            .get_result(&mut conn);

        if let Ok(chapter) = chapter_query {
            let env_manga_directory =
                std::env::var("MANGA_DIRECTORY").expect("MANGA_DIRECTORY must be set");
            let manga_directory = Path::new(&env_manga_directory);

            let file_path = Path::new(&manga_directory)
                .join(chapter.manga_title)
                .join("chapters")
                .join(format!("{} {}", chapter.number, chapter.title))
                .join(page.file_name);

            println!("{:?}", file_path);

            return Some(
                NamedFile::open(&file_path)
                    .await
                    .expect("Failed to get page"),
            );
        }
    }

    None
}
