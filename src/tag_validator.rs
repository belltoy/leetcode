//! # 591. 标签验证器
//!
//! 难度 困难
//!
//! 给定一个表示代码片段的字符串，你需要实现一个验证器来解析这段代码，并返回它是否合法。合法的代码片段需要遵守以下的所有规则：
//!
//! 1. 代码必须被合法的闭合标签包围。否则，代码是无效的。
//! 2. 闭合标签（不一定合法）要严格符合格式：`<TAG_NAME>TAG_CONTENT</TAG_NAME>`。其中，`<TAG_NAME>` 是起始标签，`</TAG_NAME>` 是结束标签。起始和结束标签中的 `TAG_NAME` 应当相同。当且仅当 `TAG_NAME` 和 `TAG_CONTENT` 都是合法的，闭合标签才是合法的。
//! 3. 合法的 `TAG_NAME` 仅含有大写字母，长度在范围 `[1,9]` 之间。否则，该 `TAG_NAME` 是不合法的。
//! 4. 合法的 `TAG_CONTENT` 可以包含其他合法的闭合标签，`cdata` （请参考规则7）和任意字符（注意参考规则1）除了不匹配的 `<`、不匹配的起始和结束标签、不匹配的或带有不合法 `TAG_NAME` 的闭合标签。否则，`TAG_CONTENT` 是不合法的。
//! 5. 一个起始标签，如果没有具有相同 `TAG_NAME` 的结束标签与之匹配，是不合法的。反之亦然。不过，你也需要考虑标签嵌套的问题。
//! 6. 一个<，如果你找不到一个后续的 `>` 与之匹配，是不合法的。并且当你找到一个 `<` 或 `</` 时，所有直到下一个 `>` 的前的字符，都应当被解析为 `TAG_NAME`（不一定合法）。
//! 7. `cdata` 有如下格式：`<![CDATA[CDATA_CONTENT]]>` 。`CDATA_CONTENT` 的范围被定义成 `<![CDATA[ 和后续的第一个 ]]>` 之间的字符。
//! 8. `CDATA_CONTENT` 可以包含任意字符。`cdata` 的功能是阻止验证器解析 `CDATA_CONTENT`，所以即使其中有一些字符可以被解析为标签（无论合法还是不合法），也应该将它们视为常规字符。
//!
//! 合法代码的例子:
//!
//! ```text
//! 输入: "<DIV>This is the first line <![CDATA[<div>]]></DIV>"
//!
//! 输出: True
//!
//! 解释:
//!
//! 代码被包含在了闭合的标签内： <DIV> 和 </DIV> 。
//!
//! TAG_NAME 是合法的，TAG_CONTENT 包含了一些字符和 cdata 。
//!
//! 即使 CDATA_CONTENT 含有不匹配的起始标签和不合法的 TAG_NAME，它应该被视为普通的文本，而不是标签。
//!
//! 所以 TAG_CONTENT 是合法的，因此代码是合法的。最终返回True。
//!
//!
//! 输入: "<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>"
//!
//! 输出: True
//!
//! 解释:
//!
//! 我们首先将代码分割为： start_tag|tag_content|end_tag 。
//!
//! start_tag -> "<DIV>"
//!
//! end_tag -> "</DIV>"
//!
//! tag_content 也可被分割为： text1|cdata|text2 。
//!
//! text1 -> ">>  ![cdata[]] "
//!
//! cdata -> "<![CDATA[<div>]>]]>" ，其中 CDATA_CONTENT 为 "<div>]>"
//!
//! text2 -> "]]>>]"
//!
//!
//! start_tag 不是 "<DIV>>>" 的原因参照规则 6 。
//! cdata 不是 "<![CDATA[<div>]>]]>]]>" 的原因参照规则 7 。
//! ```
//!
//! 不合法代码的例子:
//!
//! ```text
//! 输入: "<A>  <B> </A>   </B>"
//! 输出: False
//! 解释: 不合法。如果 "<A>" 是闭合的，那么 "<B>" 一定是不匹配的，反之亦然。
//!
//! 输入: "<DIV>  div tag is not closed  <DIV>"
//! 输出: False
//!
//! 输入: "<DIV>  unmatched <  </DIV>"
//! 输出: False
//!
//! 输入: "<DIV> closed tags with invalid tag name  <b>123</b> </DIV>"
//! 输出: False
//!
//! 输入: "<DIV> unmatched tags with invalid tag name  </1234567890> and <CDATA[[]]>  </DIV>"
//! 输出: False
//!
//! 输入: "<DIV>  unmatched start tag <B>  and unmatched end tag </C>  </DIV>"
//! 输出: False
//! ```
//!
//! ## 注意:
//!
//! 为简明起见，你可以假设输入的代码（包括提到的任意字符）只包含`数字`, `字母`, `'<','>','/','!','[',']'` 和 `' '`。
//!
//! See [leetcode](https://leetcode-cn.com/problems/tag-validator/)

