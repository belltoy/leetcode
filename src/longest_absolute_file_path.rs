//! # 388. 文件的最长绝对路径
//!
//! 难度 中等
//!
//! 假设文件系统如下图所示：![](https://assets.leetcode.com/uploads/2020/08/28/mdir.jpg)
//!
//!
//! 这里将 `dir` 作为根目录中的唯一目录。`dir` 包含两个子目录 `subdir1` 和 `subdir2` 。`subdir1` 包含文件 `file1.ext` 和子目录 `subsubdir1`；`subdir2` 包含子目录 `subsubdir2`，该子目录下包含文件 `file2.ext` 。
//!
//! 在文本格式中，如下所示(⟶表示制表符)：
//!
//! ```text
//! dir
//! ⟶ subdir1
//! ⟶ ⟶ file1.ext
//! ⟶ ⟶ subsubdir1
//! ⟶ subdir2
//! ⟶ ⟶ subsubdir2
//! ⟶ ⟶ ⟶ file2.ext
//! ```
//! 如果是代码表示，上面的文件系统可以写为 `"dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"` 。`'\n'` 和 `'\t'` 分别是换行符和制表符。
//!
//! 文件系统中的每个文件和文件夹都有一个唯一的 绝对路径 ，即必须打开才能到达文件/目录所在位置的目录顺序，所有路径用 `'/'` 连接。上面例子中，指向 `file2.ext` 的绝对路径是 `"dir/subdir2/subsubdir2/file2.ext"` 。每个目录名由字母、数字和/或空格组成，每个文件名遵循 `name.extension` 的格式，其中名称和扩展名由字母、数字和/或空格组成。
//!
//! 给定一个以上述格式表示文件系统的字符串 `input` ，返回文件系统中 **指向文件的最长绝对路径** 的长度。 如果系统中没有文件，返回 `0`。
//!
//!
//! ## 示例 1：
//!
//! ![](https://assets.leetcode.com/uploads/2020/08/28/dir1.jpg)
//!
//! ```text
//! 输入：input = "dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext"
//! 输出：20
//! 解释：只有一个文件，绝对路径为 "dir/subdir2/file.ext" ，路径长度 20
//! 路径 "dir/subdir1" 不含任何文件
//! ```
//!
//! ## 示例 2：
//!
//! ![](https://assets.leetcode.com/uploads/2020/08/28/dir2.jpg)
//!
//! ```text
//! 输入：input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
//! 输出：32
//! 解释：存在两个文件：
//! "dir/subdir1/file1.ext" ，路径长度 21
//! "dir/subdir2/subsubdir2/file2.ext" ，路径长度 32
//! 返回 32 ，因为这是最长的路径
//! ```
//!
//! ## 示例 3：
//!
//! ```text
//! 输入：input = "a"
//! 输出：0
//! 解释：不存在任何文件
//! ```
//!
//! ## 示例 4：
//!
//! ```text
//! 输入：input = "file1.txt\nfile2.txt\nlongfile.txt"
//! 输出：12
//! 解释：根目录下有 3 个文件。
//! 因为根目录中任何东西的绝对路径只是名称本身，所以答案是 "longfile.txt" ，路径长度为 12
//! ```
//!
//! ## 提示：
//!
//! * 1 <= input.length <= 10<sup>4</sup>
//! * `input` 可能包含小写或大写的英文字母，一个换行符 `'\n'`，一个指表符 `'\t'`，一个点 `'.'`，一个空格 `' '`，和数字。
//!
//! See [leetcode](https://leetcode-cn.com/problems/longest-absolute-file-path/)

pub struct Solution;

impl Solution {

    /// 用 Rust 的 `Iterator` 写函数式的解法。
    ///
    /// 先把输入拆分，因为每个片断都包含了文件名，路径深度和目录文件类型，对片断预处理。因为字符串表示是顺序表示的，所以前后之间是需要处理的。详细步骤说明：
    ///
    /// * 按 `\n` 切分输入 `split('\n')`
    /// * 把每个 `\t\tfile_name` 映射成 `(level, file_name_length, is_file)`，这里的 `level` 从 `0` 开始（即根目录 level 为 `0`）
    ///     * 找到第一个不是 `\t` 的字符的位置，position 表示 `\t` 个数，即 level 层级，剩下的即文件名，我们取其长度即可，同时判断是否为文件（文件名中是否带 `.`）
    ///     * 如果没有 `\t` 说明是在根目录下的，即 `level` 为 `0`
    /// * 计算每一个目录/文件名的长度（根据上一步生成的 tuple），因为每次分析都需要用到之前的路径在哪一层，所以需要用到 state 状态（栈），计算结果是每个目录/文件的长度，所以用 `scan`
    ///     * 初始状态是 `[]`，这里的状态表示**上一次**的父目录的绝对路径长度，即 `[level_0_len, level_1_len, level_2_len, ...]`
    ///     * 进入到当前目录/文件，需要弹出上一次的父目录，直到 `parents` 里只剩当前目录/文件的父级（这里用 `split_off(at)` 来简化多次 `pop()`
    ///     * 当前级别的目录/文件的长度就是剩下的 `parents` 里最后一个父目录的绝对路径长度和 + 当前文件名长度
    ///     * 如果当前是目录，则当前目录长度 `push` 入栈，注意需要 `+ 1` 表示算上 `/`
    ///     * `scan` 函数的参数函数里，返回 `(abs_len, is_file`) （注：`scan` 函数相当于是 `fold` + `map`，或者说 `map` 带了个 state 状态）
    /// * 经过上一步，转换的结果已经变成 `(abs_len, is_file)` 了，只要过滤掉不是文件的项
    /// * 然后找出 `max` 就是结果了
    ///
    /// 这里除了 `scan` 中每一步过程中需要更新 `parents` 这个状态，其它地方没有任何的 mutable 变量。函数式编程的方式逻辑非常清晰，代码写起来也非常方便，可读性高，加上 immutable 使得安全性也非常好。关键是在 Rust 里用函数式的方法几乎不损失性能。
    pub fn length_longest_path(input: String) -> i32 {
        input
            .split('\n')
            .map(|s| {
                s.find(|c| c != '\t')
                .map(|n| (n, s.len() - n, (&s[n..]).chars().any(|c| c == '.')))
                .unwrap_or_else(|| (0, s.len(), s.chars().any(|c| c == '.')))
            })
            .scan(vec![], |parents, (level, s_len, is_file)| {
                if level < parents.len() {
                    let _ = parents.split_off(level); // pop until parent level
                }

                let abs_len = s_len + parents.last().unwrap_or(&0);

                if !is_file {
                    parents.push(abs_len + 1);
                }

                Some((abs_len, is_file))
            })
            .filter(|(_, is_file)| *is_file)
            .map(|(n, _)| n)
            .max()
            .unwrap_or(0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (20, "dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext"),
            (32, "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"),
            (0, "a"),
            (5, "a.txt"),
            (5, "b.t\na.txt"),
            (12, "file1.txt\nfile2.txt\nlongfile.txt"),
            (16, "dir\n        file.txt"),
            (14, "a\n\tb1\n\t\tf1.txt\n\taaaaa\n\t\tf2.txt"),
            (29, "a\n\taa\n\t\taaa\n\t\t\tfile1.txt\naaaaaaaaaaaaaaaaaaaaa\n\tsth.png"),
            (10, "a\n\tb\n\t\tc.txt\n\taaaa.txt"),
        ];

        for (expect, input) in cases {
            assert_eq!(expect, Solution::length_longest_path(input.into()));
        }
    }
}
