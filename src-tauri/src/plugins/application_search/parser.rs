use crate::core::db::search_database_store::SearchDatabaseItem;
use crate::core::utils::path::get_error_icon_path;
use crate::core::utils::result::CommandResult;
use configparser::ini::Ini;
use exe::{VecPE, PE};
use lnk::ShellLink;
use log::trace;
use std::{any, fs};
use std::{
    error::Error,
    path::{Path, PathBuf},
};

/// .lnkファイルの情報を読み取る。
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use std::env;
///
/// let lnk_file_name = "Zoom";
/// let lnk_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources/fake_data").join(lnk_file_name.clone());
///
/// let result = parse_lnk(&lnk_file_path).unwrap();
/// assert_eq!(lnk_file_name, result.name);
/// ```
pub fn parse_lnk(file_path: &PathBuf, debug: bool) -> Result<SearchDatabaseItem, lnk::Error> {
    let lnk_file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
    let lnk_file_path = file_path.to_str().unwrap().to_string();
    let lnk_file = ShellLink::open(&file_path)?;
    if debug {
        dbg!(lnk_file.icon_location());
    }
    let lnk_file_icon_path = lnk_file
        .icon_location()
        .clone()
        .unwrap_or((get_error_icon_path().to_str().unwrap().to_string()));
    trace!("lnk_file_path: {}", &lnk_file_path);
    trace!("lnk_file_icon_path: {}", &lnk_file_icon_path);
    Ok(SearchDatabaseItem::new_app(
        lnk_file_name,
        lnk_file_icon_path,
        lnk_file_path,
    ))
}

/// .urlファイルの情報を読み取る。
pub fn parse_url(file_path: &PathBuf, debug: bool) -> Result<SearchDatabaseItem, Box<dyn Error>> {
    let mut config = Ini::new();
    let map = config.load(file_path)?;
    let file_icon_path = config
        .get("InternetShortcut", "IconFile")
        .unwrap_or((get_error_icon_path().to_str().unwrap().to_string()));
    let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
    if debug {
        dbg!(&file_icon_path, &file_name, &file_path);
    }
    trace!("url_file_path: {}", &file_path.to_str().unwrap());
    trace!("url_file_icon_path: {}", &file_icon_path);
    Ok(SearchDatabaseItem::new_app(
        file_name,
        file_icon_path,
        file_path.to_str().unwrap().to_string(),
    ))
}

pub fn parse_exe(file_path: &PathBuf) -> Result<SearchDatabaseItem, Box<dyn Error>> {
    let pe = VecPE::from_disk_file(file_path)?;
    let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
    trace!("{:?}", pe);
    Ok(SearchDatabaseItem::new_app(
        file_name,
        file_path.to_str().unwrap().to_string(),
        file_path.to_str().unwrap().to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use std::{
        env,
        path::{Path, PathBuf},
    };

    use super::{parse_lnk, parse_url};
    use crate::core::db::search_database_store::SearchDatabaseItem;
    use crate::core::utils::path::get_cargo_toml_dir;

    #[test]
    fn parse_lnk_test() {
        let lnk_file_name = "Zoom.lnk";
        let root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let data_path = root_path.join("tests/resources/fake_data");
        let zoom_data_path = data_path.join(lnk_file_name);
        let parse_res = parse_lnk(&zoom_data_path, true).expect("Failed to parse .lnk file.");
        dbg!(&parse_res);
        assert_eq!(parse_res.name, lnk_file_name);
    }

    #[test]
    fn parse_url_test() {
        let path = get_cargo_toml_dir()
            .join("tests")
            .join("resources")
            .join("fake_data")
            .join("Arma 3.url");
        let res = parse_url(&path, true).unwrap();
        dbg!(&res);
        assert_eq!(path.to_str().unwrap().to_string(), res.path); // file path check
                                                                  // TODO: file_name check
                                                                  // TODO: file_icon_path check
    }
}
