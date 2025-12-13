use std::{any::Any, collections::HashMap};

pub struct Fuzz {
    /// The target string to find within input location(s)
    pub input: String,
}

#[derive(Debug)]
pub struct TargetResult {
    pub id: &'static str,
    pub file_name: String,
    pub location: Location,
}

#[derive(Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Fuzz {
    pub fn new(input: String) -> Self {
        Fuzz { input }
    }

    pub fn run(&self) {
        let results = FileAnalsis::results(self.input.clone());

        for (file_name, result) in results {
            println!(
                "Found target in file: {}, at line: {}, column: {}",
                file_name, result.location.line, result.location.column
            );
        }
    }
}

struct FileAnalsis;
impl FileAnalsis {
    /// Opens a file and reads its contents into a string
    /// TODO: Convert to str and do something with a stream
    fn open_file(input_file: &String) -> anyhow::Result<String> {
        let file = std::fs::read_to_string(input_file)?;
        Ok(file)
    }

    fn results(target_text: String) -> HashMap<String, TargetResult> {
        let walker = walkdir::WalkDir::new(".").into_iter();

        let mut results = HashMap::new();

        for entry in walker.filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let curr_file_analysis =
                    Self::analyze(entry.path().to_string_lossy().to_string(), &target_text);

                for res in curr_file_analysis {
                    results.insert(res.file_name.clone(), res);
                }
            }
        }

        results
    }

    fn analyze(file_path: String, target: &String) -> Vec<TargetResult> {
        let content = Self::open_file(&file_path).unwrap_or_default();

        let mut results = Vec::new();

        content.lines().enumerate().for_each(|(e, line)| {
            if line.contains(target) {
                results.push(TargetResult {
                    id: "make_this_unique_later",
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
