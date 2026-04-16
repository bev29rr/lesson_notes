use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use pdf_canvas::{BuiltinFont, Pdf};
use pdf_extract::extract_text;
use rand::prelude::{Rng, SliceRandom};
use serde_json;

const QUESTION_COUNT: usize = 30;

struct CardData {
    source_list: Option<Vec<u32>>,
    counts: HashMap<String, u32>,
}

fn main() -> io::Result<()> {
    let cards_dir = Path::new("cards");
    let txt_dir = Path::new("txt-out");
    let output_path = Path::new("output/questions.pdf");

    if cards_dir.exists() {
        let pdf_paths = find_pdf_files(cards_dir)?;
        if !pdf_paths.is_empty() {
            fs::create_dir_all(txt_dir)?;

            for pdf_path in &pdf_paths {
                let text = extract_text(&pdf_path).map_err(|err| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("failed to extract text from {}: {}", pdf_path.display(), err),
                    )
                })?;

                let output_path = txt_dir
                    .join(
                        pdf_path
                            .file_stem()
                            .unwrap_or_else(|| OsStr::new("output"))
                            .to_owned(),
                    )
                    .with_extension("txt");

                fs::write(&output_path, text)?;
                println!("Converted {} -> {}", pdf_path.display(), output_path.display());
            }
        } else {
            println!("No PDF files found in {}. Proceeding to parse existing text files.", cards_dir.display());
        }
    }

    let counts_path = Path::new("cards/card-data.json");
    let card_data = load_card_data(counts_path)?;

    let txt_paths = find_txt_files(txt_dir)?;
    if txt_paths.is_empty() {
        eprintln!("No TXT files found in {}", txt_dir.display());
        return Ok(());
    }

    let mut entries: Vec<(u32, String, Vec<String>)> = Vec::new();
    for txt_path in &txt_paths {
        entries.extend(parse_text_file(txt_path)?);
    }

    let filtered_entries: Vec<(u32, String, Vec<String>)> = entries
        .into_iter()
        .filter(|(source_id, _, _)| {
            card_data
                .source_list
                .as_deref()
                .map(|list| list.contains(source_id))
                .unwrap_or(true)
        })
        .collect();

    let selected_questions = choose_n(&filtered_entries, QUESTION_COUNT, &card_data.counts);

    fs::create_dir_all(output_path.parent().unwrap())?;
    generate_pdf(&selected_questions, output_path)?;
    update_card_counts(counts_path, &selected_questions, card_data.source_list.as_deref())?;

    println!("Parsed {} entries from {} text file(s)", filtered_entries.len(), txt_paths.len());
    println!("Wrote PDF to {}", output_path.display());

    Ok(())
}

