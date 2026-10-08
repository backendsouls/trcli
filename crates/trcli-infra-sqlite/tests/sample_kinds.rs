//! The sample kinds' own rows round-trip through SQLite, and their use cases work against
//! the real adapter exactly as against the fake (feature `sample-kind`).

mod support;

use trcli_application::ports::interaction::Confirmation;
use trcli_application::ports::unit_of_work::{Storage, UnitOfWork};
use trcli_application::records::delete::delete;
use trcli_application::records::show::show;
use trcli_application::sample::sample_note::{self, AddSampleNote, EditSampleNote, SampleNotes};
use trcli_application::sample::specimen::{self, AddSpecimen, EditSpecimen, Specimens};
use trcli_application::testing::environment::{SeededIds, stamp};
use trcli_application::testing::interaction::ScriptedPrompter;

#[tokio::test(flavor = "current_thread")]
async fn a_specimen_is_added_edited_shown_and_deleted() {
    let databases = support::Databases::new();
    let storage = databases.fresh().await;
    let mut unit = storage.begin().await.expect("begin");
    let command = AddSpecimen::new(Some("First")).expect("valid").command;
    let added = specimen::add(&mut unit, &stamp(), &SeededIds::new(), command)
        .await
        .expect("added")
        .command;
    let command = EditSpecimen::new(&added.handle, Some("Renamed"))
        .expect("valid")
        .command;
    specimen::edit(&mut unit, &stamp(), command)
        .await
        .expect("edited");
    unit.commit().await.expect("commit");

    let mut unit = storage.begin().await.expect("begin");
    let detail = show(&unit, &Specimens::new(), &added.handle)
        .await
        .expect("shown");
    assert_eq!(
        (detail.name.as_str(), detail.fields[0].value.as_deref()),
        ("Renamed", Some("Renamed"))
    );
    let mut prompter = ScriptedPrompter::answering(Confirmation::Yes);
    delete(
        &mut unit,
        &stamp(),
        &Specimens::new(),
        &mut prompter,
        &added.handle,
    )
    .await
    .expect("deleted");
    assert!(show(&unit, &Specimens::new(), &added.handle).await.is_err());
}

#[tokio::test(flavor = "current_thread")]
async fn a_locked_sample_note_is_kept_until_unlocked() {
    let databases = support::Databases::new();
    let mut unit = databases.fresh().await.begin().await.expect("begin");
    let command = AddSampleNote::new(Some("Keep me"), true)
        .expect("valid")
        .command;
    let added = sample_note::add(&mut unit, &stamp(), &SeededIds::new(), command)
        .await
        .expect("added");
    let mut prompter = ScriptedPrompter::answering(Confirmation::Yes);
    let blocked = delete(
        &mut unit,
        &stamp(),
        &SampleNotes::new(),
        &mut prompter,
        &added.handle,
    )
    .await;
    assert_eq!(
        blocked.expect_err("locked").code.name,
        "blocked_by_dependents"
    );

    let unlock = EditSampleNote::new(&added.handle, None, Some(false))
        .expect("valid")
        .command;
    sample_note::edit(&mut unit, &stamp(), unlock)
        .await
        .expect("unlocked");
    delete(
        &mut unit,
        &stamp(),
        &SampleNotes::new(),
        &mut prompter,
        &added.handle,
    )
    .await
    .expect("deleted");
}
