use std::collections::HashMap;

pub fn display_hash() {
    let mut heros = HashMap::new();
    heros.insert("Superman", "Clark Kent");
    heros.insert("Batman", "Bruce Wayne");

    if heros.contains_key(&"Batman") {
        let the_batman = heros.get(&"Batman");
        match the_batman {
            Some(_) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }

    for (k, v) in heros.iter() {
        println!("{} = {}", k, v);
    }
}
