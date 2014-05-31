
//! Storing sounds, textures, animations etc.

// Extern crates.
use std::os::self_exe_path;

/// A place to store sounds, textures, animations etc.
///
/// The idea is to have one object which the app can use
/// to load assets for the game with a simple interface.
pub struct AssetStore {
    // The folder to load assets from.
    assets_folder: Option<String>,
}

impl AssetStore {
    /// Creates a new `AssetStore` from an assets folder.
    pub fn from_folder(assets_folder: &str) -> AssetStore {
        AssetStore {
            assets_folder: Some(assets_folder.to_string()),
        }
    }

    /// Creates an empty `AssetStore` with no assets.
    pub fn empty() -> AssetStore {
        AssetStore {
            assets_folder: None,
        }
    }
   
    /// Returns the path of an asset file. 
    pub fn path(&mut self, file: &str) -> Result<Path, String> {
        let folder = self.assets_folder.as_ref().unwrap();
        let exe_path = self_exe_path();
        let exe_path = match exe_path {
            Some(path) => path,
            None => return Err("Could not get the path to executable".to_string()),
        };
        Ok(exe_path.join(Path::new(folder.as_slice())).join(Path::new(file)))
    }
}

