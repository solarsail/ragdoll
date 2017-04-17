use std::path::PathBuf;
use find_folder;


pub fn assets_path() -> PathBuf {
    find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap()
}