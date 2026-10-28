# symfwebapi

Biblioteka kliencka dla Symfonia WebAPI w Rust.

Projekt stawia na publiczny kontrakt możliwie bliski oryginalnemu WebAPI:
kontrolery, modele i enumy zachowują układ `WebAPI.Interface.*`, a overloady są
odwzorowane przez selektory i jedną kanoniczną metodę operacji.

## Cele

- wierne odwzorowanie kontrolerów, modeli i enumów Symfonia WebAPI;
- w pełni typowany klient async oparty o `reqwest` i `serde`;
- bezpieczne zarządzanie sesją z automatycznym odnowieniem po `401`;
- możliwość pracy zarówno na modelach, jak i na surowym JSON;
- dokumentacja API dostępna bezpośrednio w `rustdoc`.

## Moduły

1. `symfwebapi::runtime` - konfiguracja klienta, transport HTTP, sesja i błędy.
2. `symfwebapi::web_api` - kontrolery, modele i enumy odwzorowujące WebAPI.
3. `symfwebapi` - reexport najważniejszych typów runtime'u.

## Status

Biblioteka udostępnia:

- `ClientConfig` z builderem, timeoutami i obsługą sesji;
- `ApiClient`;
- typed i raw response;
- automatyczne zarządzanie sesją i pojedynczy retry po `401`;
- `ApiError` z `RequestContext`;
- forward-compatible enumy;
- dokumentację API dostępną w `rustdoc`.

Wersjonowanie crate'a używa konwencji `MAJOR.MINOR.WWMM`, gdzie:

- `MAJOR` oznacza breaking change w SDK,
- `MINOR` oznacza kolejną iterację SDK dla tej samej wersji WebAPI,
- `WWMM` koduje docelową wersję Symfonia WebAPI, np. `26.20 -> 2620`.

W serii `0.x` kolejna iteracja może zawierać zmiany niezgodne źródłowo.
Wydanie `0.2.2620` wymaga migracji opisanej poniżej i jawnej aktualizacji
zależności korzystających z `0.1`.

## Szybki start

```rust
use symfwebapi::web_api::interface::enums::enumOrderByType;
use symfwebapi::web_api::interface::products::interfaces::i_products_controller::IProductsController;
use symfwebapi::{ApiClient, ClientConfig};

# async fn demo() -> Result<(), symfwebapi::ApiError> {
let config = ClientConfig::builder("https://host:9000")
    .session("app-guid", "erp-sync")
    .timeout(std::time::Duration::from_secs(30))
    .connect_timeout(std::time::Duration::from_secs(10))
    .build()?;

let client = ApiClient::new(config)?;

let page = IProductsController::GetPagedDocument(&client, 1, 10, enumOrderByType::Asc).await?;
println!("Liczba elementów: {}", page.model.TotalItems);
# Ok(())
# }
```

## Overloady

Overloady są odwzorowane przez selektory, dzięki czemu można zachować jedną
nazwę operacji i jednocześnie uzależnić typ odpowiedzi od wejścia.

```rust
use symfwebapi::web_api::interface::products::interfaces::i_products_controller::{
    get, IProductsController,
};

# async fn demo(api: &symfwebapi::ApiClient) -> Result<(), symfwebapi::ApiError> {
let by_id = IProductsController::Get(api, get::ById { id: 1 }).await?;
let by_code = IProductsController::Get(
    api,
    get::ByCode {
        code: "ABC".to_owned(),
    },
)
.await?;
let list = IProductsController::Get(api, get::List).await?;

println!("{}", by_id.model.Code);
println!("{}", by_code.model.Code);
println!("{}", list.model.len());
# Ok(())
# }
```

## Raw JSON / dane nieidealne

Jeśli potrzebujesz zejść do surowego payloadu JSON, możesz używać metod `*Raw`
na tych samych kontrolerach:

```rust
use symfwebapi::web_api::interface::enums::enumOrderByType;
use symfwebapi::web_api::interface::products::interfaces::i_products_controller::IProductsController;

# async fn demo(api: &symfwebapi::ApiClient) -> Result<(), symfwebapi::ApiError> {
let page = IProductsController::GetPagedDocumentRaw(api, 1, 10, enumOrderByType::Asc).await?;
let total_items = page.model["TotalItems"].as_i64().unwrap_or_default();
println!("TotalItems = {}", total_items);
# Ok(())
# }
```

Dla overloadów ten sam mechanizm działa przez `GetRaw(selector)`:

```rust
use symfwebapi::web_api::interface::products::interfaces::i_products_controller::{
    get, IProductsController,
};

# async fn demo(api: &symfwebapi::ApiClient) -> Result<(), symfwebapi::ApiError> {
let product = IProductsController::GetRaw(
    api,
    get::ByCode {
        code: "ABC".to_owned(),
    },
)
.await?;

println!("{}", product.model["Code"]);
# Ok(())
# }
```

## Zmiany zgodności w 0.2.2620

- `WarehouseDocumentPosition.Reservations` i `Deliveries` maja teraz typ
  `Option<Vec<T>>`. `None` oznacza brak kolekcji w odpowiedzi, a `Some(vec![])`
  jawnie pusta kolekcje. Kod korzystajacy z tych pol wymaga dostosowania;
  nie nalezy automatycznie utozsamiac braku danych z pustym wynikiem.
- Domyslne `Debug` obiektow runtime oraz `Display`/`Debug` bledow nie wypisuja
  kluczy, tokenow, payloadow ani szczegolowych komunikatow. Pola `body` i
  `message` w `ApiError` oraz surowe dane `ResponseEnvelope` pozostaja dostepne
  jawnie do diagnostyki. Ich bezposrednie logowanie, podobnie jak logowanie
  modelu, moze ujawniac dane odpowiedzi.
- Opoznione `401` nie uniewaznia juz nowszej sesji innego zadania. Pozostaje
  limit jednej ponownej proby po `401`; nie dodano retry po timeoutach ani `5xx`.

## Walidacja lokalna

```bash
cargo fmt --check
cargo check
cargo test
cargo doc --no-deps
cargo publish --dry-run
```

## Licencja

Biblioteka jest udostępniana na licencji `BSD-3-Clause`. Szczegóły są w pliku
`LICENSE`.
