use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref VOWELS: HashMap<char, [char; 6]> = HashMap::from([
        ('a', ['a', 'ā', 'á', 'ǎ', 'à', 'a']),
        ('e', ['e', 'ē', 'é', 'ě', 'è', 'e']),
        ('i', ['i', 'ī', 'í', 'ǐ', 'ì', 'i']),
        ('o', ['o', 'ō', 'ó', 'ǒ', 'ò', 'o']),
        ('u', ['u', 'ū', 'ú', 'ǔ', 'ù', 'u']),
        ('ü', ['ü', 'ǖ', 'ǘ', 'ǚ', 'ǜ', 'ü'])
    ]);
}

/// convert a *single* numeric pinyin into unicode version
/// ```rust
/// let input = "xue4";
/// assert_eq!(numeral_to_unicode(input), "xuè");
/// ```
pub fn numeral_to_unicode(s: &str) -> String {
    let num = if let Some(num) = s.chars().last() {
        match num.to_digit(10) {
            Some(n) => n,
            None => return s.to_owned(),
        }
    } else {
        return s.to_owned();
    };
    let s = &s[..s.len() - 1];
    let mut s = s.replace('v', "ü");
    let vowel_count = count_vowels(&s);
    if vowel_count == 1 {
        let v = s.chars().rfind(|c| VOWELS.contains_key(c)).unwrap();
        // s.replace_range(
        //     vi..vi + 1,
        //     &VOWELS.get(&v).unwrap()[num as usize].to_string(),
        // );
        let s = s.replacen(v, &VOWELS.get(&v).unwrap()[num as usize].to_string(), 1);
        return s;
    }
    // more than one vowel
    if s.contains('a') {
        return s.replacen('a', &VOWELS.get(&'a').unwrap()[num as usize].to_string(), 1);
    }
    if s.contains('e') {
        return s.replacen('e', &VOWELS.get(&'e').unwrap()[num as usize].to_string(), 1);
    }
    if s.contains("ou") {
        // TODO this doesn't actually work if there are both ou and o in the same string, but
        // I don't think any character exists with that pinyin?
        return s.replacen('o', &VOWELS.get(&'o').unwrap()[num as usize].to_string(), 1);
    }
    let (vi, v) = s
        .char_indices()
        .filter(|c| VOWELS.contains_key(&c.1))
        .nth(1)
        .unwrap();
    // TODO this panics when replacing in the middle of a code point, which ü seems
    // to cause to happen. But this shouldn't fail unless there is a ü at the end
    // of the pinyin, which I don't think is ever the case
    s.replace_range(
        vi..vi + 1,
        &VOWELS.get(&v).unwrap()[num as usize].to_string(),
    );
    s
}

/// return the number of vowels in a given &str
fn count_vowels(s: &str) -> u32 {
    s.chars()
        .filter(|c| VOWELS.contains_key(c))
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::numeral_to_unicode;

    #[test]
    fn pinyin1() {
        let input = "kan4";
        let expected_output = "kàn";
        let output = numeral_to_unicode(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn pinyin2() {
        let input = "kuo3";
        let expected_output = "kuǒ";
        let output = numeral_to_unicode(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn pinyin3() {
        let input = "ai3";
        let expected_output = "ǎi";
        let output = numeral_to_unicode(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn pinyin4() {
        let input = "lv4";
        let expected_output = "lǜ";
        let output = numeral_to_unicode(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn pinyin5() {
        let input = "lve4";
        let expected_output = "lüè";
        let output = numeral_to_unicode(input);
        assert_eq!(output, expected_output);
    }
}
