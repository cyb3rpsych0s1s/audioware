use std::{sync::{Arc, Mutex}, collections::HashMap, ops::Deref};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::fmt::Debug;

use crate::audio::{Audio, Ono};

lazy_static! {
    static ref BANKS: Arc<Mutex<Banks>> = Arc::new(Mutex::new(Banks::default()));
}

#[derive(Debug, Clone, Deserialize, Default)]
struct Banks(HashMap<String, Bank>);

#[derive(Debug, Clone, Deserialize, Default)]
struct Bank(HashMap<String, Ono>);
// impl Bank {
//     fn get(&self, sfx: impl AsRef<str>) -> Option<&dyn Audio> {
//         self.0.get(sfx.as_ref()).map(|a| a.deref())
//     }
// }
