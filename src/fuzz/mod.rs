use ariadne::{Color, Label, Report, ReportKind, Source};

pub struct Fuzz {
    /// The target string to find within input location(s)
    pub input: String,
}

#[derive(Debug)]
pub struct TargetResult {
    pub file_name: String,
    pub location: Location,
}

#[derive(Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

const NEW_LINE: usize = 1;
const LINE_LOCATION_OFFSET: usize = 1;
const COLUMN_LOCATION_OFFSET: usize = 1;

impl Fuzz {
    pub fn new(input: String) -> Self {
        Fuzz { input }
    }

    fn build_report(&self, content: &String, file_name: String, result: &TargetResult) {
        let byte_offset = content
            .lines()
            .take(result.location.line - LINE_LOCATION_OFFSET)
            .map(|line| line.len() + NEW_LINE)
            .sum::<usize>()
            + result.location.column
            - COLUMN_LOCATION_OFFSET;

        Report::build(
            ReportKind::Custom("Found", Color::Magenta),
            (
                file_name.clone(),
                byte_offset..byte_offset + self.input.len(),
            ),
        )
        .with_message("Reference found")
        .with_label(
            Label::new((
                file_name.clone(),
                byte_offset..byte_offset + self.input.len(),
            ))
            .with_message("referenced here")
            .with_color(Color::Cyan),
        )
        .finish()
        .eprint((file_name, Source::from(content)))
        .expect("Failed to print report");
    }

    pub fn run(&self) {
        let results = FileAnalsis::results(self.input.clone());

        for result in results {
            let name = &result.file_name;
            let content = FileAnalsis::open_file(name).unwrap_or_default();
            self.build_report(&content, name.to_string(), &result);
        }
    }
}

struct FileAnalsis;
impl FileAnalsis {
    /// Opens a file and reads its contents into a string
    fn open_file(input_file: &String) -> anyhow::Result<String> {
        let file = std::fs::read_to_string(input_file)?;
        Ok(file)
    }

    /// Walks current directory recursively and analyzes each file for the target text
    /// and parses this to a hashmap of results.
    /// For each file a TargetResult is created and stored in the hashmap with the file name as key.
    /// If multiple occurrences are found in a file, only the last one is stored. Maybe this should
    /// be a vec instead
    fn results(target_text: String) -> Vec<TargetResult> {
        let walker = walkdir::WalkDir::new(".").into_iter();

        let mut results = Vec::new();

        for entry in walker.filter_map(|e| e.ok()) {
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
