pub struct FileMaker {
    dir: tempfile::TempDir,
}

impl FileMaker {
    pub fn new() -> Self {
        Self {
            dir: tempfile::tempdir().unwrap(),
        }
    }

    pub fn make(&self, name: &str, content: &str) -> std::path::PathBuf {
        let path = self.dir.path().join(name);
        std::fs::write(&path, content).unwrap();
        path
    }
}
