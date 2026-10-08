//! The SQLite adapter passes the unit-of-work contract the in-memory fake passes (T035).

mod support;

#[tokio::test(flavor = "current_thread")]
async fn sqlite_passes_the_unit_of_work_contract() {
    let databases = support::Databases::new();
    trcli_testing::contract_uow::run(async || databases.fresh().await).await;
}
