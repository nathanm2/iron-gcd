fn gcd(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    while a != 0 {
        if a < b {
            let t = a;
            a = b;
            b = t;
        }
        a = a % b
    }
    b
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   2 * 7 * 11 * 13 * 19), 2 * 11)
}

fn main() {
    println!("Hello, world!");
}
