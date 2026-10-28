use crate::runtime::{
    push_query_param, ApiClient, ApiError, HttpMethod, QueryParam, ResponseEnvelope,
};
use serde_json::Value;

/// WebAPI controller `DevelogicPurchases`.
pub struct IDevelogicPurchasesController;

#[allow(non_snake_case)]
impl IDevelogicPurchasesController {
    /// `POST` `/api/DevelogicPurchases/Invoice`
    ///
    /// Parameter `issue`: Czy wystawić dokument zakupu. W przypadku, gdy parametr nie zostanie podany dokument zakupu zostanie dodany do bufora.
    ///
    /// Body `document`: Obiekt dokumentu zakupu do dodania do bufora lub wystawienia.
    pub async fn AddNew(
        api: &ApiClient,
        issue: bool,
        document: &crate::web_api::interface::purchases::view_models::PurchaseDocumentIssue,
    ) -> Result<
        ResponseEnvelope<crate::web_api::interface::purchases::view_models::PurchaseDocument>,
        ApiError,
    > {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "issue", &issue)?;
        api.request_with_body::<crate::web_api::interface::purchases::view_models::PurchaseDocument, _>(HttpMethod::Post, "/api/DevelogicPurchases/Invoice", query, document).await
    }

    /// `POST` `/api/DevelogicPurchases/Invoice`
    ///
    /// Parameter `issue`: Czy wystawić dokument zakupu. W przypadku, gdy parametr nie zostanie podany dokument zakupu zostanie dodany do bufora.
    ///
    /// Body `document`: Obiekt dokumentu zakupu do dodania do bufora lub wystawienia.
    pub async fn AddNewRaw(
        api: &ApiClient,
        issue: bool,
        document: &crate::web_api::interface::purchases::view_models::PurchaseDocumentIssue,
    ) -> Result<ResponseEnvelope<Value>, ApiError> {
        let mut query = Vec::<QueryParam>::new();
        push_query_param(&mut query, "issue", &issue)?;
        api.request_with_body_raw(
            HttpMethod::Post,
            "/api/DevelogicPurchases/Invoice",
            query,
            document,
        )
        .await
    }
}
