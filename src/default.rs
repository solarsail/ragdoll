use std::path::PathBuf;


pub fn assets_path() -> PathBuf {
    find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap()
}