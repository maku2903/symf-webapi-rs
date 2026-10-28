use std::env;
use std::time::Duration;

use symfwebapi::web_api::interface::bank_accounts::interfaces::i_bank_accounts_controller::IBankAccountsController;
use symfwebapi::web_api::interface::enums::enumOrderByType;
use symfwebapi::web_api::interface::products::interfaces::i_products_controller::IProductsController;
use symfwebapi::{ApiClient, ApiError, ClientConfig};

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("example failed: {err}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), ApiError> {
    let Some(base_url) = env::var("SYMFWEBAPI_BASE_URL").ok() else {
        print_usage();
        return Ok(());
    };

    let mut builder = ClientConfig::builder(base_url)
        .nul_sanitization(true)
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10));
    if let Ok(application_key) = env::var("SYMFWEBAPI_APPLICATION_KEY") {
        let device_name = env::var("SYMFWEBAPI_DEVICE_NAME")
            .unwrap_or_else(|_| "symfwebapi-rust-example".to_owned());
        builder = builder.session(application_key, device_name);
    }
    let client = ApiClient::new(builder.build()?)?;
    let products =
        IProductsController::GetPagedDocument(&client, 1, 10, enumOrderByType::Asc).await?;

    println!(
        "loaded {} products from page {} / total {}",
        products.model.Data.len(),
        products.model.PageNumber,
        products.model.TotalItems,
    );

    for product in products.model.Data.iter().take(3) {
        let code = product.Code.as_str();
        let name = product.Name.as_str();
        let id = product.Id.to_string();
        println!("  {} | {} | {}", id, code, name);
    }

    if let Some(bank_account_id) = env::var("SYMFWEBAPI_BANK_ACCOUNT_ID")
        .ok()
        .and_then(|value| value.parse::<i32>().ok())
    {
        let bank_account = IBankAccountsController::Get(&client, bank_account_id).await?;
        println!(
            "bank account {}: {} ({})",
            bank_account.model.Id,
            bank_account
                .model
                .AccountNumber
                .as_deref()
                .unwrap_or("<missing>"),
            bank_account
                .model
                .BankName
                .as_deref()
                .unwrap_or("<missing>"),
        );
    } else {
        println!("set SYMFWEBAPI_BANK_ACCOUNT_ID to fetch a single bank account");
    }

    client.close_session().await?;
    Ok(())
}

fn print_usage() {
    eprintln!("set SYMFWEBAPI_BASE_URL to run this example");
    eprintln!("optional: SYMFWEBAPI_APPLICATION_KEY for session-based auth");
    eprintln!("optional: SYMFWEBAPI_DEVICE_NAME to override session device name");
    eprintln!("optional: SYMFWEBAPI_BANK_ACCOUNT_ID to fetch one bank account");
}
