use async_trait::async_trait;
use bcrypt::verify;
use sea_orm::{ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait};
use server_constant::definition::Audience;
use server_core::web::{
    auth::Claims,
    error::AppError,
    jwt::{JwtError, JwtUtils},
};
use server_model::admin::{
    entities::{
        prelude::{SysRole, SysUser},
        sys_domain, sys_organization, sys_role, sys_user, sys_user_role,
    },
    input::LoginInput,
    output::{AuthOutput, UserWithDomainAndOrgOutput},
};
use tokio::{spawn, sync::mpsc};

use crate::{admin::sys_user_error::UserError, helper::db_helper};

macro_rules! select_user_with_domain_and_org_info {
    ($query:expr) => {{
        $query
            .select_only()
            .column_as(sys_user::Column::Id, "id")
            .column_as(sys_user::Column::DomainId, "domain_id")
            .column_as(sys_user::Column::OrgId, "org_id")
            .column_as(sys_user::Column::Username, "username")
            .column_as(sys_user::Column::Password, "password")
            .column_as(sys_user::Column::NickName, "nick_name")
            .column_as(sys_user::Column::Avatar, "avatar")
            .column_as(sys_domain::Column::Code, "domain_code")
            .column_as(sys_domain::Column::Name, "domain_name")
            .column_as(sys_organization::Column::Id, "organization_id")
            .column_as(sys_organization::Column::Name, "organization_name")
    }};
}
#[async_trait]
pub trait TAuthService {
    async fn pwd_login(&self, input: LoginInput, domain: &str) -> Result<AuthOutput, AppError>;
}

#[derive(Clone)]
pub struct SysAuthService;

#[async_trait]
impl TAuthService for SysAuthService {
    async fn pwd_login(&self, input: LoginInput, domain: &str) -> Result<AuthOutput, AppError> {
        let db = db_helper::get_db_connection().await?;

        let user = select_user_with_domain_and_org_info!(SysUser::find())
            .filter(sys_user::Column::Username.eq(&input.username))
            .filter(sys_domain::Column::Code.eq(domain))
            .join(JoinType::InnerJoin, sys_user::Relation::SysDomain.def())
            .join(JoinType::LeftJoin, sys_user::Relation::SysOrganization.def())
            .into_model::<UserWithDomainAndOrgOutput>()
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::from(UserError::UserNotFound))?;

        if !verify(&input.password, &user.password)
            .map_err(|_| AppError::from(UserError::AuthenticationFailed))?
        {
            return Err(AppError::from(UserError::WrongPassword));
        }

        let role_codes: Vec<String> = SysRole::find()
            .join(JoinType::InnerJoin, sys_role::Relation::SysUserRole.def())
            .join(JoinType::InnerJoin, sys_user_role::Relation::SysUser.def())
            .filter(sys_user::Column::Id.eq(user.id))
            .select_only()
            .column(sys_role::Column::Code)
            .all(db.as_ref())
            .await
            .map_err(AppError::from)?
            .iter()
            .filter_map(|role| Some(role.code.clone()))
            .collect();

        let auth_output = generate_auth_output(
            user.id,
            user.username,
            role_codes,
            user.domain_code,
            user.organization_name,
        )
        .await
        .map_err(AppError::from)?;

        Ok(auth_output)
    }
}

pub async fn generate_auth_output(
    user_id: i64,
    username: String,
    role_codes: Vec<String>,
    domain_code: String,
    organization_name: Option<String>,
) -> Result<AuthOutput, JwtError> {
    let claims = Claims::new(
        user_id.to_string(),
        Audience::ManagementPlatform.as_str().to_string(),
        username,
        role_codes,
        domain_code,
        organization_name,
    );

    let token = JwtUtils::generate_token(&claims).await?;

    Ok(AuthOutput {
        access_token: token,
    })
}

pub async fn start_event_listener(mut rx: mpsc::UnboundedReceiver<String>) {
    spawn(async move {
        while let Some(jwt) = rx.recv().await {
            // TODO Consider storing the token into the database?
            println!("JWT created: {}", jwt);
        }
    });
}
