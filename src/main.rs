struct Solution;
impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        println!("n: {}", n);
        let mut rest = n as i64;
        let mut range: i64 = 9;
        let mut digits: i32 = 1;

        while rest - range * digits as i64 > 0 {
            rest = rest - range * digits as i64;
            digits += 1;
            range = range * 10;
        }
        let rest = rest as i32;

        let base = if digits > 1 {
            let mut base = 1;
            for _ in 1..digits {
                base *= 10;
            }
            base
        } else {
            0
        };

        println!("rest: {}, range: {}, digits: {}, base: {}", rest, range, digits, base);
        if digits == 1 {
            return rest;
        }

        let mut target = rest / digits + base;
        let rem = rest % digits;
        if rem == 0 {
            target = target - 1;
            return target % 10;
        }

        println!("target: {}, rem: {}", target, rem);


        let mut x = target;
        let mut y = 0;
        for _ in 0..(digits + 1 - rem) {
            y = x % 10;
            x = x / 10;
            println!("y: {}, x: {}", y, x);
        }
        y
    }
}
fn main() {
    let y = Solution::find_nth_digit(365);
    println!("y: {}", y);
    let y = Solution::find_nth_digit(13);
    println!("y: {}", y);
    let y = Solution::find_nth_digit(3);
    println!("y: {}", y);
    let y = Solution::find_nth_digit(1000000000);
    println!("y: {}", y);
}
