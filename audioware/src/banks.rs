use anyhow::Context;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::audio::Sound;

lazy_static! {
    pub static ref BANKS: Arc<Mutex<Banks>> = Arc::new(Mutex::new(Banks::default()));
}

struct REDmod(std::path::PathBuf);
impl REDmod {
    /// retrieve "mods" folder
    fn try_new() -> anyhow::Result<Self> {
        let current_folder = std::env::current_exe()?;
        let redmod_folder = current_folder
            .parent()
            .context("plugins folder")?
            .parent()
            .context("red4ext folder")?
            .parent()
            .context("Cyberpunk 2077 folder")?
            .join("mods");
        if redmod_folder.is_dir() {
            return Ok(Self(redmod_folder));
        }
        anyhow::bail!("{:#?} is not a valid folder", redmod_folder.file_name());
    }
    /// retrieve all mod folders under "mods"
    fn mods(self) -> Vec<Mod> {
        std::fs::read_dir(&self.0)
            .unwrap() // safety: dir already checked
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter_map(|x| {
                let path = x.path();
                if path.is_dir() {
                    return Some(Mod(path));
                }
                None
            })
            .collect()
    }
}
struct Mod(std::path::PathBuf);
impl Mod {
    /// retrieve sound bank from a mod folder, if it exists
    fn bank(self) -> Option<Bank> {
        let mut bank = Bank::default();
        let mut sub_bank: Bank;
        for entry in std::fs::read_dir(&self.0)
            .unwrap()
            .filter_map(std::result::Result::ok)
        {
            if entry.path().is_dir() {}
        }
        if bank.is_empty() {
            return None;
        }
        Some(bank)
    }
}

struct Manifest(std::path::PathBuf);
/// check if path is valid file named "audioware" with YAML extension
fn is_manifest(file: &std::path::Path) -> bool {
    if let Some(stem) = file.file_stem() {
        let stem = stem.to_ascii_lowercase();
        if stem == "audioware" {
            if let Some(extension) = file.extension() {
                let extension = extension.to_ascii_lowercase();
                return extension == "yml" || extension == "yaml";
            }
        }
    }
    false
}

fn visit_dirs(dir: &std::path::Path) -> Option<Bank> {
    let mut bank = visit_dir(dir);
    for entry in std::fs::read_dir(dir)
        .unwrap()
        .filter_map(std::result::Result::ok)
        .filter(|x| x.path().is_dir())
    {
        if let Some(sub_bank) = visit_dirs(&entry.path()) {
            match bank.borrow_mut() {
                Some(current) => {
                    current.merge(sub_bank);
                }
                None => {
                    bank = Some(sub_bank);
                }
            }
        }
    }
    bank
}
fn visit_dir(dir: &std::path::Path) -> Option<Bank> {
    // safety: dir already checked
    for entry in std::fs::read_dir(dir)
        .unwrap()
        .filter_map(std::result::Result::ok)
    {
        if is_manifest(&entry.path()) {
            return std::fs::read(&entry.path())
                .ok()
                .map(|x| serde_yaml::from_slice::<Bank>(x.as_slice()).ok())
                .flatten();
        }
    }
    None
}

pub struct SoundBanks;
impl SoundBanks {
    pub fn initialize() -> anyhow::Result<()> {
        let redmod_folder = REDmod::try_new()?;
        let _mods = redmod_folder.mods();
        Ok(())
    }
    fn create_banks(folder: REDmod) {
        let _dirs = std::fs::read_dir(folder.0);
    }
    pub fn create_bank(name: String) {
        use std::borrow::BorrowMut;
        if let Ok(mut guard) = BANKS.clone().borrow_mut().try_lock() {
            guard.create(name);
        } else {
            red4ext_rs::error!("could not get a handle to banks");
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Banks(HashMap<String, Bank>);

impl Banks {
    pub fn create(&mut self, name: String) {
        if !self.0.contains_key(&name) {
            self.0.insert(name, Bank::default());
        } else {
            red4ext_rs::warn!("banks already contains {name}");
        }
    }
}

impl Deref for Banks {
    type Target = HashMap<String, Bank>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Banks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Bank(HashMap<String, Sound>);
impl Bank {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn merge(&mut self, rhs: Bank) {
        for key in rhs.0.keys() {
            if !self.0.contains_key(key) {
                self.0.insert(key.clone(), rhs.0.get(key).unwrap().clone());
            } else {
                red4ext_rs::warn!("{key} already exists in bank");
            }
        }
    }
}
// impl Bank {
//     fn get(&self, sfx: impl AsRef<str>) -> Option<&dyn Audio> {
//         self.0.get(sfx.as_ref()).map(|a| a.deref())
//     }
// }
