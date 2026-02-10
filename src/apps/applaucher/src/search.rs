// Filtering/search logic for applications

pub fn filter_apps<'a>(apps: &'a [crate::desktop_parser::DesktopApp], query: &str) -> Vec<&'a crate::desktop_parser::DesktopApp> {
    let query = query.to_lowercase();
    apps.iter()
        .filter(|app| app.name.to_lowercase().contains(&query))
        .collect()
}