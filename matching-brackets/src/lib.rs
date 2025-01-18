// // stack-based
// pub fn brackets_are_balanced(string: &str) -> bool {
//     let mut stack = Vec::new();
//     for c in string.chars() {
//         match c {
//             '(' | '[' | '{' => stack.push(c),
//             ')' => if stack.pop() != Some('(') { return false; },
//             ']' => if stack.pop() != Some('[') { return false; },
//             '}' => if stack.pop() != Some('{') { return false; },
//             _ => (),
//         }
//     }
//     stack.is_empty()
// }

// // hashmap-based
// use std::collections::HashMap;

// fn brackets_are_balanced(s: &str) -> bool {
//     let mut brackets = HashMap::new();
//     brackets.insert(')', '(');
//     brackets.insert(']', '[');
//     brackets.insert('}', '{');

//     let mut stack = Vec::new();

//     for c in s.chars() {
//         if "([{".contains(c) {
//             stack.push(c);
//         } else if let Some(&opening) = brackets.get(&c) {
//             if stack.pop() != Some(opening) {
//                 return false;
//             }
//         }
//     }

//     stack.is_empty()
// }

// //string reduction

// fn brackets_are_balanced(s: &str) -> bool {
//     let mut result = String::new();

//     for c in s.chars() {
//         match c {
//             '(' | '[' | '{' | ')' | ']' | '}' => {
//                 result.push(c);
//                 while result.contains("()") || result.contains("[]") || result.contains("{}") {
//                     result = result.replace("()", "")
//                                .replace("[]", "")
//                                .replace("{}", "");
//                 }
//             }
//             _ => continue,
//         }
//     }

//     result.is_empty()
// }

// using recursion

fn brackets_are_balanced(s: &str) -> bool {
    fn find_matching_pair(chars: &[char], start: usize) -> Option<usize> {
        let mut count = 1;
        let opening = chars[start];
        let closing = match opening {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            _ => return None,
        };

        for (i, &c) in chars[start + 1..].iter().enumerate() {
            if c == opening { count += 1; }
            if c == closing { count -= 1; }
            if count == 0 { return Some(start + i + 1); }
        }
        None
    }

    fn check_balance(chars: &[char]) -> bool {
        if chars.is_empty() { return true; }

        let first = chars[0];
        if !"([{".contains(first) {
            return check_balance(&chars[1..]);
        }

        if let Some(matching_pos) = find_matching_pair(chars, 0) {
            check_balance(&chars[1..matching_pos]) &&
            check_balance(&chars[matching_pos + 1..])
        } else {
            false
        }
    }

    check_balance(&s.chars().collect::<Vec<_>>())
}
fn main() {
    println!("{}", brackets_are_balanced("()"));
    println!("{}", brackets_are_balanced("()[]{}"));
    println!("{}", brackets_are_balanced("(]"));
    println!("{}", brackets_are_balanced("([)]"));
    println!("{}", brackets_are_balanced("{[]}"));
}


