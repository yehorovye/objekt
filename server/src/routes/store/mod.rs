pub mod add;
pub mod entry;
pub mod list;
pub mod metadata;
pub mod purge;
pub mod remove;
pub mod update;

macros_utils::routes! {
    load update, // protected
    load add, // protected
    load list,
    load metadata,
    load entry,
    load remove, // protected

    on "/store"
}
