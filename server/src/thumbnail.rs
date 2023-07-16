use std::env;
use std::fs::DirEntry;
use std::process::Command;

/// Generate a thumbnail for a video
pub fn generate_thumbnail(file: DirEntry, ffmpeg_binary: &str) -> Vec<u8> {
    // TODO: cache thumbnails
    //       check if thumbnail already exists
    //       if not generate one
    let temp_dir = env::temp_dir();
    let output_thumbnail = temp_dir.join("thumbnail.jpg");

    println!(
        "[INFO] Generating thumbnail for '{}'",
        file.path().to_str().unwrap()
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
            output_thumbnail.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to generate thumbnail!");

    let thumbnail: Vec<u8> = match std::fs::read(output_thumbnail) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed to read thumbnail file, {e}");
            return Vec::new();
        }
    };

    return thumbnail;
}
