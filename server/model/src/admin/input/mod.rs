pub use sys_access_key::{AccessKeyPageRequest, CreateAccessKeyInput};
pub use sys_authentication::LoginInput;
pub use sys_domain::{CreateDomainInput, DomainPageRequest, UpdateDomainInput};
pub use sys_endpoint::EndpointPageRequest;
pub use sys_login_log::LoginLogPageRequest;
pub use sys_menu::{CreateMenuInput, UpdateMenuInput};
pub use sys_role::{CreateRoleInput, RolePageRequest, UpdateRoleInput};
pub use sys_user::{CreateUserInput, UpdateUserInput, UserPageRequest};

mod sys_access_key;
mod sys_authentication;
mod sys_domain;
mod sys_endpoint;
mod sys_login_log;
mod sys_menu;
mod sys_role;
mod sys_user;
