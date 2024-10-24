use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use server_api::admin::SysDomainApi;
use server_global::global::{add_route, RouteInfo};

pub struct SysDomainRouter;

impl SysDomainRouter {
    pub async fn init_domain_router() -> Router {
        let base_path = "/domain";
        let service_name = "SysDomainApi";

        let routes = vec![
            RouteInfo::new(&format!("{}/list", base_path), Method::GET, service_name),
            RouteInfo::new(base_path, Method::POST, service_name),
            RouteInfo::new(&format!("{}/:id", base_path), Method::GET, service_name),
            RouteInfo::new(base_path, Method::PUT, service_name),
            RouteInfo::new(&format!("{}/:id", base_path), Method::DELETE, service_name),
        ];

        for route in routes {
            add_route(route).await;
        }

        let router = Router::new()
            .route("/list", get(SysDomainApi::get_paginated_domains))
            .route("/", post(SysDomainApi::create_domain))
            .route("/:id", get(SysDomainApi::get_domain))
            .route("/", put(SysDomainApi::update_domain))
            .route("/:id", delete(SysDomainApi::delete_domain));

        Router::new().nest(base_path, router)
    }
}
