use ariadne::{Color, Label, Report, ReportKind, Source};

mod analysis;

use analysis::FileAnalysis;

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
        let results = FileAnalysis::results(self.input.clone());

        for result in results {
            let name = &result.file_name;
            let content = FileAnalysis::open_file(name).unwrap_or_default();
            self.build_report(&content, name.to_string(), &result);
        }
    }
}
