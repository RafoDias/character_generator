pub mod character_repository;

pub const DB_ROOT_PATH: &str = "./databases/";

pub fn get_database_path(repo_path: &str) -> String {
    return format!("{}{}", DB_ROOT_PATH, repo_path);
}