/// 用有限状态机，处理几个有限的状态就解决了。
///
/// Rust 的好处是很容易写比较高级的抽象。而且**对错误处理非常友好**。语法上主要是 Pattern Matching 和各种高阶函数的 combinator 组合。感觉用 Erlang/Elixir 写起来应该会很方便。
///
/// 代码逻辑：
///
/// 1. `chars()` 生成字符串的 [`Iterator`](Iterator)
/// 2. `try_fold` 消费这个 [`Iterator`](Iterator)，创建一个新的 `Validator` 对象作为 fold 的初始状态传入。
/// 3. `try_fold` 的每一次回调处理一个字符，调用 `validator.handle(c)` 返回一个 `Result`。 只要每步都 Ok，那就一直 handle 下去，只要中间发现 Err，就会提早返回 (短路操作)
/// 4. 最后消费完 [`Iterator`](Iterator) 也就是所有字符都处理完了，需要判断下 `Validator.is_end()` 是否合法结束，即检查 `Validator` 的状态是否是 `End`（完全闭合最外层的标签）。
///
/// 主要是 `Validator` 里的几个状态定义：
///
/// * `Init` 初始状态，只接收 `<` 就进入 `TagName` 状态，否则失败
/// * `TagName` 这个状态比较复杂，其实就是 Tag 的定义条件，这里用了一个 `is_close` 来标识是不是 **关闭标签**
/// * `TagContent` 这个状态简单，只要遇到 `<` 就进入 `TagName` 状态，其它都 Ok
/// * `CDataTag` 这个也只有一种情况，读取完整的 `[CDATA[` 进入 `CDataContent` 状态，否则失败
/// * `CDataContent` 这个状态也简单（用了两个 `bool` 标记是否已经有连续两个 `]` ），遇到 `]]>` 就结束，回到 `TagContent` 状态（注意 reset 标记）
/// * `End` 最外层的 Tag 闭合，结束，不能再有内容（即如果这个状态下再接收到内容，都是 Err）
///
/// 这里就 `TagName` 里处理得最多，具体条件结合题目定义，看代码。
///
/// 另外，`Validator` 里定义了一个 `stack`，记录 **开标签**，用于与 **闭标签** 匹配。
///
/// Rust 的类型系统使得错误处理机制非常严谨，安全。代码里每一个地方都记录了 invalid 的错误信息，可以在最外层调用的地方统一打印。
///
/// 因为用 [`Iterator`](Iterator)，是 lazy 调用，只有在每一步迭代的时候才会执行代码，只迭代了一遍。（如果要调试输入的情况，在 `try_fold`
/// 前面加一个 `inspect` 可以打印出每个字符，结合每个地方的状态变化，可以很轻松地调试）。
/// 因为代码抽象程度比较好，虽然看起来代码长了一点，但结构非常清晰，可读性，安全性，可扩展/维护性都很优秀。
pub struct Solution;

impl Solution {
    pub fn is_valid(code: String) -> bool {
        code.chars().try_fold(Validator::new(), |mut validator, c| {
            validator.handle(c).map(|_| validator)
        })
        .and_then(|validator| match validator.is_end() {
            true => Ok(()),
            _ => Err("EOF"),
        })
        // .map_err(|e| {
        //     eprintln!("Invalid, err: {}", e);
        // })
        .is_ok()
    }
}

type Result = std::result::Result<(), &'static str>;

enum State {
    Init,
    TagName {
        cache: String,
        is_close: bool,
    },
    TagContent,
    CDataTag {
        cache: String,
    },
    CDataContent {
        // stand for close prefix `]]`
        prefix: (bool, bool),
    },
    End,
}

struct Validator {
    state: State,
    stack: Vec<String>,
}

impl Validator {
    fn new() -> Self {
        Self {
            state: State::Init,
            stack: Vec::new(),
        }
    }
}

impl Validator {
    fn is_end(&self) -> bool {
        match &self.state {
            State::End => true,
            _ => false,
        }
    }

    fn handle(&mut self, c: char) -> Result {
        match &self.state {
            State::Init => self.handle_init(c),
            State::TagName { .. } => self.handle_tag_name(c),
            State::TagContent => self.handle_tag_content(c),
            State::CDataTag { .. } => self.handle_cdata_tag(c),
            State::CDataContent { .. } => self.handle_cdata_content(c),
            State::End => self.handle_end(c),
        }
    }

    // In the Init state, only accept '<' character then go into next state: TagName
    fn handle_init(&mut self, c: char) -> Result {
        match (&self.state, c) {
            (State::Init, '<') => {
                self.state = State::TagName {
                    cache: String::new(),
                    is_close: false,
                };
                Ok(())
            }
            (State::Init, _) => Err("Expect <"),
            _ => panic!("Invalid State, expect Init"),
        }
    }

