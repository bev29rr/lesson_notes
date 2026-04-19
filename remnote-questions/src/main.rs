use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use rand::distr::weighted::WeightedIndex;
use rand::prelude::{Distribution, SliceRandom};
use rand::rng;
use serde_json;
use pdf_canvas::{BuiltinFont, Pdf};

const QUESTION_COUNT: usize = 20;

struct CardData {
    source_list: Option<Vec<u32>>,
    counts: HashMap<String, u32>,
}

fn main() -> io::Result<()> {
    let cards_dir = Path::new("cards");
    let counts_path = Path::new("cards/card-data.json");
    let card_data = load_card_data(counts_path)?;
    let paths = find_card_files(cards_dir, card_data.source_list.as_deref())?;
    let mut entries: Vec<(u32, String, Vec<String>)> = Vec::new();

    for path in &paths {
        let file_entries = parse_text_file(path)?;
        entries.extend(file_entries);
    }

    let questions: Vec<(u32, String, Vec<String>)> = choose_n(&entries, QUESTION_COUNT, &card_data.counts);

    let output_path = Path::new("output/questions.pdf");
    generate_pdf(&questions, output_path)?;

    update_card_counts(counts_path, &questions, card_data.source_list.as_deref())?;

    println!("Parsed {} entries from {} file(s)", entries.len(), paths.len());
    println!("Wrote PDF to {}", output_path.display());
    println!("Updated card counts in {}", counts_path.display());

    Ok(())
}

fn generate_pdf(questions: &[(u32, String, Vec<String>)], path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let filename = path
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid output path"))?;
    let mut document = Pdf::create(filename)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

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
        // Convert Unicode characters to ASCII equivalents
        let numbered_question = format!("{}. {}", question_number + 1, convert_unicode(&question));
        let wrapped_question = wrap_text(&numbered_question, max_chars - mark_reserve);
        let dot_lines = answers.len().saturating_mul(2).saturating_sub(1);
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
            pages
                .last_mut()
                .unwrap()
                .push((dot_line.clone(), None));
            current_y -= line_height;
        }

        current_y -= line_height * 0.5;
    }

    for page_lines in pages {
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

    document.render_page(page_width, page_height, |_canvas| Ok(()))?;

    let mut pages: Vec<Vec<(String, Option<String>)>> = vec![Vec::new()];
    current_y = page_height - margin;

    pages.last_mut().unwrap().push(("Answers".to_string(), None));
    current_y -= line_height;
    pages.last_mut().unwrap().push((String::new(), None));
    current_y -= line_height;

    for (question_number, (_, question, answers)) in questions.iter().enumerate() {
        let header = format!("{}. {}", question_number + 1, convert_unicode(&question));
        let wrapped_header = wrap_text(&header, max_chars - mark_reserve);
        let mut answer_lines = Vec::new();
        for answer in answers {
            let wrapped_answer = wrap_text(&convert_unicode(&answer), max_chars - 4);
            for line in wrapped_answer {
                answer_lines.push(format!("    {}", line));
            }
        }
        let required_space = wrapped_header.len() as f32 * line_height
            + answer_lines.len() as f32 * line_height
            + line_height * 0.5;

        if current_y - required_space < margin {
            pages.push(Vec::new());
            current_y = page_height - margin;
        }

        for (idx, line) in wrapped_header.iter().enumerate() {
            let mark = if idx == 0 {
                Some(format!("[{}]", answers.len()))
            } else {
                None
            };
            pages.last_mut().unwrap().push((line.clone(), mark));
            current_y -= line_height;
        }

        for line in answer_lines {
            pages.last_mut().unwrap().push((line, None));
            current_y -= line_height;
        }

        current_y -= line_height * 0.5;
    }

    for page_lines in pages {
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

/// Convert Unicode characters (Greek, mathematical symbols, etc.) to readable ASCII equivalents
fn convert_unicode(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            // Greek letters (lowercase)
            'α' => "alpha".to_string(),
            'β' => "beta".to_string(),
            'γ' => "gamma".to_string(),
            'δ' => "delta".to_string(),
            'ε' | 'ϵ' => "epsilon".to_string(),
            'ζ' => "zeta".to_string(),
            'η' => "eta".to_string(),
            'θ' | 'ϑ' => "theta".to_string(),
            'ι' => "iota".to_string(),
            'κ' | 'ϰ' => "kappa".to_string(),
            'λ' => "lambda".to_string(),
            'μ' => "mu".to_string(),
            'ν' => "nu".to_string(),
            'ξ' => "xi".to_string(),
            'ο' => "omicron".to_string(),
            'π' | 'ϖ' => "pi".to_string(),
            'ρ' | 'ϱ' => "rho".to_string(),
            'ς' | 'σ' => "sigma".to_string(),
            'τ' => "tau".to_string(),
            'υ' => "upsilon".to_string(),
            'φ' | 'ϕ' => "phi".to_string(),
            'χ' => "chi".to_string(),
            'ψ' => "psi".to_string(),
            'ω' => "omega".to_string(),
            // Greek letters (uppercase)
            'Α' => "Alpha".to_string(),
            'Β' => "Beta".to_string(),
            'Γ' => "Gamma".to_string(),
            'Δ' => "Delta".to_string(),
            'Ε' => "Epsilon".to_string(),
            'Ζ' => "Zeta".to_string(),
            'Η' => "Eta".to_string(),
            'Θ' => "Theta".to_string(),
            'Ι' => "Iota".to_string(),
            'Κ' => "Kappa".to_string(),
            'Λ' => "Lambda".to_string(),
            'Μ' => "Mu".to_string(),
            'Ν' => "Nu".to_string(),
            'Ξ' => "Xi".to_string(),
            'Ο' => "Omicron".to_string(),
            'Π' => "Pi".to_string(),
            'Ρ' => "Rho".to_string(),
            'Σ' => "Sigma".to_string(),
            'Τ' => "Tau".to_string(),
            'Υ' => "Upsilon".to_string(),
            'Φ' => "Phi".to_string(),
            'Χ' => "Chi".to_string(),
            'Ψ' => "Psi".to_string(),
            'Ω' => "Omega".to_string(),
            // Mathematical symbols
            '∝' => " proportional_to ".to_string(),
            '≈' => "approx".to_string(),
            '≠' => "!=".to_string(),
            '≤' => "<=".to_string(),
            '≥' => ">=".to_string(),
            '∞' => "infinity".to_string(),
            '√' => "sqrt".to_string(),
            '∫' => "integral".to_string(),
            '∑' => "sum".to_string(),
            '∏' => "product".to_string(),
            '±' => "+/-".to_string(),
            '×' => "x".to_string(),
            '÷' => "/".to_string(),
            '·' => ".".to_string(),
            '°' => "deg".to_string(),
            '′' => "'".to_string(),
            '″' => "''".to_string(),
            '∠' => "angle".to_string(),
            '⊥' => "perpendicular".to_string(),
            '∥' => "parallel".to_string(),
            '→' => "->".to_string(),
            '←' => "<-".to_string(),
            '↑' => "up".to_string(),
            '↓' => "down".to_string(),
            '↔' => "<->".to_string(),
            '⇒' => "=>".to_string(),
            '⇐' => "<=".to_string(),
            '⇔' => "<=>".to_string(),
            '∈' => "elem".to_string(),
            '∉' => "not_elem".to_string(),
            '∅' => "empty".to_string(),
            '∇' => "nabla".to_string(),
            _ => c.to_string(),
        })
        .collect()
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

fn find_card_files(dir: &Path, source_list: Option<&[u32]>) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|ext| ext.to_str()) == Some("txt") {
            if let Some(source_list) = source_list {
                let source_id = extract_source_id(&path);
                if !source_list.contains(&source_id) {
                    continue;
                }
            }
            paths.push(path);
        }
    }

    paths.sort();
    Ok(paths)
}

