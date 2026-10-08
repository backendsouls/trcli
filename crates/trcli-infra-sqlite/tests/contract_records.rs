//! The SQLite adapter passes the records contract the in-memory fake passes (T071).

mod support;

#[tokio::test(flavor = "current_thread")]
async fn sqlite_passes_the_records_contract() {
    let databases = support::Databases::new();
    trcli_application::testing::contract_records::run(async || databases.fresh().await).await;
}
