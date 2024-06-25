pub mod db;
pub mod entity;
mod hash;
pub mod pwd;
pub mod tree;

use std::path::PathBuf;

pub fn get_project_root() -> std::io::Result<PathBuf> {
    if let Some(root) = get_cargo_project_root()? {
        Ok(root)
    } else {
        Ok(std::env::current_dir()?)
    }
}

pub fn get_cargo_project_root() -> std::io::Result<Option<PathBuf>> {
    let current_path = std::env::current_dir()?;
    for ancestor in current_path.ancestors() {
        for dir in std::fs::read_dir(ancestor)? {
            let dir = dir?;
            if dir.file_name() == *"Cargo.lock" {
                return Ok(Some(ancestor.to_path_buf()));
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod test{
    use chrono::{DateTime, FixedOffset, Utc};

    #[test]
    fn test(){
        let now=Utc::now();
        let now:DateTime<FixedOffset>=now.fixed_offset();
        println!("{:#?}",now)
    }
}