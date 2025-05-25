pub mod auth;
pub mod root;
pub mod store;

macros_utils::routes! {
    load root,
    load auth,
    load store
}
