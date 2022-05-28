use std::collections::HashMap;

#[derive(Debug)]
struct Noun {
    name: String,
}

#[derive(Debug)]
struct Adjective {
    name: String,

}

#[derive(Debug)]
struct Pronoun {
    name: String,

}

#[derive(Debug)]
struct Verb {
    name: String,
}

#[derive(Debug)]
struct Adverb {
    name: String,
}

#[derive(Debug)]
struct Preposition {
    name: String,
}

#[derive(Debug)]
struct Conjunction {
    name: String,
}

#[derive(Debug)]
struct Interjection {
    name: String,
}

#[derive(Debug)]
struct AnswerKey {
    name: String,
}

trait PartOfSpeech {
    fn description(&self) -> String;
}

impl PartOfSpeech for Noun {
    fn description(&self) -> String {
        "who or what?".to_string()
    }
}

impl PartOfSpeech for Adjective {
    fn description(&self) -> String {
        "how the NOUN or PRONOUN looks".to_string()
    }
}

impl PartOfSpeech for Pronoun {
    fn description(&self) -> String {
        "who or what?".to_string()
    }
}

impl PartOfSpeech for Verb {
    fn description(&self) -> String {
        "how the NOUN or PRONOUN does".to_string()
    }
}

impl PartOfSpeech for Adverb {
    fn description(&self) -> String {
        "how the VERBS acts".to_string()
    }
}

impl PartOfSpeech for Preposition {
    fn description(&self) -> String {
        "how PREPOSITION and CONJUNCTION connect the words".to_string()
    }
}

impl PartOfSpeech for Interjection {
    fn description(&self) -> String {
        "bhoooooooooo?".to_string()
    }
}


fn main() {

    let nouns = vec!([
        Noun { name: "bikini".to_string() },
        Noun { name: "locker room".to_string() },
        Noun { name: "towel".to_string() },
        Noun { name: "life preserver".to_string() },
        Noun { name: "beach ball".to_string() },
        Noun { name: "suntan lotion".to_string() },
        Noun { name: "sunglasses".to_string() },
        Noun { name: "pool".to_string() },
        Noun { name: "flippers".to_string() },
        Noun { name: "umbrella".to_string() },
        Noun { name: "bathing suit".to_string() },
        Noun { name: "lifegurd chair".to_string() },
        Noun { name: "mexican hat".to_string() },
        Noun { name: "air mattress".to_string() },
        Noun { name: "diving board".to_string() },
        Noun { name: "whistle".to_string() },
        Noun { name: "pool ladder".to_string() },
        Noun { name: "hamburger".to_string() },
        Noun { name: "hot dog".to_string() },
        Noun { name: "goggles".to_string() },
    ]);

    println!("{:?}", nouns);
}
