//2024年3月30日
use actix_web::{rt::System, *};
use chrono::NaiveDate;
use log::{info, warn};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseBackend,
    DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryTrait, TransactionTrait,};
//controller层 未加进来
use crate::{controller::SystemError, model::entities::click};

//init
#[derive(Debug)]
pub struct ClickDao;

impl ClickDao {
    /// # CRUD
    /// ##create
    /// 创建一个新的点击记录。
    ///
    /// 这个函数在数据库中创建一个新的点击记录，记录的level的ID和日期由参数提供。
    /// 创建的点击记录的 `count_today` 字段被设置为1。
    ///
    /// 如果 `commit` 参数为 `true`，这个函数将提交事务。
    /// 如果 `commit` 参数为 `false`，事务将不会被提交，这允许你在稍后提交事务，或者添加更多的操作到同一个事务中。
    ///
    /// ### 参数
    ///
    /// * `db` - 数据库连接。
    /// * `date` - 新的点击记录的日期。
    /// * `level_id` - 新的点击记录的心情ID。
    /// * `attemp_times` - 目前的尝试次数
    /// * `pass_times` - 目前的通过次数
    /// * `commit` - 是否应该在这个函数中提交事务。
    ///
    /// 使用Option作为传入参数可以应付NULL问题
    /// ### 返回值
    ///
    /// * `Ok(())` - 成功时返回 `()`。
    /// * `Err(SystemError)` - 如果在创建新的点击记录或提交事务时发生错误，返回一个 `SystemError`。
    pub async fn create(
        db: DatabaseConnection,
        level_id: Option<i32>,
        date: Option<String>,
        attemp_times: Option<i32>,
        pass_times: Option<i32>,
    ) -> Result<(), SystemError> {
        //

        return Ok(());
    }

    /// 创建一个新的点击记录,但是不提交事务。
    async fn do_create(
        tx: &mut sea_orm::DatabaseTransaction,
        date: Option<String>,
        level_id: Option<i32>,
        attempt_times: Option<i32>,
        pass_times: Option<i32>,
    ) -> Result<(), SystemError> {
        let click = click::ActiveModel {
            date: ActiveValue::set(date),
            level_id: ActiveValue::set(level_id),
            attempt_times:ActiveValue::set(attempt_times),
            pass_times: ActiveValue::set(pass_times),
            ..Default::default()
        };
        click.save(tx).await.map_err(SystemError::DbErr)?;

        return Ok(());
    }
}
