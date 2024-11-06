use std::collections::HashMap;

pub struct IastToSlp1Transliterate {
    map: HashMap<&'static str, &'static str>,
}

pub struct ISlp1ToIastTransliterate {
    map: HashMap<&'static str, &'static str>,
}

#[allow(dead_code)]
pub trait Transliterate {
    fn new() -> Self;
    fn transliterate(&self, input: &str);
    fn generate_output(&self, output: &mut String, i: usize, chars: Vec<char>);
}

impl Transliterate for IastToSlp1Transliterate {
    fn new() -> IastToSlp1Transliterate {
        IastToSlp1Transliterate {
            map: create_iast_to_slp1_map(),
        }
    }

    fn transliterate(&self, input: &str) {
        let mut output = String::new();

        let i = 0;
        let chars: Vec<char> = input.chars().collect();
        self.generate_output(&mut output, i, chars);
    }

    fn generate_output(&self, output: &mut String, mut i: usize, chars: Vec<char>) {
        while i < chars.len() {
            let mut found = false;

            // Try to match sequences of length 2 or more first
            for len in (1..=2).rev() {
                if i + len <= chars.len() {
                    let slice: String = chars[i..i + len].iter().collect();
                    if let Some(&replacement) = self.map.get(slice.as_str()) {
                        output.push_str(replacement);
                        i += len;
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                // If no sequence is matched, add the character as is
                output.push(chars[i]);
                i += 1;
            }
        }
    }
}

impl Transliterate for ISlp1ToIastTransliterate {
    fn new() -> ISlp1ToIastTransliterate {
        ISlp1ToIastTransliterate {
            map: create_slp1_to_iast_map(),
        }
    }

    fn transliterate(&self, input: &str) {
        let mut output = String::new();

        let i = 0;
        let chars: Vec<char> = input.chars().collect();
        self.generate_output(&mut output, i, chars);
    }

    fn generate_output(&self, output: &mut String, mut i: usize, chars: Vec<char>) {
        while i < chars.len() {
            let mut found = false;

            // Try to match sequences of length 2 or more first
            for len in (1..=2).rev() {
                if i + len <= chars.len() {
                    let slice: String = chars[i..i + len].iter().collect();
                    if let Some(&replacement) = self.map.get(slice.as_str()) {
                        output.push_str(replacement);
                        i += len;
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                // If no sequence is matched, add the character as is
                output.push(chars[i]);
                i += 1;
            }
        }
    }
}

fn create_iast_to_slp1_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // Add mappings for vowels
    map.insert("ā", "A");
    map.insert("ī", "I");
    map.insert("ū", "U");
    map.insert("ṛ", "f");
    map.insert("ṝ", "F");
    map.insert("ḷ", "x");
    map.insert("ḹ", "X");
    map.insert("e", "e");
    map.insert("o", "o");
    map.insert("ai", "E");
    map.insert("au", "O");
    map.insert("k", "k");
    map.insert("kh", "K");
    map.insert("g", "g");
    map.insert("gh", "G");
    map.insert("ṅ", "N");
    map.insert("c", "c");
    map.insert("ch", "C");
    map.insert("j", "j");
    map.insert("jh", "J");
    map.insert("ñ", "Y");
    map.insert("ṭ", "w");
    map.insert("ṭh", "W");
    map.insert("ḍ", "q");
    map.insert("ḍh", "Q");
    map.insert("ṇ", "R");
    map.insert("t", "t");
    map.insert("th", "T");
    map.insert("d", "d");
    map.insert("dh", "D");
    map.insert("n", "n");
    map.insert("p", "p");
    map.insert("ph", "P");
    map.insert("b", "b");
    map.insert("bh", "B");
    map.insert("m", "m");
    map.insert("y", "y");
    map.insert("r", "r");
    map.insert("l", "l");
    map.insert("v", "v");
    map.insert("ś", "S");
    map.insert("ṣ", "z");
    map.insert("s", "s");
    map.insert("h", "h");

    // Add mappings for other symbols
    map.insert("ṃ", "M");
    map.insert("ḥ", "H");
    map.insert("'", "'");
    map.insert(" ", " ");

    map
}

fn create_slp1_to_iast_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // Add mappings for vowels
    map.insert("A", "ā");
    map.insert("I", "ī");
    map.insert("U", "ū");
    map.insert("f", "ṛ");
    map.insert("F", "ṝ");
    map.insert("x", "ḷ");
    map.insert("X", "ḹ");
    map.insert("e", "e");
    map.insert("o", "o");
    map.insert("E", "ai");
    map.insert("O", "au");

    // Add mappings for consonants
    map.insert("k", "k");
    map.insert("K", "kh");
    map.insert("g", "g");
    map.insert("G", "gh");
    map.insert("N", "ṅ");
    map.insert("c", "c");
    map.insert("C", "ch");
    map.insert("j", "j");
    map.insert("J", "jh");
    map.insert("Y", "ñ");
    map.insert("w", "ṭ");
    map.insert("W", "ṭh");
    map.insert("q", "ḍ");
    map.insert("Q", "ḍh");
    map.insert("R", "ṇ");
    map.insert("t", "t");
    map.insert("T", "th");
    map.insert("d", "d");
    map.insert("D", "dh");
    map.insert("n", "n");
    map.insert("p", "p");
    map.insert("P", "ph");
    map.insert("b", "b");
    map.insert("B", "bh");
    map.insert("m", "m");
    map.insert("y", "y");
    map.insert("r", "r");
    map.insert("l", "l");
    map.insert("v", "v");
    map.insert("S", "ś");
    map.insert("z", "ṣ");
    map.insert("s", "s");
    map.insert("h", "h");

    // Add mappings for other symbols
    map.insert("M", "ṃ");
    map.insert("H", "ḥ");
    map.insert("'", "'");
    map.insert(" ", " ");

    map
}
