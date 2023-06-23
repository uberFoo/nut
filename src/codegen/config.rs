//! Code Generation Configuration
//!
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    inner: HashMap<Uuid, ConfigValue>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: Uuid, value: ConfigValue) -> Option<ConfigValue> {
        self.inner.insert(key, value)
    }

    pub fn get(&self, key: &Uuid) -> Option<&ConfigValue> {
        self.inner.get(key)
    }

    pub fn is_singleton(&self, key: &Uuid) -> bool {
        self.get_singleton(key).is_some()
    }

    pub fn get_singleton(&self, key: &Uuid) -> Option<&SingletonObject> {
        if let Some(opt) = self.get(key) {
            if let Some(ref so) = opt.singleton_object {
                Some(&so)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_singleton_objects(&self) -> Vec<(&Uuid, &SingletonObject)> {
        let mut result = Vec::new();

        for (id, value) in &self.inner {
            if let Some(ref s) = value.singleton_object {
                result.push((id, s))
            }
        }

        result
    }

    pub fn is_imported(&self, key: &Uuid) -> bool {
        self.get_imported(key).is_some()
    }

    pub fn get_imported(&self, key: &Uuid) -> Option<&ImportedObject> {
        if let Some(opt) = self.get(key) {
            if let Some(ref io) = opt.imported_object {
                Some(&io)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_imported_objects(&self) -> Vec<(&Uuid, &ImportedObject)> {
        let mut result = Vec::new();

        for (id, value) in &self.inner {
            if let Some(ref i) = value.imported_object {
                result.push((id, i))
            }
        }

        result
    }
}

/// The Main Configuration
///
/// Originally I expected to hang this off of the json in a model. I think that is still
/// relevant, even if I don't use it much at first. Now I want to embed configuration
/// in the object description. I think that these two ideas can work together.
///
/// I'm imagining something like this in the type description:
///
/// `❗️ { "type”: “Object”, "domain": “sarzak”, “model_file”: “ooa/sarzak.json" }`
///
/// Everything after that fabulous emoji is JSON that can be parsed with one of the
/// configuration options. We can feed the description through here to parse out
/// the configurations. But then where do we store them? With the type, is the
/// easiest answer. I wonder if that will be too disruptive?
///
/// Another option is to parse all of the type descriptions when we load the model,
/// and then build a Config object, as if it were coming from the json file. Less
/// disruptive to the overall theme. It will only work if the config options are
/// globally scoped.
///
/// As time goes on, things will get welded on. For now, it's just imported objects.
/// Soon, I imagine it'll be singleton objects...
///
/// This has recently morphed into ConfigValue so that Config may be the container
/// of all ConfigValues.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigValue {
    pub imported_object: Option<ImportedObject>,
    pub singleton_object: Option<SingletonObject>,
}

impl ConfigValue {
    pub fn new() -> Self {
        Self {
            imported_object: None,
            singleton_object: None,
        }
    }
}

/// An Imported Object
///
/// An imported object belongs to a different domain than the one in which it's being
/// used. When generating relationship traversal macros, we need to know which ObjectStore
/// to query.
///
/// Wouldn't it be cool to suck this out of the object description? That's the way
/// it should be done, not by welding on a [Config] object to the Model. Not that
/// having a config in the model is a bad thing. It's the perfect thing, but it's
/// not very user friendly given the current modeling tool.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImportedObject {
    pub domain: String,
    pub package: String,
    pub model_path: PathBuf,
}

/// A Singleton Object
///
/// This is a type that has a single attribute, `id`. I'm honestly not sure if
/// it's better to mark them as generating as singletons, or just do it based on
/// layout.
///
/// Anyway, this thing translates to a const with the value being the value of the
/// type's `id`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SingletonObject(pub bool);
