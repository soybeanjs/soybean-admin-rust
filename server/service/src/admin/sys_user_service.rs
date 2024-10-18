use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter, Set,
};
use server_core::web::{error::AppError, page::PaginatedData};
use server_model::admin::{
    entities::{prelude::SysUser, sys_user},
    input::{CreateUserInput, UpdateUserInput, UserPageRequest},
    output::UserWithoutPassword,
};

use super::sys_user_error::UserError;
use crate::helper::db_helper;

#[async_trait]
pub trait TUserService {
    async fn find_all(&self) -> Result<Vec<UserWithoutPassword>, AppError>;
    async fn find_paginated_users(
        &self,
        params: UserPageRequest,
    ) -> Result<PaginatedData<UserWithoutPassword>, AppError>;

    async fn create_user(&self, input: CreateUserInput) -> Result<UserWithoutPassword, AppError>;
    async fn get_user(&self, id: i64) -> Result<UserWithoutPassword, AppError>;
    async fn update_user(&self, input: UpdateUserInput) -> Result<UserWithoutPassword, AppError>;
    async fn delete_user(&self, id: i64) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct SysUserService;

impl SysUserService {
    async fn check_username_unique(&self, username: &str) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;
        let existing_user = SysUser::find()
            .filter(sys_user::Column::Username.eq(username))
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?;

        if existing_user.is_some() {
            return Err(UserError::UsernameAlreadyExists.into());
        }
        Ok(())
    }

    async fn get_user_by_id(&self, id: i64) -> Result<sys_user::Model, AppError> {
        let db = db_helper::get_db_connection().await?;
        SysUser::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| UserError::UserNotFound.into())
    }
}

#[async_trait]
impl TUserService for SysUserService {
    async fn find_all(&self) -> Result<Vec<UserWithoutPassword>, AppError> {
        let db = db_helper::get_db_connection().await?;
        SysUser::find()
            .all(db.as_ref())
            .await
            .map(|users| users.into_iter().map(UserWithoutPassword::from).collect())
            .map_err(AppError::from)
    }

    async fn find_paginated_users(
        &self,
        params: UserPageRequest,
    ) -> Result<PaginatedData<UserWithoutPassword>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = SysUser::find();

        if let Some(ref keywords) = params.keywords {
            let condition = Condition::any().add(sys_user::Column::Username.contains(keywords));
            query = query.filter(condition);
        }

        let total = query.clone().count(db.as_ref()).await.map_err(AppError::from)?;

        let paginator = query.paginate(db.as_ref(), params.page_details.size);
        let records = paginator
            .fetch_page(params.page_details.current - 1)
            .await
            .map_err(AppError::from)?
            .into_iter()
            .map(UserWithoutPassword::from)
            .collect();

        Ok(PaginatedData {
            current: params.page_details.current,
            size: params.page_details.size,
            total,
            records,
        })
    }

    async fn create_user(&self, input: CreateUserInput) -> Result<UserWithoutPassword, AppError> {
        self.check_username_unique(&input.username).await?;

        let db = db_helper::get_db_connection().await?;
        let user = sys_user::ActiveModel {
            domain_id: Set(input.domain_id),
            org_id: Set(input.org_id),
            username: Set(input.username),
            password: Set(input.password), /* Note: In a real application, you should hash the
                                            * password */
            nick_name: Set(input.nick_name),
            avatar: Set(input.avatar),
            email: Set(input.email),
            phone: Set(input.phone),
            status: Set(input.status),
            ..Default::default()
        };

        let user_model = user.insert(db.as_ref()).await.map_err(AppError::from)?;
        Ok(UserWithoutPassword::from(user_model))
    }

    async fn get_user(&self, id: i64) -> Result<UserWithoutPassword, AppError> {
        let db = db_helper::get_db_connection().await?;
        SysUser::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .map(UserWithoutPassword::from)
            .ok_or_else(|| UserError::UserNotFound.into())
    }

    async fn update_user(&self, input: UpdateUserInput) -> Result<UserWithoutPassword, AppError> {
        let mut user = self.get_user_by_id(input.id).await?.into_active_model();

        // Check if username has changed and if so, check for uniqueness
        if input.user.username != *user.username.as_ref() {
            self.check_username_unique(&input.user.username).await?;
        }

        // Update fields
        user.domain_id = Set(input.user.domain_id);
        user.org_id = Set(input.user.org_id);
        user.username = Set(input.user.username);
        user.password = Set(input.user.password); // Note: In a real application, you should hash the password
        user.nick_name = Set(input.user.nick_name);
        user.avatar = Set(input.user.avatar);
        user.email = Set(input.user.email);
        user.phone = Set(input.user.phone);
        user.status = Set(input.user.status);

        let db = db_helper::get_db_connection().await?;
        let updated_user = user.update(db.as_ref()).await.map_err(AppError::from)?;
        Ok(UserWithoutPassword::from(updated_user))
    }

    async fn delete_user(&self, id: i64) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;

        let result = SysUser::delete_by_id(id).exec(db.as_ref()).await.map_err(AppError::from)?;

        if result.rows_affected == 0 {
            return Err(UserError::UserNotFound.into());
        }

        Ok(())
    }
}
