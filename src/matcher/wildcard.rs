// src/macher/wildcard.rs
// 定义了一个名为WildcardMatcher的结构体，实现了通配符匹配的功能。

#[derive(Debug, Clone)]
pub struct WildcardMatcher {
    chars: Vec<char>,   // 用于存储通配符模式的字符数组
}

impl WildcardMatcher {
    pub fn new(s: &str) -> Self {
        Self {
            chars: s.chars().collect::<Vec<char>>(),
        }
    }

    /// 判断给定的字符串s是否与通配符模式匹配
    pub fn is_match(&self, s: &str) -> bool {
        // 可变的字符迭代器，用于遍历s字符串的字符
        let mut chars = s.chars();
        // 创建一个布尔变量dot，用于标记是否遇到了通配符模式中的.字符
        let mut dot = false;

        for ch in &self.chars {
            match ch {
                '*' => {
                    match chars.next() {
                        Some(c) => {
                            if c == '.' {
                                return false;
                            }
                        }
                        None => return false,
                    }
                    while let Some(n) = chars.next() {
                        if n == '.' {
                            dot = true;
                            break;
                        }
                    }
                }
                word => {
                    if dot {
                        if word == &'.' {
                            dot = false;
                            continue;
                        } else {
                            return false;
                        }
                    }
                    match chars.next() {
                        Some(ch) => {
                            if word != &ch {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
            }
        }

        if dot {
            return false;
        }

        chars.next().is_none()
    }
}
