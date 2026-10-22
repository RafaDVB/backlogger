use crate::error::{AppError, AppResult};
use std::path::{Component, Path};
use uuid::Uuid;

const ALLOWED_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "webp", "gif"];
const MAX_IMAGE_BYTES: usize = 25 * 1024 * 1024;

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

fn checked_extension(extension: &str) -> AppResult<String> {
    let ext = extension.trim_start_matches('.').to_ascii_lowercase();
    if ALLOWED_EXTENSIONS.contains(&ext.as_str()) {
        Ok(ext)
    } else {
        Err(AppError::Invalid(format!(
            "'.{ext}' images aren't supported"
        )))
    }
}

fn new_relative_path(kind: &Kind, ext: &str) -> String {
    format!("{}/{}.{}", kind.folder(), Uuid::new_v4(), ext)
}

pub fn import(images_dir: &Path, kind: Kind, source: &Path) -> AppResult<String> {
    let ext = source
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| AppError::Invalid("that file has no extension".into()))?;
    let ext = checked_extension(ext)?;

    let relative = new_relative_path(&kind, &ext);
    let target = images_dir.join(&relative);
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::copy(source, &target)?;
    Ok(relative)
}

pub fn import_bytes(
    images_dir: &Path,
    kind: Kind,
    extension: &str,
    bytes: &[u8],
) -> AppResult<String> {
    if bytes.is_empty() {
        return Err(AppError::Invalid("the image is empty".into()));
    }
    if bytes.len() > MAX_IMAGE_BYTES {
        return Err(AppError::Invalid(
            "that image is too large (the limit is 25 MB)".into(),
        ));
    }
    let ext = checked_extension(extension)?;

    let relative = new_relative_path(&kind, &ext);
    let target = images_dir.join(&relative);
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&target, bytes)?;
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
        assert_eq!(
            fs::read(images.join(&relative)).unwrap(),
            b"fake image bytes"
        );
    }

    #[test]
    fn rejects_unsupported_and_missing_extensions() {
        let dir = tempfile::tempdir().unwrap();
        let text = dir.path().join("notes.txt");
        let bare = dir.path().join("picture");
        fs::write(&text, b"x").unwrap();
        fs::write(&bare, b"x").unwrap();

        assert!(matches!(
            import(dir.path(), Kind::Cover, &text),
            Err(AppError::Invalid(_))
        ));
        assert!(matches!(
            import(dir.path(), Kind::Cover, &bare),
            Err(AppError::Invalid(_))
        ));
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

    #[test]
    fn imports_bytes_into_the_screenshots_folder() {
        let dir = tempfile::tempdir().unwrap();
        let relative = import_bytes(dir.path(), Kind::Screenshot, "PNG", b"pixels").unwrap();

        assert!(relative.starts_with("screenshots/"));
        assert!(relative.ends_with(".png"));
        assert_eq!(
            std::fs::read(dir.path().join(&relative)).unwrap(),
            b"pixels"
        );
    }

    #[test]
    fn rejects_empty_oversized_and_unsupported_bytes() {
        let dir = tempfile::tempdir().unwrap();
        assert!(matches!(
            import_bytes(dir.path(), Kind::Cover, "png", b""),
            Err(AppError::Invalid(_))
        ));
        assert!(matches!(
            import_bytes(dir.path(), Kind::Cover, "exe", b"x"),
            Err(AppError::Invalid(_))
        ));
        let huge = vec![0u8; MAX_IMAGE_BYTES + 1];
        assert!(matches!(
            import_bytes(dir.path(), Kind::Cover, "png", &huge),
            Err(AppError::Invalid(_))
        ));
    }
}
