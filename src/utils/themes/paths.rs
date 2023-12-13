pub fn get_default_theme_path() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.push("themes");
    path.push("__DEFAULT__");
    path.to_str().unwrap().to_string()
}
