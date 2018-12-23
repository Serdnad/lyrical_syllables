#[cfg(test)]
use super::*;

#[test]
fn test_syllables_in_words() {
    let cases = vec![
        ("life", 1),
        ("can", 1),
        ("be", 1),
        ("hard", 1),
        ("but", 1),
        ("chocolate", 3),
        ("truly", 2),
        ("makes", 1),
        ("date", 1),
        ("belated", 3),
        ("it", 1),
        ("quite", 1),
        ("beautiful", 3),
        ("polymorphism", 4),
        ("hatred", 2),
        ("lucozade", 3),
        ("the", 1),
        ("baked", 1),
        ("named", 1),
        ("dated", 2),
        ("baited", 2),
        ("my", 1),
        ("autonomy", 4),
        ("smile", 1),
        ("smiles", 1),
        ("comprehensive", 4),
        ("comprehensives", 4),
        ("tests", 1),
        ("test", 1),
        ("haste", 1),
        ("queue", 1),
        ("rated", 2),
        ("everytime", 3),
        ("cry", 1),
        ("were", 1),
        ("eye", 1),
        ("couldnt", 2),
        ("collapse", 2),
        ("holes", 1),
        ("creatures", 2),
        ("themselves", 2),
        ("velvet", 2),
        ("selvedge", 2),
        ("these", 1),
        ("thesis", 2),
        ("being", 2),
        ("crazy", 2),
        ("wanted", 2),
        ("cased", 1),
        ("cases", 2),
        ("crises", 2),
        ("race", 1),
        ("races", 2),
        ("promise", 2)
    ];

    for case in cases {
        assert_eq!(
            syllables_in_word(case.0),
            case.1,
            "Failed for word: {}",
            case.0
        );
    }
}

#[test]
fn test_syllables_in_sentences() {
    let cases = vec![
        ("there are eight syllables in here", 8),
        ("to be or not to be is the    question??", 10),
        ("I should add more test cases to \n this!", 9),
        ("When you were here \tbefore", 6),
        ("Couldn't look you in the eye", 7),
        ("You're just like an angel", 6),
        ("Your skin makes me cry", 5),
    ];

    for case in cases {
        let count = syllables_in_words(case.0);

        assert_eq!(count, case.1, "Failed for: \n{}", case.0);
    }
}
