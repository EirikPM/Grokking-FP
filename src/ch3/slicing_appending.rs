fn first_two(list: &[String]) -> Vec<String> {
    list[0..2].to_vec()
}

fn last_two(list: &[String]) -> Vec<String> {
    list[list.len() - 2..].to_vec()
}

fn moved_first_two_to_the_end(list: &[String]) -> Vec<String> {
    let mut items: Vec<String> = Vec::with_capacity(list.len());
    items.extend_from_slice(&list[2..]);
    items.extend_from_slice(&list[0..2]);
    items
}

fn inserted_before_last(list: &[String], element: String) -> Vec<String> {
    let mut items: Vec<String> = Vec::with_capacity(list.len() + 1);
    items.extend_from_slice(&list[0..list.len() - 1]);
    items.push(element);
    items.push(list.last().unwrap().clone());
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_two_assert() {
        let list: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
        let result = first_two(&list);
        assert_eq!(result, vec!["a", "b"])
    }

    #[test]
    fn last_two_assert() {
        let list: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
        let result = last_two(&list);
        assert_eq!(result, vec!["b", "c"])
    }

    #[test]
    fn moved_first_two_to_the_end_assert() {
        let list: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
        let result = moved_first_two_to_the_end(&list);
        assert_eq!(result, vec!["c", "a", "b"])
    }

    #[test]
    fn inserted_before_last_assert() {
        let list: Vec<String> = vec!["a".into(), "b".into()];
        let result = inserted_before_last(&list, "c".into());
        assert_eq!(result, vec!["a", "c", "b"])
    }
}
