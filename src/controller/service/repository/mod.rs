pub mod character_repository;

pub const DB_ROOT_PATH: &str = "./databases/";

pub fn get_database_path(repo_path: &str) -> String {
    return format!("{}{}", DB_ROOT_PATH, repo_path);
}

pub fn id_generator(repo_path: &str) -> String {
    let db: sled::Db = sled::open(get_database_path(repo_path)).unwrap();
    let id: String = db.generate_id().unwrap().to_string();
    return id;
}
