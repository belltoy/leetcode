//! # 71. 简化路径
//!
//! 难度 中等
//!
//! 以 Unix 风格给出一个文件的绝对路径，你需要简化它。或者换句话说，将其转换为规范路径。
//!
//! 在 Unix 风格的文件系统中，一个点（.）表示当前目录本身；此外，两个点 （..） 表示
//! 将目录切换到上一级（指向父目录）；两者都可以是复杂相对路径的组成部分。
//! 更多信息请参阅：[Linux / Unix中的绝对路径 vs 相对路径](https://blog.csdn.net/u011327334/article/details/50355600)
//!
//! 请注意，返回的规范路径必须始终以斜杠 / 开头，并且两个目录名之间必须只有一个斜杠 /。
//! 最后一个目录名（如果存在）**不能**以 / 结尾。此外，规范路径必须是表示绝对路径的**最短**字符串。
//!
//! ## 示例 1：
//!
//! ```no_run
//! /// 输入："/home/"
//! /// 输出："/home"
//! /// 解释：注意，最后一个目录名后面没有斜杠。
//! use leetcode::simplify_path::Solution;
//! assert_eq!("/home", Solution::simplify_path("/home/".into()));
//! ```
//!
//! ## 示例 2：
//!
//! ```no_run
//! /// 输入："/../"
//! /// 输出："/"
//! /// 解释：从根目录向上一级是不可行的，因为根是你可以到达的最高级。
//! use leetcode::simplify_path::Solution;
//! assert_eq!("/", Solution::simplify_path("/../".into()));
//! ```
//!
//! ## 示例 3：
//!
//! ```no_run
//! /// 输入："/home//foo/"
//! /// 输出："/home/foo"
//! /// 解释：在规范路径中，多个连续斜杠需要用一个斜杠替换。
//! use leetcode::simplify_path::Solution;
//! assert_eq!("/home/foo", Solution::simplify_path("/home//foo/".into()));
//! ```
//!
//! ## 示例 4：
//!
//! ```no_run
//! /// 输入："/a/./b/../../c/"
//! /// 输出："/c"
//! use leetcode::simplify_path::Solution;
//! assert_eq!("/c", Solution::simplify_path("/a/./b/../../c/".into()));
//! ```
//!
//! ## 示例 5：
//!
//! ```no_run
//! /// 输入："/a/../../b/../c//.//"
//! /// 输出："/c"
//! use leetcode::simplify_path::Solution;
//! assert_eq!("/c", Solution::simplify_path("/a/../../b/../c//.//".into()));
//! ```
//!
//! ## 示例 6：
//!
//! ```no_run
//! /// 输入："/a//b////c/d//././/.."
//! /// 输出："/a/b/c"
//! use leetcode::simplify_path::Solution;
//! assert_eq!("/a/b/c", Solution::simplify_path("/a//b////c/d//././/..".into()));
//! ```
//!
//! See [leetcode](https://leetcode-cn.com/problems/simplify-path/)

pub struct Solution;

impl Solution {

    /// 使用 `Iterator` 操作会很简
    pub fn simplify_path(path: String) -> String {
        let paths = path
            .split('/')
            .filter(|&p| p != "." && !p.is_empty())
            .fold(vec![], |mut acc, p| {
                match p {
                    ".." => {
                        acc.pop();
                        acc
                    }
                    p => {
                        acc.push(p);
                        acc
                    }
                }
            })
            .as_slice()
            .join("/");

        "/".to_string() + &paths
    }
}
