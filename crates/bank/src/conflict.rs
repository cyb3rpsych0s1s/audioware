//! Identify potential conflicts between instances of same type.

use std::collections::HashSet;

use red4ext_rs::types::CName;

use super::{BothKey, GenderKey, Id, Key, LocaleKey, UniqueKey};

/// Identify a type as potentially conflictual.
pub trait Conflictual {}

/// Search indexes for conflictual combination.
pub trait Conflict<T: Conflictual> {
    fn conflict(&self, other: &T) -> bool;
}

impl Conflict<UniqueKey> for HashSet<Id> {
    /// Unique key must not conflict with any kind of id.
    fn conflict(&self, other: &UniqueKey) -> bool {
        for id in self.iter() {
            if AsRef::<CName>::as_ref(id) == &other.0 {
                return true;
            }
        }
        false
    }
}

impl Conflict<GenderKey> for HashSet<Id> {
    /// Gender key must not conflict with other kind of id, and self-duplicate.
    fn conflict(&self, other: &GenderKey) -> bool {
        for id in self.iter() {
            match id.as_ref() {
                Key::Locale(LocaleKey(cname, _))
                | Key::Both(BothKey(cname, ..))
                | Key::Unique(UniqueKey(cname))
                    if cname == &other.0 =>
                {
                    return true;
                }
                Key::Gender(GenderKey(cname, gender))
                    if cname == &other.0 && gender == &other.1 =>
                {
                    return true;
                }
                _ => continue,
            }
        }
        false
    }
}

impl Conflict<LocaleKey> for HashSet<Id> {
    /// Locale key must not conflict with other kind of id, and self-duplicate.
    fn conflict(&self, other: &LocaleKey) -> bool {
        for id in self.iter() {
            match id.as_ref() {
                Key::Unique(UniqueKey(key))
                | Key::Gender(GenderKey(key, _))
                | Key::Both(BothKey(key, ..))
                    if key == &other.0 =>
                {
                    return true;
                }
                Key::Locale(LocaleKey(key, locale)) if key == &other.0 && locale == &other.1 => {
                    return true;
                }
                _ => continue,
            }
        }
        false
    }
}

impl Conflict<BothKey> for HashSet<Id> {
    /// Both key must not conflict with other kind of id, and self-duplicate.
    fn conflict(&self, other: &BothKey) -> bool {
        for id in self.iter() {
            match id.as_ref() {
                Key::Unique(UniqueKey(key))
                | Key::Gender(GenderKey(key, _))
                | Key::Locale(LocaleKey(key, _))
                    if key == &other.0 =>
                {
                    return true;
                }
                Key::Both(BothKey(key, locale, gender))
                    if key == &other.0 && locale == &other.1 && gender == &other.2 =>
                {
                    return true;
                }
                _ => continue,
            }
        }
        false
    }
}

impl Conflictual for UniqueKey {}
impl Conflictual for GenderKey {}
impl Conflictual for LocaleKey {}
impl Conflictual for BothKey {}
