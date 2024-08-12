use serde::{Deserialize, Serialize};
use server_core::web::page::PageRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainPageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    pub keywords: Option<String>,
}
