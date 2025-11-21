pub fn is_armstrong_number(num: u32) -> bool {
    if num < 10 {
        return true;
    }

    // Convert number to digits without allocation
    let mut digits = [0u8; 10]; // max digits for u32
    let mut len = 0;
    let mut n = num;

    while n > 0 {
        digits[len] = (n % 10) as u8;
        n /= 10;
        len += 1;
    }

    let mut sum: u64 = 0;
    for i in 0..len {
        sum += (digits[i] as u64).pow(len as u32);
    }

    sum == num as u64
}
