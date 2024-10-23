pub mod sub_module_helpers {
    pub fn get_full_name(first_name: &str, last_name: &str) -> String {
        format!("{} {}", first_name, last_name)
    }
}
