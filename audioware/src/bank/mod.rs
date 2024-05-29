use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};

use error::ensure_no_duplicate_accross_depots;
use kira::sound::static_sound::StaticSoundData;
use once_cell::sync::OnceCell;
use snafu::ResultExt;

pub mod conflict;
mod error;
pub use error::Error;
mod id;
pub use id::*;

use crate::{
    manifest::{
        conv::{ensure_music, ensure_ono, ensure_sfx, ensure_voice},
        de::{DialogLine, Manifest},
        depot::{R6Audioware, REDmod},
        error::{ensure_manifest_no_duplicates, CannotParseManifestSnafu, CannotReadManifestSnafu},
    },
    ok_or_continue,
};

static UNIQUES: OnceCell<HashMap<UniqueKey, StaticSoundData>> = OnceCell::new();
static GENDERS: OnceCell<HashMap<GenderKey, StaticSoundData>> = OnceCell::new();
static LOCALES: OnceCell<HashMap<LocaleKey, StaticSoundData>> = OnceCell::new();
static MULTIS: OnceCell<HashMap<BothKey, StaticSoundData>> = OnceCell::new();

static LOC_SUB: OnceCell<HashMap<LocaleKey, DialogLine>> = OnceCell::new();
static MUL_SUB: OnceCell<HashMap<BothKey, DialogLine>> = OnceCell::new();

static KEYS: OnceCell<HashSet<Id>> = OnceCell::new();

pub struct Banks;
impl Banks {
    pub fn setup() -> Result<Initialization, Error> {
        let since = Instant::now();

        let mut mods = Vec::with_capacity(30);
        let mut redmod_exists = false;
        if let Ok(redmod) = REDmod::try_new() {
            mods = redmod.mods();
            redmod_exists = true;
        }
        if let Ok(r6audioware) = R6Audioware::try_new() {
            for m in r6audioware.mods().into_iter() {
                if let Err(e) =
                    ensure_no_duplicate_accross_depots(redmod_exists, &m, mods.as_slice())
                {
                    red4ext_rs::error!("{e}");
                    continue;
                }
                mods.push(m);
            }
        }
        mods.sort();

        let mut file: Vec<u8>;
        let mut entries: Manifest;
        let mut ids: HashSet<Id> = HashSet::new();
        let mut uniques: HashMap<UniqueKey, StaticSoundData> = HashMap::new();
        let mut genders: HashMap<GenderKey, StaticSoundData> = HashMap::new();
        let mut single_voices: HashMap<LocaleKey, StaticSoundData> = HashMap::new();
        let mut dual_voices: HashMap<BothKey, StaticSoundData> = HashMap::new();
        let mut single_subs: HashMap<LocaleKey, DialogLine> = HashMap::new();
        let mut dual_subs: HashMap<BothKey, DialogLine> = HashMap::new();
        for m in mods {
            let mut manifests = m.load_manifests();
            manifests.sort();
            for ref manifest in manifests {
                file = ok_or_continue!(std::fs::read(manifest).context(CannotReadManifestSnafu {
                    manifest: manifest.display().to_string(),
                }));
                entries = ok_or_continue!(serde_yaml::from_slice::<Manifest>(file.as_slice())
                    .context(CannotParseManifestSnafu {
                        manifest: manifest.display().to_string(),
                    },));
                ok_or_continue!(ensure_manifest_no_duplicates(&entries));
                if let Some(sfx) = entries.sfx {
                    for (key, value) in sfx {
                        ok_or_continue!(ensure_sfx(
                            key.as_str(),
                            value,
                            &m,
                            &mut ids,
                            &mut uniques
                        ));
                    }
                }
                if let Some(onos) = entries.onos {
                    for (key, value) in onos {
                        ok_or_continue!(ensure_ono(
                            key.as_str(),
                            value,
                            &m,
                            &mut ids,
                            &mut genders
                        ));
                    }
                }
                if let Some(voices) = entries.voices {
                    for (key, value) in voices {
                        ok_or_continue!(ensure_voice(
                            key.as_str(),
                            value,
                            &m,
                            &mut ids,
                            &mut single_voices,
                            &mut dual_voices,
                            &mut single_subs,
                            &mut dual_subs
                        ));
                    }
                }
                if let Some(music) = entries.music {
                    for (key, value) in music {
                        ok_or_continue!(ensure_music(key.as_str(), value, &m, &mut ids));
                    }
                }
            }
        }

        let lengths = ids.iter().fold((0, 0, 0), |acc, x| {
            let (mut odsta, mut odstr, mut imsta) = acc;
            match x {
                Id::OnDemand(Usage::Static(..)) => odsta += 1,
                Id::OnDemand(Usage::Streaming(..)) => odstr += 1,
                Id::InMemory(..) => imsta += 1,
            }
            (odsta, odstr, imsta)
        });

        let report = Initialization {
            duration: Instant::now() - since,
            lengths: format!(
                r##"ids:
- on-demand static audio    -> {}
- on-demand streaming audio -> {}
- in-memory static audio    -> {}"##,
                lengths.0, lengths.1, lengths.2
            ),
            len_ids: ids.len(),
        };

        let _ = KEYS.set(ids);
        let _ = UNIQUES.set(uniques);
        let _ = GENDERS.set(genders);
        let _ = LOCALES.set(single_voices);
        let _ = MULTIS.set(dual_voices);
        let _ = LOC_SUB.set(single_subs);
        let _ = MUL_SUB.set(dual_subs);

        Ok(report)
    }
}

pub struct Initialization {
    duration: Duration,
    lengths: String,
    len_ids: usize,
}

impl std::fmt::Display for Initialization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Initialization {
            duration,
            lengths,
            len_ids,
        } = self;
        write!(
            f,
            r##"{lengths}
for a total of: {len_ids} id(s)
in {duration:?}
"##
        )
    }
}
