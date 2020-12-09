//! # 392. 判断子序列
//!
//! 难度 简单
//!
//! 给定字符串 s 和 t ，判断 s 是否为 t 的子序列。
//!
//! 你可以认为 s 和 t 中仅包含英文小写字母。字符串 t 可能会很长（长度 ~= 500,000），而 s 是个短字符串（长度 <=100）。
//!
//! 字符串的一个子序列是原始字符串删除一些（也可以不删除）字符而不改变剩余字符相对位置形成的新字符串。（例如，"ace"是"abcde"的一个子序列，而"aec"不是）。
//!
//! ## 示例 1:
//!
//! ```text
//! s = "abc", t = "ahbgdc"
//! 返回 true.
//! ```
//!
//! ## 示例 2:
//!
//! ```text
//! s = "axc", t = "ahbgdc"
//! 返回 false.
//! ```
//!
//! ## 后续挑战 :
//!
//! 如果有大量输入的 S，称作 S1, S2, ... , Sk
//! 其中 k >= 10 亿，你需要依次检查它们是否为 T 的子序列。
//!
//! 在这种情况下，你会怎样改变代码？
//!
//! See [leetcode](https://leetcode-cn.com/problems/is-subsequence/)

pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        // let matching = Matching::new(t);
        // matching.is_match(s)
        Self::simple_match(&s, &t)
    }

    /// 双指针
    pub fn simple_match(s: &str, t: &str) -> bool {
        let (mut i, mut j) = (0, 0);
        let (s, t) = (s.chars().collect::<Vec<_>>(), t.chars().collect::<Vec<_>>());
        let (s_len, t_len) = (s.len(), t.len());
        while i < s_len && j < t_len {
            if s[i] == t[j] {
                i += 1;
            }
            j += 1;
        }

        i == s_len
    }
}

/// 后续挑战
/// 预处理字符串
pub struct Matching {
    pos: Vec<[Option<usize>; 26]>,
}

impl Matching {

    pub fn new(target: String) -> Self {
        let len = target.len();
        let t: Vec<_> = target.chars().collect();
        let mut pos: Vec<[Option<usize>; 26]> = Vec::with_capacity(len);
        while pos.len() < len {
            pos.push([None; 26]);
        }

        for i in (0..len).rev() {
            for j in 0..26usize {
                if t[i] as usize == j + 'a' as usize {
                    pos[i][j] = Some(i);
                } else {
                    pos[i][j] = if i + 1 < len {
                        pos[i + 1][j]
                    } else {
                        None
                    };
                }
            }
        }

        Self {
            pos,
        }
    }

    pub fn is_match(&self, s: String) -> bool {
        if s.len() > self.pos.len() {
            return false;
        }
        let mut x = 0;
        for c in s.chars() {
            let char_offset = c as usize - 'a' as usize;
            if let Some(v) = self.pos.get(x).and_then(|v| v[char_offset]) {
                x = v + 1;
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();
        assert!(Solution::simple_match(&s, &t));
        assert!(Solution::is_subsequence(s, t));

        let s = "axc".to_string();
        let t = "ahbgdc".to_string();
        assert!(!Solution::simple_match(&s, &t));
        assert!(!Solution::is_subsequence(s, t));

        let s = "axc".to_string();
        let t = "".to_string();
        assert!(!Solution::simple_match(&s, &t));
        assert!(!Solution::is_subsequence(s, t));

        let s = "aaaaaa".to_string();
        let t = "bbaaaa".to_string();
        assert!(!Solution::simple_match(&s, &t));
        assert!(!Solution::is_subsequence(s, t));

        let s = "twn".to_string();
        let t = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxtxxxxxxxxxxxxxxxxxxxxwxxxxxxxxxxxxxxxxxxxxxxxxxn".to_string();
        assert!(Solution::simple_match(&s, &t));
        assert!(Solution::is_subsequence(s, t));
    }
}
