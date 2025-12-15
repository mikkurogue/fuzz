use std::path::Path;

use super::{Location, TargetResult};

pub struct FileAnalysis;
impl FileAnalysis {
    /// Opens a file and reads its contents into a string
    pub fn open_file(input_file: &String) -> anyhow::Result<String> {
        let file = std::fs::read_to_string(input_file)?;
        Ok(file)
    }

    /// Walks current directory recursively and analyzes each file for the target text
    /// and parses this to a hashmap of results.
    /// For each file a TargetResult is created and stored in the hashmap with the file name as key.
    /// If multiple occurrences are found in a file, only the last one is stored. Maybe this should
    /// be a vec instead
    pub fn results(target_text: String) -> Vec<TargetResult> {
        let walker = walkdir::WalkDir::new(".").into_iter();

        let mut results = Vec::new();
        let ignored = Self::read_gitignore();

        for entry in walker.filter_map(|e| e.ok()) {
            if ignored
                .iter()
                .any(|ig| entry.path().to_string_lossy().contains(ig))
            {
                continue;
            }

            if entry.file_type().is_file() {
                let curr_file_analysis =
                    Self::analyze(entry.path().to_string_lossy().to_string(), &target_text);

                for res in curr_file_analysis {
                    results.push(res);
                }
            }
        }

        results
    }

    fn read_gitignore() -> Vec<String> {
        let mut ignores = Vec::new();

        if let Ok(content) = Self::open_file(&".gitignore".to_string()) {
            for line in content.lines() {
                ignores.push(line.to_string());
            }
        }

        if Self::is_git_repo() {
            ignores.push(".git".to_string());
        }

        ignores
    }

    fn is_git_repo() -> bool {
        Path::new(".git").exists()
    }

    fn analyze(file_path: String, target: &String) -> Vec<TargetResult> {
        let content = Self::open_file(&file_path).unwrap_or_default();
        let mut results = Vec::new();

        content.lines().enumerate().for_each(|(e, line)| {
            if line.contains(target) {
                results.push(TargetResult {
                    file_name: file_path.clone(),
                    location: Location {
                        line: e + 1,
                        column: line.find(target).unwrap_or(0) + 1,
                    },
                })
            }
        });

        results
    }
}
