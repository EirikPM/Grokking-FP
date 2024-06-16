use std::cmp::Ordering;

pub fn score(word: &str) -> i32 {
    word.replace('a', "").len() as i32
}

pub fn score_comparator(w1: &str, w2: &str) -> Ordering {
    score(w2).cmp(&score(w1))
}

// Version 1: Mutate state
pub fn ranked_mut_words_with_hidden_flow(words: &mut Vec<String>) -> Vec<String> {
    words.sort_by(|a, b| score_comparator(a, b));
    words.to_vec()
}

// Version 2: Ranking word without mutating input-parameter
pub fn ranked_words_reference_only(words: &[String]) -> Vec<String> {
    let mut sorted_words = words.to_vec();
    sorted_words.sort_by(|a, b| score_comparator(a, b));
    sorted_words
}

// Version 3: Passing algorithms as arguments
pub fn ranked_words_with_comp_signature<F>(comparator: F, words: &[String]) -> Vec<String>
where
    F: Fn(&str, &str) -> Ordering,
{
    let mut sorted_words: Vec<String> = words.to_vec();
    sorted_words.sort_by(|a, b| comparator(a, b));
    sorted_words
}

// Version 4: Changing ranking algorithm. The old way of scoring should still be supported
pub fn score_with_bonus(word: &str) -> i32 {
    let base = score(word);
    if word.contains('c') {
        base + 5
    } else {
        base
    }
}

pub fn score_with_bonus_comparator(w1: &str, w2: &str) -> Ordering {
    score_with_bonus(w2).cmp(&score_with_bonus(w1))
}

// Use closures to deal with code duplication
fn closure_examples(words: &[String]) {
    let score_comparator = |w1: &str, w2: &str| score(w2).cmp(&score(w1));
    let score_with_bonus_comparator =
        |w1: &str, w2: &str| score_with_bonus(w2).cmp(&score_with_bonus(w1));

    // Old scoring system, passing comparator stored as a closure-variable and directly
    ranked_words_with_comp_signature(score_comparator, words);
    ranked_words_with_comp_signature(|w1, w2| score(w2).cmp(&score(w1)), words);

    // New scoring system, passing comparator stored as a closure-variable and directly
    ranked_words_with_comp_signature(score_with_bonus_comparator, words);
    ranked_words_with_comp_signature(
        |w1, w2| score_with_bonus(w2).cmp(&score_with_bonus(w1)),
        words,
    );
}

// Version 5: Passing the scoring function as an argument
pub fn ranked_words<F>(word_score: F, words: &[String]) -> Vec<String>
where
    F: Fn(&str) -> i32,
{
    let mut items = words.to_vec();
    items.sort_by(|a, b| word_score(b).cmp(&word_score(a)));
    items
}

// Coffee break: Add a penalty of 7 points if word contain the character 's'. Must still support the other scoring functions
fn bonus_points(word: &str) -> i32 {
    if word.contains('c') {
        5
    } else {
        0
    }
}

fn penalty_points(word: &str) -> i32 {
    if word.contains('s') {
        7
    } else {
        0
    }
}

// Return a list with the score for each word
fn word_scores<F>(words: &[String], word_score: F) -> Vec<i32>
where
    F: Fn(&str) -> i32,
{
    words.iter().map(|w| word_score(w)).collect()
}

fn high_scoring_words_imperative<F>(words: &[String], word_score: F) -> Vec<String>
where
    F: Fn(&str) -> i32,
{
    let mut result: Vec<String> = Vec::with_capacity(words.len());
    for word in words {
        if word_score(&word) > 1 {
            result.push(word.clone());
        }
    }
    result
}

fn high_scoring_words_declarative<F>(words: &[String], word_score: F) -> Vec<String>
where
    F: Fn(&str) -> i32,
{
    words
        .into_iter()
        .filter(|word| word_score(word) > 1)
        .cloned()
        .collect()
}

