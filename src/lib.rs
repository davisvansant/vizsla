use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use tar::Builder;

pub struct Archive {}

impl Archive {
    pub fn init() -> Archive {
        Archive {}
    }

    pub fn create(self) -> Result<(), std::io::Error> {
        let tar_gz = File::create("some_name.tar.gz")?;
        let format = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = Builder::new(format);
        Ok(())
    }

    pub fn extract(self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        Archive::init();
    }

    #[test]
    fn create() {
        let test_archive = Archive::init();
        test_archive.create().unwrap();
    }

    #[test]
    fn extract() {
        let test_archive = Archive::init();
        test_archive.extract().unwrap();
    }
}
