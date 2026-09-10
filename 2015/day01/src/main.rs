use std::fs;

fn calc_floor(input: &str) -> i32 {
    input.chars().map(|c| if c == '(' { 1 } else { -1 }).sum()
}

fn enter_sub(input: &str) -> Option<usize> {
    let mut floor = 0;
    input.chars().enumerate().find_map(|(index, c)| {
        floor += if c == '(' { 1 } else { -1 };
        if floor == -1 {
            Some(index + 1)
        } else {
            None
        }
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    let input = input.trim();

    println!("Parte 1: {}", calc_floor(input));
    println!("Parte 2: {:?}", enter_sub(input));

    Ok(())
}

// ========== TESTES ==========
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step01() {
        assert_eq!(calc_floor("(())"), 0);
        assert_eq!(calc_floor("()()"), 0);
        assert_eq!(calc_floor("((("), 3);
        assert_eq!(calc_floor("(()(()("), 3);
        assert_eq!(calc_floor("))((((("), 3);
        assert_eq!(calc_floor("())"), -1);
        assert_eq!(calc_floor("))("), -1);
        assert_eq!(calc_floor(")))"), -3);
        assert_eq!(calc_floor(")())())"), -3);
    }

    #[test]
    fn test_step02() {
        assert_eq!(enter_sub(")"), Some(1));
        assert_eq!(enter_sub("()())"), Some(5));
        assert_eq!(enter_sub("((("), None);
        assert_eq!(enter_sub("())"), Some(3));
    }
}
