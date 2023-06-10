// src/macher/location.rs
// 定义了一个名为LocationMatcher的结构体，用于匹配路径或位置

use crate::util;
use globset::GlobMatcher;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct LocationMatcher(MatchMode);

// 匹配模式
#[derive(Debug, Clone)]
enum MatchMode {
    Glob(GlobMatcher),
    Regex(Regex),
    Start(String),
    End(String),
}

impl LocationMatcher {
    // 使用glob表达式进行匹配
    pub fn glob(location: &str) -> Result<Self, String> {
        Ok(LocationMatcher(MatchMode::Glob(util::to_glob(location)?)))
    }

    // 使用正则表达式进行匹配
    pub fn regex(location: &str) -> Result<Self, String> {
        Ok(LocationMatcher(MatchMode::Regex(util::to_regex(location)?)))
    }

    // 匹配位置的开头
    pub fn start(location: &str) -> Self {
        LocationMatcher(MatchMode::Start(location.to_string()))
    }

    // 匹配位置的结尾
    pub fn end(location: &str) -> Self {
        LocationMatcher(MatchMode::End(location.to_string()))
    }

    // 判断给定的路径或位置是否与匹配模式匹配
    pub fn is_match(&self, path: &str) -> bool {
        match &self.0 {
            MatchMode::Glob(glob) => glob.is_match(path),
            MatchMode::Regex(reg) => reg.is_match(path),
            MatchMode::Start(s) => path.starts_with(s),
            MatchMode::End(s) => path.ends_with(s),
        }
    }
}