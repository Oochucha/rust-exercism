pub fn annotate(garden: &[&str]) -> Vec<String> {
    let height = garden.len();
    if (height == 0) {
        return vec![];
    }

    let width = garden[0].len();
    let mut result = Vec::with_capacity(height);

    for r in 0..height {
        let row_bytes = garden[r].as_bytes();
        let mut out_row = Vec::with_capacity(width);

        for c in 0..width {
            if row_bytes[c] == b'*' {
                // This is a flower — copy it exactly.
                out_row.push(b'*');
                continue;
            }

            // Count adjacent flowers.
            let mut count = 0;

            for dr in [-1i32, 0, 1] {
                for dc in [-1i32, 0, 1] {
                    if dr == 0 && dc == 0 {
                        continue;
                    }

                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;

                    if nr >= 0
                        && nr < height as i32
                        && nc >= 0
                        && nc < width as i32
                        && garden[nr as usize].as_bytes()[nc as usize] == b'*'
                    {
                        count += 1;
                    }
                }
            }

            if count == 0 {
                out_row.push(b' ');
            } else {
                out_row.push(b'0' + count as u8);
            }
        }

        // Convert row of bytes → String (safe because ASCII).
        result.push(String::from_utf8(out_row).unwrap());
    }

    result   
}
