use crate::loaders::anime::THUMBNAILS_CACHE_DIR;
use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};
use urlencoding::decode;

#[get("/thumbnail/<file_name>")]
pub async fn thumbnail(file_name: String) -> NamedFile {
    let decoded_file_name: String = decode(&file_name).expect("UTF-8").to_string();
    let cache_dir: PathBuf = dirs::cache_dir().unwrap();
    let thumbnails_dir = cache_dir.join(THUMBNAILS_CACHE_DIR);

    let file_path = Path::new(&thumbnails_dir).join(decoded_file_name);
    NamedFile::open(&file_path)
        .await
        .expect("Failed to get thumbnail")
}
