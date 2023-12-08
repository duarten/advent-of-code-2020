pub mod bitvec;
pub mod list;

pub fn reverse(mut n: usize) -> usize {
    let mut reversed = 0;
    while n != 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }
    reversed
}

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}
