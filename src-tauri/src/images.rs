use crate::error::{AppError, AppResult};
use std::path::{Component, Path};
use uuid::Uuid;

const ALLOWED_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "webp", "gif"];

pub enum Kind {
    Cover,
    Screenshot,
}

impl Kind {
    fn folder(&self) -> &'static str {
        match self {
            Kind::Cover => "covers",
            Kind::Screenshot => "screenshots",
        }
    }
}

pub fn import(images_dir: &Path, kind: Kind, source: &Path) -> AppResult<String> {
    let ext = source
        .extension() // Option<&OsStr>
        .and_then(|e| e.to_str()) // Option<&str>
        .map(|e| e.to_ascii_lowercase()) // Option<String>
        .ok_or_else(|| AppError::Invalid("that file has no extension".into()))?;

    if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) {
        return Err(AppError::Invalid(format!("'.{ext}' images aren't supported")));
    }

    let relative = format!("{}/{}.{}", kind.folder(), Uuid::new_v4(), ext);
    let target = images_dir.join(&relative);
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::copy(source, &target)?;
    Ok(relative)
}

fn is_inside(relative: &str) -> bool {
    Path::new(relative)
        .components()
        .all(|c| matches!(c, Component::Normal(_)))
}

pub fn remove(images_dir: &Path, relative: &str) {
    if relative.starts_with("placeholder:") || !is_inside(relative) {
        return;
    }
    let _ = std::fs::remove_file(images_dir.join(relative));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn imports_an_image_into_the_right_folder() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("shot.PNG");
        fs::write(&source, b"fake image bytes").unwrap();
        let images = dir.path().join("images");

        let relative = import(&images, Kind::Screenshot, &source).unwrap();

        assert!(relative.starts_with("screenshots/"));
        assert!(relative.ends_with(".png"));
        assert_eq!(fs::read(images.join(&relative)).unwrap(), b"fake image bytes");
    }

    #[test]
    fn rejects_unsupported_and_missing_extensions() {
        let dir = tempfile::tempdir().unwrap();
        let text = dir.path().join("notes.txt");
        let bare = dir.path().join("picture");
        fs::write(&text, b"x").unwrap();
        fs::write(&bare, b"x").unwrap();

        assert!(matches!(import(dir.path(), Kind::Cover, &text), Err(AppError::Invalid(_))));
        assert!(matches!(import(dir.path(), Kind::Cover, &bare), Err(AppError::Invalid(_))));
    }

    #[test]
    fn remove_deletes_the_file_and_tolerates_a_missing_one() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("a.jpg");
        fs::write(&source, b"x").unwrap();
        let images = dir.path().join("images");
        let relative = import(&images, Kind::Cover, &source).unwrap();

        remove(&images, &relative);
        assert!(!images.join(&relative).exists());
        remove(&images, &relative); // second time: nothing to delete, no problem
    }

    #[test]
    fn remove_refuses_paths_that_climb_out_of_the_folder() {
        let dir = tempfile::tempdir().unwrap();
        let outside = dir.path().join("outside.txt");
        fs::write(&outside, b"keep me").unwrap();
        let images = dir.path().join("images");
        fs::create_dir_all(&images).unwrap();

        remove(&images, "../outside.txt");

        assert!(outside.exists());
    }
}