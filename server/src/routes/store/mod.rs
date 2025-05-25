pub mod add;
pub mod entry;
pub mod metadata;

macros_utils::routes! {
    load add,
    load metadata,
    load entry,

    on "/store"
}
