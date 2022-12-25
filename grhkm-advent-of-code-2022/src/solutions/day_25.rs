use std::collections::HashMap;

pub fn solve(contents: &str) -> (String, String) {
    let digit_map: HashMap<char, i128> =
        [('0', 0), ('1', 1), ('2', 2), ('-', -1), ('=', -2)].into();

    let symbols_to_int = |syms: &str| -> i128 {
        let mut res = 0;
        for c in syms.chars() {
            res = res * 5 + digit_map[&c];
        }
        res
    };

    let int_to_symbols = |num: i128| -> String {
        // Assuming num > 0
        assert!(num > 0);
        let mut num = num;
        let mut syms = "".to_string();
        while num > 0 {
            // Find LSD by considering modulo 5
            for (sym, digit) in &digit_map {
                if (num - digit) % 5 == 0 {
                    syms.push(*sym);
                    num = (num - digit) / 5;
                    break;
                }
            }
        }
        syms.chars().rev().collect()
    };

    let part1 = int_to_symbols(contents.lines().map(symbols_to_int).sum());
    let part2 = "There is no part 2! Merry Christmas!".to_string();

    (part1, part2)
}
