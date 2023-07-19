use std::fs::DirEntry;
use std::path::PathBuf;
use std::process::Command;

/// Generate a thumbnail for a video
pub fn generate_thumbnail(show_title: &str, file: DirEntry, ffmpeg_binary: &str) -> Vec<u8> {
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

        Command::new(ffmpeg_binary)
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
