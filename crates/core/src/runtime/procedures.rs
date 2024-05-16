use crate::extractors::AppInfo;

/// Runtime procedure
/// This procedure is used to get app name
pub async fn get_app_name(app: AppInfo) -> String {
    app.name.clone()
}

/// Runtime procedure
/// This procedure is used to get app version
pub async fn get_app_version(app: AppInfo) -> String {
    app.version.clone()
}

/// Runtime procedure
/// This procedure is used to get app description
pub async fn get_app_description(app: AppInfo) -> String {
    app.description.clone()
}
