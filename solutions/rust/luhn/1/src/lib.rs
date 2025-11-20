/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut digits: Vec<u8> = Vec::with_capacity(code.len());

    for b in code.as_bytes() {
        match b {
            b' ' => continue,    // skip spaces
            b'0'..=b'9' => digits.push(b - b'0'),
            _ => return false,   // invalid char
        }
    }

    // Must be at least 2 characters
    if digits.len() <= 1 {
        return false;
    }

    let mut sum: u32 = 0;
    let mut double = false;

    // Traverse from right -> left
    for &d in digits.iter().rev() {
        let mut val = d as u32;

        if double {
            val *= 2;
            if val > 9 {
                val -= 9;
            }
        }

        sum += val;
        double = !double;
    }

    sum % 10 == 0
}