fn high_scoring_words_fn<F>(words: Vec<String>, word_score: F) -> impl Fn(i32) -> Vec<String>
where
    F: Fn(&str) -> i32,
{
    move |higher_than: i32| {
        words
            .iter()
            .filter(|word| word_score(word) > higher_than)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutates_states() {
        let original_state: Vec<String> = vec![
            "ada".into(),
            "haskell".into(),
            "scala".into(),
            "java".into(),
            "rust".into(),
        ];
        let mut words: Vec<String> = vec![
            "ada".into(),
            "haskell".into(),
            "scala".into(),
            "java".into(),
            "rust".into(),
        ];
        let result = ranked_mut_words_with_hidden_flow(&mut words);
        assert_eq!(result, words);
        assert_ne!(original_state, words);
    }

    #[test]
    fn pass_by_reference_do_not_mutate_state() {
        let words: Vec<String> = vec![
            "ada".into(),
            "haskell".into(),
            "scala".into(),
            "java".into(),
            "rust".into(),
        ];
        let result = ranked_words_reference_only(&words);
        let expected_result: Vec<String> = vec![
            "haskell".into(),
            "rust".into(),
            "scala".into(),
            "java".into(),
            "ada".into(),
        ];
        assert_eq!(result, expected_result);
        assert_ne!(result, words);
    }

    #[test]
    fn pass_comparator_as_argument() {
        let words: Vec<String> = vec![
            "ada".into(),
            "haskell".into(),
            "scala".into(),
            "java".into(),
            "rust".into(),
        ];
        let expected_result: Vec<String> = vec![
            "haskell".into(),
            "rust".into(),
            "scala".into(),
            "java".into(),
            "ada".into(),
        ];
        let result = ranked_words_with_comp_signature(score_comparator, &words);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn new_ranking_algorithm() {
        let words: Vec<String> = vec![
            "ada".into(),
            "haskell".into(),
            "scala".into(),
            "java".into(),
            "rust".into(),
        ];
        let expected_result: Vec<String> = vec![
            "scala".into(),
            "haskell".into(),
            "rust".into(),
            "java".into(),
            "ada".into(),
        ];

        let result = ranked_words_with_comp_signature(score_with_bonus_comparator, &words);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn new_ranking_algorithm_supports_legacy() {
        let words: Vec<String> = vec![
            "ada".into(),
            "haskell".into(),
            "scala".into(),
            "java".into(),
            "rust".into(),
        ];
        let expected_result: Vec<String> = vec![
            "haskell".into(),
            "rust".into(),
            "scala".into(),
            "java".into(),
            "ada".into(),
        ];

        let result = ranked_words_with_comp_signature(score_comparator, &words);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn passing_legacy_scoring_function_as_parameter() {
        let words: Vec<String> = vec![
            "ada".into(),
            "haskell".into(),
            "scala".into(),
            "java".into(),
            "rust".into(),
        ];
        let expected_result: Vec<String> = vec![
            "haskell".into(),
            "rust".into(),
            "scala".into(),
            "java".into(),
            "ada".into(),
        ];

        let result = ranked_words(score, &words);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn passing_new_scoring_function_as_parameter() {
        let words: Vec<String> = vec![
            "ada".into(),
            "haskell".into(),
            "scala".into(),
            "java".into(),
            "rust".into(),
        ];
        let expected_result: Vec<String> = vec![
            "scala".into(),
            "haskell".into(),
            "rust".into(),
            "java".into(),
            "ada".into(),
        ];

        let result = ranked_words(score_with_bonus, &words);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn penalty_score() {
        let words: Vec<String> = vec![
            "ada".into(),
            "haskell".into(),
            "scala".into(),
            "java".into(),
            "rust".into(),
        ];
        let expected_result: Vec<String> = vec![
            "java".into(),
            "ada".into(),
            "scala".into(),
            "haskell".into(),
            "rust".into(),
        ];
        let result = ranked_words(|w| score(w) + bonus_points(w) - penalty_points(w), &words);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn word_score() {
        let words: Vec<String> = vec![
            "ada".into(),
            "haskell".into(),
            "scala".into(),
            "java".into(),
            "rust".into(),
        ];
        let expected_result: Vec<i32> = vec![1, -1, 1, 2, -3];
        let result = word_scores(&words, |w| score(w) + bonus_points(w) - penalty_points(w));
        assert_eq!(result, expected_result);
    }

    #[test]
    fn high_scoring_words() {
        let words: Vec<String> = vec![
            "ada".into(),
            "haskell".into(),
            "scala".into(),
            "java".into(),
            "rust".into(),
        ];
        let expected_result: Vec<String> = vec!["java".into()];
        let result = high_scoring_words_declarative(&words, |w| {
            score(w) + bonus_points(w) - penalty_points(w)
        });
        assert_eq!(result, expected_result);
    }
}
