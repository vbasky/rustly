use std::collections::HashMap;

pub fn slp1_to_iast(input: &str) -> String {
    let map = [
        ("a", "ā"),
        ("A", "Ā"),
        ("i", "ī"),
        ("I", "Ī"),
        ("u", "ū"),
        ("U", "Ū"),
        ("f", "ḥ"),
        ("x", "ñ"),
        ("e", "ē"),
        ("E", "Ē"),
        ("o", "ō"),
        ("O", "Ō"),
    ];
    let mut transliteration_map = HashMap::new();
    for &(slp1, iast) in map.iter() {
        transliteration_map.insert(slp1, iast);
    }

    let mut result = String::new();
    for character in input.chars() {
        if let Some(iast_char) = transliteration_map.get(&character.to_string() as &str) {
            result.push_str(iast_char);
        } else {
            result.push(character);
        }
    }
    result
}
