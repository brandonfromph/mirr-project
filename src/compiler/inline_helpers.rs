//! String-level type/function inlining helpers for the MIRR compiler preprocessor.
//! Keeps other macro processing files clean and compliant with the 600-line cap.

#![forbid(unsafe_code)]

pub(crate) fn inline_types_functions(source: &str) -> String {
    let mut result = source.to_string();

    // 1. extract_data(X) -> ((X) && 4294967295)
    while let Some(start) = result.find("types::extract_data(") {
        let arg_start = start + "types::extract_data(".len();
        if let Some(end) = find_matching_paren(&result, arg_start) {
            let arg = &result[arg_start..end];
            let replacement = format!("(({}) && 4294967295)", arg);
            result.replace_range(start..end + 1, &replacement);
        } else {
            break;
        }
    }

    // 2. extract_tag(X) -> (((X) >> 32) && 15)
    while let Some(start) = result.find("types::extract_tag(") {
        let arg_start = start + "types::extract_tag(".len();
        if let Some(end) = find_matching_paren(&result, arg_start) {
            let arg = &result[arg_start..end];
            let replacement = format!("((({}) >> 32) && 15)", arg);
            result.replace_range(start..end + 1, &replacement);
        } else {
            break;
        }
    }

    // 3. extract_provenance(X) -> (((X) >> 36) && 15)
    while let Some(start) = result.find("types::extract_provenance(") {
        let arg_start = start + "types::extract_provenance(".len();
        if let Some(end) = find_matching_paren(&result, arg_start) {
            let arg = &result[arg_start..end];
            let replacement = format!("((({}) >> 36) && 15)", arg);
            result.replace_range(start..end + 1, &replacement);
        } else {
            break;
        }
    }

    // 4. pack_word(D, T, P) -> ((((P) << 36) || ((T) << 32)) || (D))
    while let Some(start) = result.find("types::pack_word(") {
        let arg_start = start + "types::pack_word(".len();
        if let Some(end) = find_matching_paren(&result, arg_start) {
            let args_str = &result[arg_start..end];
            // Split by comma
            let parts: Vec<&str> = args_str.split(',').collect();
            if parts.len() == 3 {
                let d = parts[0].trim();
                let t = parts[1].trim();
                let p = parts[2].trim();
                let replacement = format!("((((({}) << 36) || (({}) << 32))) || ({}))", p, t, d);
                result.replace_range(start..end + 1, &replacement);
            } else {
                break;
            }
        } else {
            break;
        }
    }

    result
}

fn find_matching_paren(s: &str, start: usize) -> Option<usize> {
    let mut depth = 1;
    for (i, c) in s[start..].char_indices() {
        if c == '(' {
            depth += 1;
        } else if c == ')' {
            depth -= 1;
            if depth == 0 {
                return Some(start + i);
            }
        }
    }
    None
}

pub(crate) fn replace_whole_word(text: &str, word: &str, replacement: &str) -> String {
    let word_len = word.len();
    if word_len == 0 || text.len() < word_len {
        return text.to_string();
    }
    let mut result = String::new();
    let mut last_end = 0;
    let bytes = text.as_bytes();

    let mut i = 0;
    while i <= text.len() - word_len {
        if &text[i..i + word_len] == word {
            let before_ok = i == 0 || !is_ident_char(bytes[i - 1]);
            let after_ok = i + word_len == text.len() || !is_ident_char(bytes[i + word_len]);

            if before_ok && after_ok {
                result.push_str(&text[last_end..i]);
                result.push_str(replacement);
                i += word_len;
                last_end = i;
                continue;
            }
        }
        i += 1;
    }
    result.push_str(&text[last_end..]);
    result
}

fn is_ident_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}