fn parse_text_file(path: &Path) -> io::Result<Vec<(u32, String, Vec<String>)>> {
    let contents = fs::read_to_string(path)?;
    let source_id = extract_source_id(path);

    let mut entries = Vec::new();
    let mut current_question: Option<String> = None;
    let mut current_answer_lines: Vec<String> = Vec::new();

    for line in contents.lines() {
        let raw = line.trim_end();
        if raw.is_empty() {
            continue;
        }

        let is_answer_line = raw.starts_with(' ') || raw.starts_with('\t');
        if is_answer_line {
            if current_question.is_some() {
                current_answer_lines.push(raw.trim_start().to_string());
            }
            continue;
        }

        if let Some(question_text) = current_question.take() {
            entries.push((source_id, question_text, current_answer_lines.drain(..).collect()));
        }

        let (question_text, answer_text) = split_question_answer(raw);
        current_question = Some(question_text);
        if let Some(answer) = answer_text {
            current_answer_lines.push(answer);
        }
    }

    if let Some(question_text) = current_question {
        entries.push((source_id, question_text, current_answer_lines.drain(..).collect()));
    }

    Ok(entries)
}

fn split_question_answer(line: &str) -> (String, Option<String>) {
    if let Some((left, right)) = line.split_once('→') {
        (left.trim().to_string(), Some(right.trim().to_string()))
    } else {
        (line.trim().to_string(), None)
    }
}

fn extract_source_id(path: &Path) -> u32 {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(|stem| stem.chars().filter(|c| c.is_ascii_digit()).collect::<String>())
        .and_then(|digits| digits.parse::<u32>().ok())
        .unwrap_or(0)
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

    let mut rng = rng();
    let sample_n = n.min(items.len());
    let mut selected = Vec::with_capacity(sample_n);

    for _ in 0..sample_n {
        let dist = WeightedIndex::new(&weights).expect("weights must be positive");
        let idx = dist.sample(&mut rng);
        selected.push(items.swap_remove(idx));
        weights.swap_remove(idx);
    }

    selected.shuffle(&mut rng);
    selected
}
