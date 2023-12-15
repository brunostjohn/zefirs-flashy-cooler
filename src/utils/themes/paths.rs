use std::path::PathBuf;

const DEFAULT_HTML: &str = include_str!("../../../resources/default.html");
const DEFAULT_CAT_IMG: &[u8] = include_bytes!("../../../resources/cat.png");

pub fn get_default_theme_path() -> String {
    let mut path = std::env::var("USERPROFILE")
        .map(|val| {
            let mut temp = PathBuf::from(val);
            temp.push("Documents");
            temp.push("Zefir's Flashy Cooler");

            temp
        })
        .unwrap_or({
            let mut temp = std::env::current_exe().unwrap();
            temp.pop();

            temp
        });
    path.push("Themes");
    path.push("__DEFAULT__");
    let _ = std::fs::create_dir_all(&path);
    let _ = std::fs::write(path.join("index.html"), DEFAULT_HTML);
    let _ = std::fs::write(path.join("cat.png"), DEFAULT_CAT_IMG);
    path.to_str().unwrap().to_string()
}

pub fn get_all_themes_path() -> PathBuf {
    let mut path = std::env::var("USERPROFILE")
        .map(|val| {
            let mut temp = PathBuf::from(val);
            temp.push("Documents");
            temp.push("Zefir's Flashy Cooler");

            temp
        })
        .unwrap_or({
            let mut temp = std::env::current_exe().unwrap();
            temp.pop();

            temp
        });
    path.push("Themes");

    path
}
