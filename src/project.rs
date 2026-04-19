use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct Project {
    pub name: String,
    pub directory: PathBuf,
}

impl Project {
    /// Return the true path to the readme file of the project.
    /// This handles different common spellings for the readme file.
    fn get_description_path(&self) -> Option<PathBuf> {
        let supported_variants = [
            "README.md",
            "README",
            "Readme",
            "Readme.md",
            "readme",
            "readme.md",
        ];

        for supported_variant in supported_variants {
            let readme_path = self.directory.join(supported_variant);
            match fs::exists(&readme_path) {
                Ok(true) => {
                    return Some(readme_path);
                }
                _ => {}
            }
        }
        return None;
    }

    /// Get the description(readme contents) of the given project
    /// and return them. If no such description can be found, then
    /// None is returned.
    pub fn get_description(&self) -> Option<String> {
        let description_filepath = match self.get_description_path() {
            Some(path) => path,
            _ => {
                return None;
            }
        };

        let description = match fs::read_to_string(description_filepath) {
            Ok(content) => content,
            _ => {
                return None;
            }
        };

        return Some(description);
    }
}
