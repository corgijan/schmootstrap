//! schmfy - a library to schmfy everything
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Copy, Clone)]
enum CaseType {
    /// the whole word is in lowercase [default]
    Lcase,
    /// the whole word is in uppercase
    Ucase,
    /// the first letter is uppercase, the rest is lowercase
    FstUcase,
}

/// Returns the case type of a str
fn get_case(txt: &str) -> CaseType {
    let mut cnt_lcase: usize = 0;
    let mut cnt_ucase: usize = 0;

    let alph = txt
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();

    alph.iter().for_each(|c| {
        if c.is_uppercase() {
            cnt_ucase += 1;
        }
        if c.is_lowercase() {
            cnt_lcase += 1;
        }
    });

    if alph.len() == 0 {
        return CaseType::Lcase; // default
    } else if cnt_lcase > 0 && cnt_ucase == 0 {
        return CaseType::Lcase;
    } else if cnt_lcase == 0 && cnt_ucase > 0 {
        return CaseType::Ucase;
    } else if alph[0].is_uppercase() && alph[1].is_lowercase() {
        // at least 2 entries
        return CaseType::FstUcase;
    }

    CaseType::Lcase
}

fn restore_case(txt: String, case: CaseType) -> String {
    match case {
        CaseType::FstUcase => txt
            .to_lowercase()
            .chars()
            .enumerate()
            .map(|(pos, c)| {
                if pos == 0 {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            })
            .collect::<String>(),
        CaseType::Lcase => txt.to_lowercase(),
        CaseType::Ucase => txt.to_uppercase(),
    }
}

/// Schmfies any str, preserving case and everything non-alphabetical
#[wasm_bindgen]
pub fn schmfy(source: &str) -> String {
    // instantly return if input is non-alphabetic single char
    if source.len() == 1 && !source.chars().next().unwrap().is_alphabetic() {
        return String::from(source);
    }

    let case = get_case(source);

    // already schmfied
    if source.to_lowercase().starts_with("schm") {
        return String::from(source);
    }

    // empty
    if source.len() == 0 {
        return String::from(source);
    }

    // Schmfy each substring separately
    let mut current_substr: Vec<char> = vec![];
    let mut substrings: Vec<String> = vec![];
    source.chars().for_each(|c| {
        if c.is_alphabetic() {
            current_substr.push(c)
        } else {
            if current_substr.len() > 0 {
                substrings.push(current_substr.iter().collect::<String>());
                current_substr.clear();
            }
            substrings.push(c.to_string())
        }
    });
    if current_substr.len() > 0 {
        substrings.push(current_substr.iter().collect::<String>());
    }

    if substrings.len() > 1 {
        return substrings
            .iter()
            .map(|txt| schmfy(txt))
            .collect::<Vec<String>>()
            .join("");
    }

    // substrings now has to contain exactly one element
    let source = substrings[0].to_lowercase();

    if !source.chars().next().unwrap().is_alphabetic() {
        return String::from(source);
    }

    // schmfy first char if word is no longer than 3
    if source.len() <= 3 && case != CaseType::FstUcase {
        let first_c_size = source.chars().next().unwrap().len_utf8();
        let (prefix, suffix) = source.split_at(first_c_size);
        let c = prefix.chars().next().unwrap_or('-');
        return restore_case(schmfy_char(c) + suffix, case);
    }

    // Normal words - replace prefix before first vocal
    // with "schm"
    let vok_pos = source.find(|c| "aeiouäöü".contains(c)).unwrap_or(0);

    let (_, suffix) = source.split_at(vok_pos);

    restore_case(String::from("schm") + suffix, case)
}

/// Schmfies single char
fn schmfy_char(c: char) -> String {
    let mut ret = String::from("schm");
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'ä' | 'ö' | 'ü' => {
            ret.push(c);
        }
        'b' | 'c' | 'd' | 'g' | 'p' | 't' | 'w' => ret.push('e'),
        'f' | 'l' | 'm' | 'n' | 'r' | 's' => {
            ret.push('e');
            ret.push(c)
        }
        'h' | 'k' => ret.push('a'),
        'j' => {
            ret.push('o');
            ret.push('t')
        }
        'q' => ret.push('u'),
        'v' => {
            ret.push('a');
            ret.push('u')
        }
        'x' => {
            ret.push('i');
            ret.push('x')
        }
        'y' => ret.push(c),
        'z' => {
            ret.push('e');
            ret.push('t')
        }
        _ => ret.push(c),
    }
    ret

}
