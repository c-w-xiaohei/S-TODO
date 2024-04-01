#[derive(Debug)]
pub enum SystemError {
    DbErr(sea_orm::DbErr),
    /// 请在系统内部重试
    InnerRetry,
    Busy(String),
    /// AI错误
    AIError(String),
}