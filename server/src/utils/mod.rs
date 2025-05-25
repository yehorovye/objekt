// todo: get rid of ts
pub fn sanitize_path_keys(keys: String) -> String {
    keys.replace("/", ":")
}
