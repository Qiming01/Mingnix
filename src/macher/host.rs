// src/macher/host.rs
// 定义了一个名为HostMatcher的结构体，用于匹配HTTP头部中的Host字段

use super::WildcardMatcher;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct HostMatcher {
    modes: Vec<MatchMode>,
}

#[derive(Debug, Clone)]
enum MatchMode {
    Text(String),
    Wildcard(WildcardMatcher),
}

impl Default for HostMatcher {
    fn default() -> HostMatcher {
        HostMatcher {
            modes: Vec::default(),
        }
    }
}

impl HostMatcher {
    // 创建HostMatcher实例
    pub fn new(items: Vec<&str>) -> Self {
        Self {
            modes: items
                .into_iter()
                .collect::<BTreeSet<&str>>()
                .into_iter()
                .map(|item| {
                    if item.contains('*') {
                        // Use wildcard match: *.example.com
                        MatchMode::Wildcard(WildcardMatcher::new(item))
                    } else {
                        // Plain Text: example.com
                        MatchMode::Text(item.to_string())
                    }
                })
                .collect::<Vec<MatchMode>>(),
        }
    }

    // 获取原始的匹配模式
    pub fn get_raw(&self) -> Vec<&String> {
        let mut v = vec![];
        for item in &self.modes {
            match item {
                MatchMode::Text(s) => {
                    v.push(s);
                }
                _ => todo!(),
            }
        }
        v
    }

    pub fn is_empty(&self) -> bool {
        self.modes.is_empty()
    }

    // 判断给定的主机是否与任何匹配模式匹配
    pub fn is_match(&self, host: &str) -> bool {
        // 匹配模式为空
        if self.is_empty() {
            return true;
        }

        for matcher in &self.modes {
            match matcher {
                MatchMode::Text(text) => {
                    if text == host {
                        return true;
                    }
                }
                MatchMode::Wildcard(wildcard) => {
                    if wildcard.is_match(host) {
                        return true;
                    }
                }
            }
        }

        false
    }
}