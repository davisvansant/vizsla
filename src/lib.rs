pub struct Archive {}

impl Archive {
    pub fn init() -> Archive {
        Archive {}
    }

    pub fn create(self) -> Result<(), std::io::Error> {
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