fn generate_pdf(questions: &[(u32, String, Vec<String>)], path: &Path) -> io::Result<()> {
    let filename = path
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid output path"))?;
    let mut document = Pdf::create(filename).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let page_width = 595.0_f32;
    let page_height = 842.0_f32;
    let margin = 40.0_f32;
    let font_size = 11.0_f32;
    let line_height = 14.0_f32;
    let max_chars = ((page_width - margin * 2.0) / 6.5).floor() as usize;
    let mark_reserve = 10;
    let dot_line = std::iter::repeat('.').take(max_chars).collect::<String>();

    let mut pages: Vec<Vec<(String, Option<String>)>> = vec![Vec::new()];
    let mut current_y = page_height - margin;

    for (question_number, (_, question, answers)) in questions.iter().enumerate() {
        let numbered_question = format!("{}. {}", question_number + 1, question);
        let wrapped_question = wrap_text(&numbered_question, max_chars - mark_reserve);
        let dot_lines = answers.len().saturating_mul(2);
        let required_space = wrapped_question.len() as f32 * line_height
            + dot_lines as f32 * line_height
            + line_height * 0.5;

        if current_y - required_space < margin {
            pages.push(Vec::new());
            current_y = page_height - margin;
        }

        for (idx, line) in wrapped_question.iter().enumerate() {
            let mark = if idx == 0 {
                Some(format!("[{}]", answers.len()))
            } else {
                None
            };
            pages.last_mut().unwrap().push((line.clone(), mark));
            current_y -= line_height;
        }

        for _ in 0..dot_lines {
            pages.last_mut().unwrap().push((dot_line.clone(), None));
            current_y -= line_height;
        }

        current_y -= line_height * 0.5;
    }

    for page_lines in &pages {
        document.render_page(page_width, page_height, |canvas| {
            for (idx, (text, mark)) in page_lines.iter().enumerate() {
                let y = page_height - margin - idx as f32 * line_height;
                canvas.left_text(margin, y, BuiltinFont::Courier, font_size, text)?;
                if let Some(mark_text) = mark {
                    canvas.right_text(
                        page_width - margin,
                        y,
                        BuiltinFont::Courier,
                        font_size,
                        mark_text,
                    )?;
                }
            }
            Ok(())
        })?;
    }

    let mut answer_pages: Vec<Vec<(String, Option<String>)>> = vec![Vec::new()];
    current_y = page_height - margin;
    answer_pages.last_mut().unwrap().push(("Answers".to_string(), None));
    current_y -= line_height;
    answer_pages.last_mut().unwrap().push((String::new(), None));
    current_y -= line_height;

    for (question_number, (_, question, answers)) in questions.iter().enumerate() {
        let header = format!("{}. {}", question_number + 1, question);
        let wrapped_header = wrap_text(&header, max_chars - mark_reserve);
        let mut answer_lines = Vec::new();
        for answer in answers {
            let wrapped_answer = wrap_text(answer, max_chars - 4);
            for line in wrapped_answer {
                answer_lines.push(format!("    {}", line));
            }
        }
        let required_space = wrapped_header.len() as f32 * line_height
            + answer_lines.len() as f32 * line_height
            + line_height * 0.5;

        if current_y - required_space < margin {
            answer_pages.push(Vec::new());
            current_y = page_height - margin;
        }

        for (idx, line) in wrapped_header.iter().enumerate() {
            let mark = if idx == 0 {
                Some(format!("[{}]", answers.len()))
            } else {
                None
            };
            answer_pages.last_mut().unwrap().push((line.clone(), mark));
            current_y -= line_height;
        }

        for line in answer_lines {
            answer_pages.last_mut().unwrap().push((line, None));
            current_y -= line_height;
        }

        current_y -= line_height * 0.5;
    }

    for page_lines in answer_pages {
        document.render_page(page_width, page_height, |canvas| {
            for (idx, (text, mark)) in page_lines.iter().enumerate() {
                let y = page_height - margin - idx as f32 * line_height;
                if !text.is_empty() {
                    canvas.left_text(margin, y, BuiltinFont::Courier, font_size, text)?;
                }
                if let Some(mark_text) = mark {
                    canvas.right_text(
                        page_width - margin,
                        y,
                        BuiltinFont::Courier,
                        font_size,
                        mark_text,
                    )?;
                }
            }
            Ok(())
        })?;
    }

    document.finish().map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
}

fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if current.len() + 1 + word.len() <= max_chars {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current);
            current = word.to_string();
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

fn find_pdf_files(cards_dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut pdf_paths = Vec::new();

    for entry in fs::read_dir(cards_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file()
            && path
                .extension()
                .and_then(OsStr::to_str)
                .map(|ext| ext.eq_ignore_ascii_case("pdf"))
                .unwrap_or(false)
        {
            pdf_paths.push(path);
        }
    }

    Ok(pdf_paths)
}

fn find_txt_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut txt_paths = Vec::new();

    if !dir.exists() {
        return Ok(txt_paths);
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file()
            && path
                .extension()
                .and_then(OsStr::to_str)
                .map(|ext| ext.eq_ignore_ascii_case("txt"))
                .unwrap_or(false)
        {
            txt_paths.push(path);
        }
    }

    txt_paths.sort();
    Ok(txt_paths)
}

fn parse_text_file(path: &Path) -> io::Result<Vec<(u32, String, Vec<String>)>> {
    let contents = fs::read_to_string(path)?;
    let source_id = extract_source_id(path);

    let mut entries = Vec::new();
    let mut current_term: Option<String> = None;
    let mut current_answer = String::new();

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.eq_ignore_ascii_case("www.pmt.education") {
            continue;
        }

        if is_section_header(line) {
            continue;
        }

        if let Some((term, definition)) = split_term_definition(line) {
            if let Some(term_text) = current_term.take() {
                entries.push((source_id, question_for_term(&term_text), vec![clean_answer(&current_answer)]));
            }

            current_term = Some(term.to_string());
            current_answer = definition.trim().to_string();
            continue;
        }

        if current_term.is_some() {
            if !current_answer.is_empty() {
                current_answer.push(' ');
            }
            current_answer.push_str(line);
        }
    }

    if let Some(term_text) = current_term {
        entries.push((source_id, question_for_term(&term_text), vec![clean_answer(&current_answer)]));
    }

    Ok(entries)
}

fn clean_answer(answer: &str) -> String {
    let answer = answer.trim();
    let answer = answer.strip_prefix('?').unwrap_or(answer);
    answer
        .chars()
        .filter(|&c| {
            c != '\u{200B}' && c != '\u{FEFF}' && c != '\u{200C}' && c != '\u{200D}' && c != '\u{2060}'
        })
        .collect::<String>()
        .trim()
        .to_string()
}

