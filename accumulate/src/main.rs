pub fn accumulate<F, T, U>(input: Vec<T>, mut f: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    let mut result = Vec::with_capacity(input.len());
    for item in input {
        result.push(f(item))
    }
    result
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_square_numbers() {
//         let input = vec![1, 2, 3, 4, 5];
//         let expected = vec![1, 4, 9, 16, 25];
//         assert_eq!(accumulate_basic(input.clone(), |x| x * x), expected);
//     }
// }