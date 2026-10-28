//! Rust client for Symfonia WebAPI.
//!
//! Publiczny shape biblioteki jest utrzymywany możliwie blisko oryginalnego
//! `WebAPI.Interface.*` Symfonii:
//!
//! - async path przez [`web_api::interface`],
//! - kontrolery i modele odzwierciedlają namespace'y WebAPI,
//! - overloady są modelowane przez typed selektory i jedną kanoniczną metodę.
//!
//! Przykład:
//!
//! ```no_run
//! use symfwebapi::web_api::interface::enums::enumOrderByType;
//! use symfwebapi::web_api::interface::products::interfaces::i_products_controller::IProductsController;
//! use symfwebapi::{ApiClient, ClientConfig};
//!
//! # async fn demo() -> Result<(), symfwebapi::ApiError> {
//! let config = ClientConfig::builder("https://host:9000")
//!     .session("app-guid", "erp-sync")
//!     .build()?;
//! let client = ApiClient::new(config)?;
//! let page = IProductsController::GetPagedDocument(&client, 1, 10, enumOrderByType::Asc).await?;
//! println!("items: {}", page.model.TotalItems);
//! # Ok(())
//! # }
//! ```
pub mod runtime;
pub mod web_api;

pub use runtime::{
    ApiClient, ApiError, ClientConfig, ClientConfigBuilder, ClientTimeouts, RequestContext,
    ResponseEnvelope,
};
