use wasm_bindgen::prelude::*;
use walkdir::WalkDir;

#[wasm_bindgen]
pub struct EnvFinder {
    exclude_dirs: Vec<String>,
}

#[wasm_bindgen]
impl EnvFinder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        EnvFinder {
            exclude_dirs: vec!["node_modules".to_string(), ".git".to_string()],
        }
    }

    #[wasm_bindgen(js_name = addExcludeDir)]
    pub fn add_exclude_dir(&mut self, dir: String) {
        self.exclude_dirs.push(dir);
    }

    #[wasm_bindgen(js_name = findEnvFiles)]
    pub fn find_env_files(&self, dir_path: &str) -> Vec<String> {
        let mut matched_paths = Vec::new();
        
        for entry in WalkDir::new(dir_path)
            .into_iter()
            .filter_entry(|e| {
                !e.path()
                    .file_name()
                    .and_then(|s| s.to_str())
                    .map(|s| self.exclude_dirs.contains(&s.to_string()))
                    .unwrap_or(false)
            })
        {
            if let Ok(entry) = entry {
                if entry.file_type().is_file() {
                    let path = entry.path();
                    if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                        if file_name.starts_with(".env") {
                            if let Some(path_str) = path.to_str() {
                                matched_paths.push(path_str.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        matched_paths
    }

    #[wasm_bindgen(js_name = findWithPattern)]
    pub fn find_with_pattern(&self, dir_path: &str, pattern: &str) -> Vec<String> {
        let mut matched_paths = Vec::new();
        
        for entry in WalkDir::new(dir_path)
            .into_iter()
            .filter_entry(|e| {
                !e.path()
                    .file_name()
                    .and_then(|s| s.to_str())
                    .map(|s| self.exclude_dirs.contains(&s.to_string()))
                    .unwrap_or(false)
            })
        {
            if let Ok(entry) = entry {
                if entry.file_type().is_file() {
                    let path = entry.path();
                    if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                        if file_name.contains(pattern) {
                            if let Some(path_str) = path.to_str() {
                                matched_paths.push(path_str.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        matched_paths
    }
}

#[wasm_bindgen(js_name = findEnvFiles)]
pub fn find_env_files(dir_path: &str) -> Vec<String> {
    let finder = EnvFinder::new();
    finder.find_env_files(dir_path)
}