use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let parts: Vec<&str> = input.split('=').collect();
    if parts.len() != 2 {
        return None;
    }
    let lhs = parts[0];
    let rhs = parts[1];

    let summands: Vec<&str> = lhs.split('+').collect();
    if summands.is_empty() {
        return None;
    }
    let mut unique_letters = Vec::new();
    for word in summands.iter().chain(std::iter::once(&rhs)) {
        for c in word.chars() {
            if !unique_letters.contains(&c) {
                unique_letters.push(c);
            }
        }
    }

    if unique_letters.len() > 10 {
        return None;
    }

    let digits: Vec<u8> = (0..10).collect();
    let all_permutations = permutations_of(&digits, unique_letters.len());

    'outer: for perm in all_permutations {
        let candidate: HashMap<char, u8> = unique_letters
            .iter()
            .cloned()
            .zip(perm.iter().cloned())
            .collect();

        for word in summands.iter().chain(std::iter::once(&rhs)) {
            if word.len() > 1 {
                let first_letter = word.chars().next().unwrap();
                if candidate[&first_letter] == 0 {
                    continue 'outer;
                }
            }
        }

        let sum_lhs: u64 = summands
            .iter()
            .map(|&word| word_to_num(word, &candidate))
            .sum();
        let val_rhs = word_to_num(rhs, &candidate);

        if sum_lhs == val_rhs {
            return Some(candidate);
        }
    }

    None
}

fn word_to_num(word: &str, mapping: &HashMap<char, u8>) -> u64 {
    let mut value = 0;
    for c in word.chars() {
        value = value * 10 + mapping[&c] as u64;
    }
    value
}
fn permutations_of(items: &[u8], k: usize) -> Vec<Vec<u8>> {
    let mut results = Vec::new();
    let mut chosen = Vec::with_capacity(k);
    let mut used = vec![false; items.len()];

    fn backtrack(
        items: &[u8],
        k: usize,
        chosen: &mut Vec<u8>,
        used: &mut [bool],
        results: &mut Vec<Vec<u8>>,
    ) {
        if chosen.len() == k {
            results.push(chosen.clone());
            return;
        }
        for i in 0..items.len() {
            if !used[i] {
                used[i] = true;
                chosen.push(items[i]);
                backtrack(items, k, chosen, used, results);
                chosen.pop();
                used[i] = false;
            }
        }
    }

    backtrack(items, k, &mut chosen, &mut used, &mut results);
    results
}