    fn handle_tag_name(&mut self, c: char) -> Result {
        match self.state {
            State::TagName { ref mut cache, ref mut is_close } => {
                match (c, cache.len(), &is_close) {
                    // meet 'A'..='Z', tag name cache should less then 9
                    (c @ 'A' ..= 'Z', 0..=8, _) => {
                        cache.push(c);
                        Ok(())
                    }
                    // meet '>', cache length should in 1..=9
                    // if not is_close then into TagContent
                    ('>', 1..=9, false) => {
                        self.stack.push(cache.to_string());
                        self.state = State::TagContent;
                        Ok(())
                    }
                    // if is_close then into TagContent or End
                    ('>', 1..=9, true) => {
                        match self.stack.pop() {
                            Some(s) if s == *cache => {
                                if self.stack.len() > 0 {
                                    // inner level tag closed, into outer TagContent
                                    self.state = State::TagContent;
                                } else {
                                    // top level tag closed, into End
                                    self.state = State::End;
                                }
                                Ok(())
                            }
                            _ => Err("Wrong close tag"),
                        }
                    }
                    // meet '/' only when not is_close and cache is empty
                    ('/', 0, false) => {
                        *is_close = true;
                        Ok(())
                    }
                    // otherwise invalid tag character
                    // ('/', _, true) | ('/', _, false) => Err("Invalid tag char"),
                    // meet '!' only when cache is empty & not !is_close, which means '!' just after '<'
                    ('!', 0, false) if !self.stack.is_empty() => {
                        self.state = State::CDataTag { cache: String::new() };
                        Ok(())
                    }
                    // you can match more different conditions for more error detail
                    _ => Err("Invalid character in TagName State"),
                }
            }
            _ => panic!("Invalid State, expect TagName"),
        }
    }

    fn handle_tag_content(&mut self, c: char) -> Result {
        match (&self.state, c) {
            // if meet '<', then into inner TagName
            (State::TagContent, '<') => {
                self.state = State::TagName {
                    cache: String::new(),
                    is_close: false,
                };
                Ok(())
            }
            // otherwise all characters would be ok
            (State::TagContent, _) => Ok(()),
            _ => panic!("Invalid State, expect TagContent"),
        }
    }

    fn handle_cdata_tag(&mut self, c: char) -> Result {
        match self.state {
            State::CDataTag { ref mut cache } => {
                match (c, cache.len()) {
                    (c, 0..=6) => {
                        cache.push(c);
                        if "[CDATA[" == cache.as_str() {
                            self.state = State::CDataContent { prefix: (false, false) };
                            Ok(())
                        } else if "[CDATA[".starts_with(cache.as_str()) {
                            Ok(())
                        } else {
                            Err("Wrong CDataTag character")
                        }
                    }
                    _ => Err("CDataTag invalid"),
                }
            }
            _ => panic!("Invalid State, expect CDataTag"),
        }
    }

    fn handle_cdata_content(&mut self, c: char) -> Result {
        match self.state {
            State::CDataContent { ref mut prefix } => {
                match (&prefix.0, &prefix.1, c) {
                    (true, true, '>') => {
                        self.state = State::TagContent;
                        Ok(())
                    }
                    (true, true, ']') => Ok(()),
                    (true, true, _) => {
                        // reset prefix
                        *prefix = (false, false);
                        Ok(())
                    }
                    (true, false, ']') => {
                        *prefix = (true, true);
                        Ok(())
                    }
                    (false, false, ']') => {
                        *prefix = (true, false);
                        Ok(())
                    }
                    _ => Ok(()),
                }
            }
            _ => panic!("Invalid State, expect CDataContent"),
        }
    }

    fn handle_end(&mut self, _c: char) -> Result {
        match self.state {
            State::End => Err("End of code"),
            _ => panic!("Invalid State, expect End"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (true, "<DIV>This is the first line <![CDATA[<div>]]></DIV>"),
            (true, "<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>"),
            (true, "<TRUE><![CDATA[wahaha]]]><![CDATA[]> wahaha]]></TRUE>"),

            (false, "<DIV>>>  ![cdata[]] </![CDATA[<div>]>]]>]]>>]</DIV>"),
            (false, "<A>  <B> </A>   </B>"),
            (false, "<DIV>  div tag is not closed  <DIV>"),
            (false, "<DIV>  unmatched <  </DIV>"),
            (false, "<DIV> closed tags with invalid tag name  <b>123</b> </DIV>"),
            (false, "<DIV> unmatched tags with invalid tag name  </1234567890> and <CDATA[[]]>  </DIV>"),
            (false, "<DIV>  unmatched start tag <B>  and unmatched end tag </C>  </DIV>"),
            (false, "<A><![CDATA[</A>]]123></A>"),
        ];
        for (expect, arg) in cases {
            assert_eq!(expect, Solution::is_valid(arg.into()));
        }
    }
}
