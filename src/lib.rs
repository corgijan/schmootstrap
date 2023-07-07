// Schmfies any String
pub fn schmfy(source: String) -> String {
    if source.starts_with("schm") {
        return source;
    }

    if source.len() == 0 {
        return source;
    }

    // can't be empty
    if !source.chars().next().unwrap().is_alphabetic() {
        return source;
    }

    // if source is subsite (e.g. news/fsr), schmfy all parts separately
    if source.contains('/') {
        return source
            .split('/')
            .map(|s| schmfy(String::from(s)))
            .collect::<Vec<String>>()
            .join("/");
    }

    if source.is_empty() {
        return source;
    }

    // schmfy first char if word is no longer than 3
    if source.len() <= 3 {
        let (prefix, suffix) = source.split_at(1);
        let c = prefix.chars().next().unwrap_or('-');
        return schmfy_char(c) + suffix;
    }

    // Normal words - replace prefix before first vocal
    // with "schm"
    let vok_pos = source
        .chars()
        .position(|c| "aeiouäöü".contains(c))
        .unwrap_or(0);

    let (_, suffix) = source.split_at(vok_pos);

    String::from("schm") + suffix
}

// Schmfies single char
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let schmfied = schmfy(String::from("test"));
        assert_eq!(schmfied, "schmest");
    }
}
