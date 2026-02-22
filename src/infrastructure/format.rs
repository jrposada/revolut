pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

pub fn format_number_signed(n: i64) -> String {
    if n < 0 {
        format!("-{}", format_number(n.unsigned_abs()))
    } else {
        format_number(n as u64)
    }
}
