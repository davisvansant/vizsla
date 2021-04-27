use flate2::read::GzDecoder;
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

    pub fn create(&self) -> Result<(), std::io::Error> {
        let tar_gz = File::create("some_name.tar.gz")?;
        let format = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = Builder::new(format);
        let mut file = File::open("/etc/os-release")?;
        tar.append_file("os-release", &mut file)?;
        tar.finish()?;
        Ok(())
    }

    pub fn extract(&self) -> Result<(), std::io::Error> {
        let tar_gz = File::open("some_name.tar.gz")?;
        let gzip = GzDecoder::new(tar_gz);
        let mut tar = Archive::new(gzip);
        tar.unpack("some_name")?;
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
        let test_file = std::fs::File::open("some_name.tar.gz").unwrap();
        let test_file_metadata = test_file.metadata().unwrap();
        assert_eq!(test_file_metadata.is_file(), true);
        std::fs::remove_file("some_name.tar.gz").unwrap();
    }

    #[test]
    fn extract() {
        let test_archive = Archiver::init();
        test_archive.create().unwrap();
        test_archive.extract().unwrap();
        let test_file = std::fs::File::open("./some_name/os-release").unwrap();
        let test_file_metadata = test_file.metadata().unwrap();
        assert_eq!(test_file_metadata.is_file(), true);
        std::fs::remove_file("some_name.tar.gz").unwrap();
        std::fs::remove_file("./some_name/os-release").unwrap();
        std::fs::remove_dir("./some_name/").unwrap();
    }
}
