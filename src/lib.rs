use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use tar::Archive;
use tar::Builder;

pub struct Archiver {}

impl Archiver {
    pub fn init() -> Archiver {
        Archiver {}
    }

    pub fn create(self) -> Result<(), std::io::Error> {
        let tar_gz = File::create("some_name.tar.gz")?;
        let format = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = Builder::new(format);
        Ok(())
    }

    pub fn extract(self) -> Result<(), std::io::Error> {
        let tar_gz = File::open("some_name.tar.gz")?;
        let gzip = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = Archive::new(gzip);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        Archiver::init();
    }

    #[test]
    fn create() {
        let test_archive = Archiver::init();
        test_archive.create().unwrap();
    }

    #[test]
    fn extract() {
        let test_archive = Archiver::init();
        test_archive.extract().unwrap();
    }
}
