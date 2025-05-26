pub mod add;
pub mod entry;
pub mod list;
pub mod metadata;
pub mod remove;
pub mod upsert;

macros_utils::routes! {
    load upsert, // protected
    load add, // protected
    load list,
    load metadata,
    load entry,
    load remove, // protected

    on "/store"
}
