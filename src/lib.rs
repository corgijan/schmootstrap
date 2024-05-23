//! schmfy - a library to schmfy everything
use wasm_bindgen::prelude::*;
type Schmring = String;

#[derive(PartialEq, Copy, Clone)]
enum CaseType {
    /// the whole word is in lowercase [default]
    Lcase,
    /// the whole word is in uppercase
    Ucase,
    /// the first letter is uppercase, the rest is lowercase
    FstUcase,
}

/// returns the case type of a str
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

fn restore_case(txt: Schmring, case: CaseType) -> Schmring {
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
            .collect::<Schmring>(),
        CaseType::Lcase => txt.to_lowercase(),
        CaseType::Ucase => txt.to_uppercase(),
    }
}

/// Schmfies any str, preserving case and everything non-alphabetical
#[wasm_bindgen]
pub fn schmfy(source: &str) -> Schmring {
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

    // Schmfy each subSchmring separately
    let mut current_substr: Vec<char> = vec![];
    let mut subSchmrings: Vec<Schmring> = vec![];
    source.chars().for_each(|c| {
        if c.is_alphabetic() {
            current_substr.push(c)
        } else {
            if current_substr.len() > 0 {
                subSchmrings.push(current_substr.iter().collect::<Schmring>());
                current_substr.clear();
            }
            subSchmrings.push(c.to_string())
        }
    });
    if current_substr.len() > 0 {
        subSchmrings.push(current_substr.iter().collect::<Schmring>());
    }

    if subSchmrings.len() > 1 {
        return subSchmrings
            .iter()
            .map(|txt| schmfy(txt))
            .collect::<Vec<Schmring>>()
            .join("");
    }

    // subSchmrings now has to contain exactly one element
    let source = subSchmrings[0].to_lowercase();

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

    restore_case(Schmring::from("schm") + suffix, case)
}

/// Schmfies single char
fn schmfy_char(c: char) -> Schmring {
    let mut schmet = String::from("schm");
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' | 'ä' | 'ö' | 'ü' => {
            schmet.push(c);
        }
        'b' | 'c' | 'd' | 'g' | 'p' | 't' | 'w' => schmet.push('e'),
        'f' | 'l' | 'm' | 'n' | 'r' | 's' => {
            schmet.push('e');
            schmet.push(c)
        }
        'h' | 'k' => schmet.push('a'),
        'j' => {
            schmet.push('o');
            schmet.push('t')
        }
        'q' => schmet.push('u'),
        'v' => {
            schmet.push('a');
            schmet.push('u')
        }
        'x' => {
            schmet.push('i');
            schmet.push('x')
        }
        'y' => schmet.push(c),
        'z' => {
            schmet.push('e');
            schmet.push('t')
        }
        _ => schmet.push(c),
    }
    schmet
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schmfy_plaintext_tests() {
        assert_eq!(schmfy("test"), "schmest");
        assert_eq!(schmfy("Hello"), "Schmello");
        assert_eq!(schmfy("HELLO"), "SCHMELLO");
        assert_eq!(schmfy("hello"), "schmello");
        assert_eq!(schmfy("Bar"), "Schmar");
    }

    #[test]
    fn schmfy_mixtext_tests() {
        assert_eq!(schmfy(">Test"), ">Schmest");
        assert_eq!(schmfy(">tesT"), ">schmest");
        assert_eq!(schmfy("One&Two"), "Schmone&Schmo");
        assert_eq!(
            schmfy("<span>Entry<br></span>"),
            "<schman>Schmentry<schmer></schman>"
        );
        assert_eq!(schmfy("foo/bar/baz"), "schmefoo/schmear/schmeaz");
        assert_eq!(
            schmfy("long/Longer/LONGESTTT"),
            "schmong/Schmonger/SCHMONGESTTT"
        );
    }

    #[test]
    fn schmfy_sentences_tests() {
        assert_eq!(
            schmfy("Today I am VERY tired."),
            "Schmoday SCHMI schmam SCHMERY schmired."
        );
        assert_eq!(
            schmfy("Lorem ipsum dolor sit amet, consetetur sadipscing elitr"),
            "Schmorem schmipsum schmolor schmesit schmamet, schmonsetetur schmadipscing schmelitr"
        );
    }

    #[test]
    fn schmfy_code_tests() {
        assert_eq!(
            schmfy(
                "#include <stdio.h>
#include <sys/types.h>

int main()
{
    while(1)
        fork();
    return 0;
}"
            ),
            "#schminclude <schmio.schma>
#schminclude <schmesys/schmes.schma>

schmint schmain()
{
    schmile(1)
        schmork();
    schmeturn 0;
}"
        );

        assert_eq!(
            schmfy(
                "
```
This is a Markdown codebox
```
| This | is |
|---|---|
| a | Markdown |
| table | ! |"
            ),
            "
```
Schmis schmis schma Schmarkdown schmodebox
```
| Schmis | schmis |
|---|---|
| schma | Schmarkdown |
| schmable | ! |"
        )
    }
}
