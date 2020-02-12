use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FilesystemShell {}

impl FilesystemShell {
    pub fn new() -> Self {
        unimplemented!()
    }
}

impl super::Shell for FilesystemShell {
    fn name(&self) -> String {
        unimplemented!()
    }

    fn homedir(&self) -> Option<PathBuf> {
        unimplemented!()
    }
}
