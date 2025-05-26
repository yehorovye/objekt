pub mod add;
pub mod entry;
pub mod list;
pub mod metadata;
pub mod remove;
pub mod upsert;

macros_utils::routes! {
    load upsert,
    load add,
    load list,
    load metadata,
    load entry,
    load remove,

    on "/store"
}
