fn main() {
    let words: Vec<String> = vec![
        "ada".into(),
        "haskell".into(),
        "scala".into(),
        "java".into(),
        "rust".into(),
    ];
    let result = rank_words(&words);
    println!("Result: {:?}", result);
    let result = rank_words_eirik(&words, |w| score(w) + bonus(w) - penalty(w));
    println!("Result: {:?}", result);
}

fn score(word: &str) -> i32 {
    word.replace('a', "").len() as i32
}

fn bonus(word: &str) -> i32 {
    if word.contains('r') {
        5
    } else {
        0
    }
}

fn penalty(word: &str) -> i32 {
    if word.contains('j') {
        7
    } else {
        0
    }
}

fn rank_words(words: &[String]) -> Vec<String> {
    let mut items = words.to_vec();
    items.sort_by(|a, b| score(b).cmp(&score(a)));
    items
}

fn rank_words_eirik<F>(words: &[String], word_score: F) -> Vec<String>
where
    F: Fn(&str) -> i32,
{
    let mut items = words.to_owned();
    items.sort_by(|a, b| word_score(b).cmp(&word_score(a)));
    items
}
