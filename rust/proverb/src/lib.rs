
struct Sentence {
    key: &'static str,
    sentence: &'static str,
    ending: &'static str,
}

const ENDING_1: &'static str = &"And all for the want of a nail.";
const ENDING_2: &'static str = &"And all for the want of a horseshoe nail.";

const SENTENCES: [Sentence; 7] = [
    Sentence {
        key: &"nail",
        sentence: &"",
        ending: ENDING_1,
    },
    Sentence {
        key: &"shoe",
        sentence: &"For want of a nail the shoe was lost.",
        ending: ENDING_1,
    },
    Sentence {
        key: &"horse",
        sentence: &"For want of a shoe the horse was lost.",
        ending: ENDING_2,
    },
    Sentence {
        key: &"rider",
        sentence: &"For want of a horse the rider was lost.",
        ending: ENDING_2,
    },
    Sentence {
        key: &"message",
        sentence: &"For want of a rider the message was lost.",
        ending: ENDING_2,
    },
    Sentence {
        key: &"battle",
        sentence: &"For want of a message the battle was lost.",
        ending: ENDING_2,
    },
    Sentence {
        key: &"kingdom",
        sentence: &"For want of a battle the kingdom was lost.",
        ending: ENDING_2,
    },
];


pub fn build_proverb(keywords: Vec<&str>) -> String {
    let keywords = keywords.into_iter().rev().enumerate().rev();
    SENTENCES
        .iter()
        .zip(keywords)
        .take_while(|&(s, (i, key))| s.key == key)
        .flat_map(|(s, (i, key))| if i == 0 {
            vec![s.sentence, s.ending]
        } else {
            vec![s.sentence]
        })
        .filter(|&s| !s.is_empty())
        .collect::<Vec<&'static str>>()
        .join("\n")
}
