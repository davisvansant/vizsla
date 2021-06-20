use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::path::PathBuf;
use tar::Archive;
use tar::Builder;

pub struct Archiver {
    pub name: PathBuf,
    pub file_name: PathBuf,
}

impl Archiver {
    pub fn init(name: &str) -> Archiver {
        let mut path_buf = PathBuf::with_capacity(20);
        path_buf.push(name);
        let mut file_name = PathBuf::with_capacity(20);
        file_name.push(name);
        file_name.set_extension("tar.gz");

        Archiver {
            name: path_buf,
            file_name,
        }
    }

    pub fn create(&self) -> Result<(), std::io::Error> {
        let tar_gz = File::create(self.file_name.as_path())?;
        let format = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = Builder::new(format);
        let mut file = File::open("/etc/os-release")?;
        tar.append_file("os-release", &mut file)?;
        tar.finish()?;
        Ok(())
    }

    pub fn extract(&self) -> Result<(), std::io::Error> {
        let tar_gz = File::open(self.file_name.as_path())?;
        let gzip = GzDecoder::new(tar_gz);
        let mut tar = Archive::new(gzip);
        tar.unpack(self.name.as_path())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let test_archiver = Archiver::init("test_archive");
        assert_eq!(
            test_archiver.name.as_path(),
            std::path::Path::new("test_archive"),
        );
        assert_eq!(
            test_archiver.file_name.as_path(),
            std::path::Path::new("test_archive.tar.gz"),
        );
    }

    #[test]
    fn create() {
        let test_archive = Archiver::init("test_archive");
        test_archive.create().unwrap();
        let test_file = std::fs::File::open(test_archive.file_name.as_path()).unwrap();
        let test_file_metadata = test_file.metadata().unwrap();
        assert!(test_file_metadata.is_file());
        std::fs::remove_file(test_archive.file_name.as_path()).unwrap();
    }

    #[test]
    fn extract() {
        let test_archive = Archiver::init("test_archive");
        test_archive.create().unwrap();
        test_archive.extract().unwrap();
        let test_file = std::fs::File::open("./test_archive/os-release").unwrap();
        let test_file_metadata = test_file.metadata().unwrap();
        assert!(test_file_metadata.is_file());
        std::fs::remove_file(test_archive.file_name.as_path()).unwrap();
        std::fs::remove_file("./test_archive/os-release").unwrap();
        std::fs::remove_dir(test_archive.name.as_path()).unwrap();
    }
}
