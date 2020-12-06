struct Solution;
impl Solution {
    pub fn decode_string(s: String) -> String {
        Self::decode(&mut s.chars())
    }

    fn decode(mut iter: &mut std::str::Chars) -> String {
        let mut n = String::new();
        let mut s = String::new();
        while let Some(c) = iter.next() {
            match c {
                '[' => {
                    let r = Self::decode(&mut iter);
                    s.push_str(&r.repeat(n.parse::<usize>().unwrap_or(1)));
                    n = String::new(); // reset `n`, because `n` is right before `[`
                }
                ']' => {
                    break;
                }
                '0'..='9' => {
                    n.push(c);
                }
                c => {
                    s.push(c);
                }
            }
        }
        s.repeat(n.parse::<usize>().unwrap_or(1))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!("aaabcbc", Solution::decode_string("3[a]2[bc]".into()));
        assert_eq!("accaccacc", Solution::decode_string("3[a2[c]]".into()));
        assert_eq!("abcabccdcdcdef", Solution::decode_string("2[abc]3[cd]ef".into()));
        assert_eq!("abccdcdcdxyz", Solution::decode_string("abc3[cd]xyz".into()));
        assert_eq!("zzzyypqjkjkefjkjkefjkjkefjkjkefyypqjkjkefjkjkefjkjkefjkjkefef", Solution::decode_string("3[z]2[2[y]pq4[2[jk]e1[f]]]ef".into()));
    }
}
