use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(rename_all = "lowercase")]
pub enum LineCoverage {
    Covered,
    Uncovered,
    Ignored,
}

#[derive(Serialize)]
pub struct SimpleReport {
    pub files: Vec<SimpleFileReport>,
    pub coverage: f32,
}

#[derive(Serialize)]
pub struct SimpleFileReport {
    pub path: PathBuf,
    pub lines: Vec<Line>,
    pub covered: u32,
    pub coverable: u32,
}

#[derive(Serialize)]
pub struct Line {
    pub number: u32,
    pub content: String,
    pub coverage: LineCoverage,
}

impl From<Report> for SimpleReport {
    fn from(report: Report) -> Self {
        let mut coverable = 0;
        let mut covered = 0;

        let mut files = Vec::new();
        for file in &report.files {
            coverable += file.coverable;
            covered += file.covered;

            let mut lines = Vec::new();
            for (i, line) in file.content.lines().enumerate() {
                let coverage = match file.traces.iter().find(|trace| trace.line == i as u32 + 1) {
                    // TODO: this is slow
                    Some(trace) => {
                        if trace.stats.line > 0 {
                            LineCoverage::Covered
                        } else {
                            LineCoverage::Uncovered
                        }
                    }
                    None => LineCoverage::Ignored,
                };
                lines.push(Line {
                    number: i as u32 + 1,
                    content: line.to_string(),
                    coverage,
                });
            }

            files.push(SimpleFileReport {
                path: file.path.iter().collect(),
                lines,
                covered: file.covered,
                coverable: file.coverable,
            });
        }

        SimpleReport {
            files,
            coverage: covered as f32 / coverable as f32,
        }
    }
}

#[derive(Deserialize)]
pub struct Report {
    pub files: Vec<FileReport>,
}

#[derive(Deserialize)]
pub struct FileReport {
    pub path: Vec<String>,
    pub content: String,
    pub traces: Vec<Trace>,
    pub covered: u32,
    pub coverable: u32,
}

#[derive(Deserialize)]
pub struct Trace {
    pub line: u32,
    pub address: Vec<u32>,
    pub length: u32,
    pub stats: Stats,
    pub fn_name: Option<String>,
}

#[derive(Deserialize)]
pub struct Stats {
    #[serde(rename(deserialize = "Line"))]
    pub line: u32,
}

pub fn run_tarpaulin(path: &PathBuf) -> Result<Report, anyhow::Error> {
    let output_path = get_output_path(path)?;
    let mut proc = std::process::Command::new("cargo")
        .args([
            "tarpaulin",
            "--out",
            "Json",
            "--output-dir",
            &output_path.to_string_lossy(),
        ])
        .current_dir(path)
        .spawn()?;

    let result = proc.wait()?;
    if !result.success() {
        return Err(anyhow::anyhow!("Tarpaulin failed"));
    }

    let report_json = std::fs::read_to_string(output_path.join("tarpaulin-report.json"))?;

    // Windows doesn't clean temp automatically, so we need to do it manually
    #[cfg(windows)]
    std::fs::remove_dir_all(&output_path)?;

    serde_json::from_str(&report_json).map_err(Into::into)
}

pub fn get_output_path(project_path: &Path) -> Result<PathBuf, anyhow::Error> {
    let temp_dir = std::env::temp_dir();
    let output_dir = temp_dir.join(format!("tarpaulin-{}", uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_URL, project_path.to_string_lossy().as_bytes())));
    std::fs::create_dir(&output_dir)?;
    Ok(output_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_output_path() {
        let result = get_output_path(PathBuf::from("/tmp/something").as_path()).unwrap();
        assert!(result
            .into_os_string()
            .into_string()
            .unwrap()
            .contains("tarpaulin-"));
    }

    #[test]
    fn test_from_report_for_simplereport() {
        let report = Report {
            files: vec![FileReport {
                path: vec!["src".to_string(), "main.rs".to_string()],
                content: "fn main() {\n    println!(\"Hello, world!\");\n}".to_string(),
                traces: vec![
                    Trace {
                        line: 1,
                        address: vec![0],
                        length: 1,
                        stats: Stats { line: 1 },
                        fn_name: None,
                    },
                    Trace {
                        line: 2,
                        address: vec![1],
                        length: 1,
                        stats: Stats { line: 1 },
                        fn_name: None,
                    },
                ],
                covered: 2,
                coverable: 2,
            }],
        };
        let simple_report: SimpleReport = report.into();
        assert_eq!(simple_report.coverage, 1.0);
        assert_eq!(simple_report.files.len(), 1);
        assert_eq!(simple_report.files[0].path, PathBuf::from("src/main.rs"));
        assert_eq!(simple_report.files[0].lines.len(), 3);
        assert_eq!(simple_report.files[0].lines[0].content, "fn main() {");
        assert_eq!(
            simple_report.files[0].lines[0].coverage,
            LineCoverage::Covered
        );
        assert_eq!(
            simple_report.files[0].lines[1].content,
            "    println!(\"Hello, world!\");"
        );
        assert_eq!(
            simple_report.files[0].lines[1].coverage,
            LineCoverage::Covered
        );
        assert_eq!(simple_report.files[0].lines[2].content, "}");
        assert_eq!(simple_report.files[0].lines[2].coverage, LineCoverage::Ignored);
    }
}
