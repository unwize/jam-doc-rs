use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use anyhow::Result;

const VALID_TRACK_EXTENSIONS: [&str; 5] = ["mp3", "wav", "flac", "ogg", "aac"];

/// Check if a track's extension matches the allowed audio extensions
fn is_valid_track(path: &PathBuf) -> bool {
    VALID_TRACK_EXTENSIONS.iter().any(|ext| {
        let e = path.extension();

        if let Some(e) = e {
            return &e == ext;
        }

        false
    })
}

/// Recusively walk a path and its sub-dirs, returning a vec containing all valid paths to audio files
pub fn scan_dir(path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let mut entries: Vec<PathBuf> = vec![];

    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        match path.is_dir() {
            true => {
                let mut sub_dir = scan_dir(path)?;
                entries.append(&mut sub_dir);
            }
            false => {
                if is_valid_track(&path) {
                    entries.push(path);
                }
            }
        }
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_mp3() {
        let p = PathBuf::from(r"C:/dev/jam-doc-rs/song.mp3");
        assert!(is_valid_track(&p));
    }

    #[test]
    fn is_not_valid_mp4() {
        let p = PathBuf::from(r"C:/dev/jam-doc-rs/song.mp4");
        assert!(!is_valid_track(&p));
    }

    #[test]
    fn is_not_valid_dir() {
        let p = PathBuf::from(r"C:/dev/jam-doc-rs/");
        assert!(!is_valid_track(&p));
    }
}