fn load_card_data(path: &Path) -> io::Result<CardData> {
    if !path.exists() {
        return Ok(CardData { source_list: None, counts: HashMap::new() });
    }

    let contents = fs::read_to_string(path)?;
    if contents.trim().is_empty() {
        return Ok(CardData { source_list: None, counts: HashMap::new() });
    }

    let value: serde_json::Value = serde_json::from_str(&contents).map_err(|err| {
        io::Error::new(io::ErrorKind::InvalidData, format!("invalid JSON in {}: {}", path.display(), err))
    })?;

    let source_list = value.get("source-list").and_then(|v| {
        v.as_array().map(|arr| {
            arr.iter()
                .filter_map(|item| item.as_u64().map(|n| n as u32))
                .collect::<Vec<u32>>()
        })
    });

    let mut counts = HashMap::new();
    if let Some(counts_obj) = value.get("counts").and_then(|v| v.as_object()) {
        for (key, val) in counts_obj {
            if let Some(n) = val.as_u64() {
                counts.insert(key.clone(), n as u32);
            }
        }
    }

    if let Some(map) = value.as_object() {
        for (key, val) in map {
            if key == "source-list" || key == "counts" {
                continue;
            }
            if let Some(n) = val.as_u64() {
                counts.insert(key.clone(), n as u32);
            }
        }
    }

    Ok(CardData { source_list, counts })
}

fn update_card_counts(path: &Path, questions: &[(u32, String, Vec<String>)], source_list: Option<&[u32]>) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let card_data = load_card_data(path)?;
    let mut counts = card_data.counts;

    for (source_id, _, _) in questions {
        let key = source_id.to_string();
        *counts.entry(key).or_default() += 1;
    }

    let mut output = serde_json::Map::new();
    if let Some(list) = source_list {
        output.insert("source-list".to_string(), serde_json::json!(list));
    }
    let counts_map = counts.iter().map(|(k, v)| (k.clone(), serde_json::json!(v))).collect();
    output.insert("counts".to_string(), serde_json::Value::Object(counts_map));

    let file = fs::File::create(path)?;
    serde_json::to_writer_pretty(file, &serde_json::Value::Object(output)).map_err(|err| {
        io::Error::new(io::ErrorKind::Other, format!("failed to write {}: {}", path.display(), err))
    })?;

    Ok(())
}

fn choose_n(
    v: &[(u32, String, Vec<String>)],
    n: usize,
    counts: &HashMap<String, u32>,
) -> Vec<(u32, String, Vec<String>)> {
    let mut items: Vec<_> = v.iter().cloned().collect();
    let mut weights: Vec<f64> = items
        .iter()
        .map(|(source_id, _, _)| {
            let count = counts.get(&source_id.to_string()).copied().unwrap_or(0) as f64;
            1.0 / (count + 1.0)
        })
        .collect();

    let mut rng = rand::rng();
    let sample_n = n.min(items.len());
    let mut selected = Vec::with_capacity(sample_n);

    for _ in 0..sample_n {
        let total_weight: f64 = weights.iter().sum();
        let mut target = rng.random_range(0.0..total_weight);
        let idx = weights
            .iter()
            .position(|w| {
                target -= *w;
                target <= 0.0
            })
            .unwrap_or(weights.len() - 1);
        selected.push(items.swap_remove(idx));
        weights.swap_remove(idx);
    }

    selected.shuffle(&mut rng);
    selected
}

fn split_term_definition(line: &str) -> Option<(&str, &str)> {
    let mut parts = line.splitn(2, ':');
    let before = parts.next()?;
    let after = parts.next()?;

    if is_section_header_before(before) {
        return None;
    }

    Some((before.trim(), after))
}

fn is_section_header(line: &str) -> bool {
    if let Some((before, _)) = line.split_once(':') {
        is_section_header_before(before)
    } else {
        false
    }
}

fn is_section_header_before(prefix: &str) -> bool {
    let trimmed = prefix.trim();
    !trimmed.is_empty() && trimmed.chars().all(|c| c.is_ascii_digit() || c == '.')
}

fn question_for_term(term: &str) -> String {
    let term = term.trim().trim_end_matches(|c: char| c == '.' || c == '?' || c == '!');
    if term.is_empty() {
        return String::from("What is this?");
    }

    let lower = term.to_lowercase();
    if lower.starts_with("what is") || lower.starts_with("define") || lower.starts_with("describe") {
        return term.to_string();
    }

    format!("What is {}?", term)
}

fn extract_source_id(path: &Path) -> u32 {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(|stem| stem.chars().filter(|c| c.is_ascii_digit()).collect::<String>())
        .and_then(|digits| digits.parse::<u32>().ok())
        .unwrap_or(0)
}
