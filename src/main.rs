

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

    let noun = Noun {
        name: "ciao".to_string()
    };

    let noun = Adjective {
        name: "ciao".to_string()
    };


    println!("{:?}", noun);
}
