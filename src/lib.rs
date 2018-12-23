mod tests;

use regex::Regex;

// POTENTIAL IMPOVEMENTS:
// - CODE:
// --- TODO: Cleanup unused code and old comments
// - FUNCTIONALITY:
// --- MOAR TESTS
// TODO: cmd option -o <filename> that writes output to file

pub fn syllables_in_word(s: &str) -> usize {
    assert!(
        !s.contains(' '),
        "One word at a time! Did you trim all non alpha characters?"
    );

    // Remove all non alpha characters
    let regex_alpha = Regex::new(r"[^\w\s]").unwrap();
    let word = regex_alpha.replace_all(s, "").to_lowercase();

    let mut count;

    // The number of syllables in a word is tied to the number of "vowel groups".
    // Start by finding the number of vowel groups, before applying special rules.
    {
        let re = Regex::new(r"([aeiuo]+)").unwrap();
        let vowel_groups = re.captures_iter(&word).count();

        count = vowel_groups;
    }

    // Check for y in words following certain consonants.
    // Note: Might be missing a few consonants.
    // Examples: rowdy, lowly, every
    {
        let re = Regex::new(r"([dlmnrstz]y)").unwrap();
        let y_count = re.captures_iter(&word).count();
        count += y_count;
    }

    // Check for cases of aXe(s), which constitute 1 syllable despite having 2 vowel groups.
    // Examples: case, race, chase, promise, dice, lice, choose, st[ate]sman
    {
        let re = Regex::new(r"([aiou][b-df-hj-np-rs-v-z]e)").unwrap();
        let axe_count = re.captures_iter(&word).count();
        count -= axe_count;
    }

    // Check for *exceptions* to cases of aXe(s), which do actually constitute 2 syllables.
    // Seems to only apply when the final s is present and X is c or s. May be missing X's though.
    // Examples: cases (2) vs case (1), races (2) vs race (1)
    {
        let re = Regex::new(r"([aiou][cs]es)").unwrap();
        let axes_count = re.captures_iter(&word).count();
        count += axes_count;
    }

    // Acknowledge special cases of aste.
    // Examples: paste, haste
    {
        let re = Regex::new(r"(aste)").unwrap();
        if re.captures_iter(&word).count() == 1 {
            count -= 1;
        }
    }

    // Acknowledge special cases of apse.
    // Examples: lapse, collapse
    {
        let re = Regex::new(r"(apse)").unwrap();
        if re.captures_iter(&word).count() == 1 {
            count -= 1;
        }
    }

    // Add syllable for instances of ted, for which a syllable is removed in one of the earlier regexes.
    // Examples: rated (2) vs rate (1)
    // Exceptions: wanted (2)
    {
        let re = Regex::new(r"([^b-df-hj-np-tv-z]ted$)").unwrap();
        if re.captures_iter(&word).count() == 1 {
            count += 1;
        }
    }

    // Remove syllable for every instance of eXe.
    // Examples: eye, eve
    {
        let re = Regex::new(r"(e[rsvy]e)").unwrap();
        let exe_count = re.captures_iter(&word).count();
        count -= exe_count;
    }

    // Add syllable for contractions
    // Examples: coulDNt, woulDVe
    {
        let re = Regex::new(r"(d[nv])").unwrap();
        let contraction_count = re.captures_iter(&word).count();
        count += contraction_count;
    }

    // Rule for 'elve'
    // Examples: selves
    // Exceptions: e.g. velvet
    {
        let re = Regex::new(r"(elve[^t])").unwrap();
        let elve_count = re.captures_iter(&word).count();
        count -= elve_count;
    }

    // Add syllable for e + ing
    // Examples: being, seeing
    {
        let re = Regex::new(r"(eing)").unwrap();
        if re.captures_iter(&word).count() == 1 {
            count += 1;
        }
    }

    count
}

pub fn syllables_in_words(text: &str) -> usize {
    // Remove non alpha characters before splitting words by whitespace
    let re = Regex::new(r"[^\w\s]").unwrap();
    let text = re.replace_all(text, "").to_string();
    let words = text.split_whitespace();

    let mut count = 0;
    for word in words {
        count += syllables_in_word(word);
    }

    count
}
