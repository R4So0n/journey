use regex::Regex;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn generate_docs_for_project(src_path: &str, docs_path: &str) -> io::Result<()> {
    fs::create_dir_all(docs_path)?;
    for entry in fs::read_dir(src_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let file_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");

            let output_md_file = format!("{}/{}.md", docs_path, file_name);
            generate_docs(&path, &output_md_file)?;
        }
    }
    Ok(())
}

fn generate_docs(file_path: &Path, output_path: &str) -> io::Result<()> {
    let content = fs::read_to_string(file_path)?;

    let doc_marker = Regex::new(r"// @DOCMYRUST").unwrap();
    let h1 = Regex::new(r"//\s*#\s*(.*)").unwrap();
    let h2 = Regex::new(r"//\s*##\s*(.*)").unwrap();
    let h3 = Regex::new(r"//\s*###\s*(.*)").unwrap();
    let link = Regex::new(r"//\s*\[\[(.*)\]\((.*)\)\]").unwrap();

    let mut in_doc_section = false;
    let mut doc_content = String::new();

    for line in content.lines() {
        if doc_marker.is_match(line) {
            in_doc_section = true;
        } else if in_doc_section {
            if let Some(captures) = h1.captures(line) {
                doc_content.push_str(&format!("# {}\n", &captures[1]));
            } else if let Some(captures) = h2.captures(line) {
                doc_content.push_str(&format!("## {}\n", &captures[1]));
            } else if let Some(captures) = h3.captures(line) {
                doc_content.push_str(&format!("### {}\n", &captures[1]));
            } else if let Some(captures) = link.captures(line) {
                doc_content.push_str(&format!("[{}]({})\n", &captures[1], &captures[2]));
            } else if line.starts_with("//") {
                doc_content.push_str(&line[2..].trim());
                doc_content.push('\n');
            } else {
                in_doc_section = false;
            }
        }
    }

    let mut output_file = File::create(output_path)?;
    output_file.write_all(doc_content.as_bytes())?;

    Ok(())
}
