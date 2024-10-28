use strum_macros::{AsRefStr, Display, EnumString};

/// Token 状态枚举
#[derive(Debug, Clone, PartialEq, Eq, AsRefStr, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenStatus {
    /// 活跃状态，可以正常使用
    Active,
    /// 已被刷新，表示该 token 已被新 token 替换
    Refreshed,
    /// 已被撤销（手动注销或安全原因）
    Revoked,
}

impl TokenStatus {
    pub fn is_valid(&self) -> bool {
        matches!(self, TokenStatus::Active)
    }

    pub fn can_refresh(&self) -> bool {
        matches!(self, TokenStatus::Active)
    }
}
