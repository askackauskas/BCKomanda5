use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

use derive_more::From;
use serde::{de::DeserializeOwned, Deserialize};
use serde_repr::Deserialize_repr;
use ssz_new::SszDecode;

#[derive(PartialEq, Eq, Deserialize_repr)]
#[repr(u8)]
pub enum BlsSetting {
    Optional = 0,
    Required = 1,
    Ignored = 2,
}

impl Default for BlsSetting {
    fn default() -> Self {
        Self::Optional
    }
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Meta {
    pub bls_setting: BlsSetting,
    pub blocks_count: usize,
    pub deposits_count: usize,
}

#[derive(Clone, Copy, From)]
pub struct Case<'path> {
    case_directory_relative_to_repository_root: &'path str,
}

impl<'path> Case<'path> {
    #[must_use]
    pub fn meta(self) -> Meta {
        self.try_yaml("meta").unwrap_or_default()
    }

    pub fn iterator<D: SszDecode>(
        self,
        object_name: &'path str,
        object_count: usize,
    ) -> impl Iterator<Item = D> + 'path {
        (0..object_count).map(move |index| self.ssz(format!("{}_{}", object_name, index)))
    }

    pub fn bytes(self, file_name: impl AsRef<Path>) -> Vec<u8> {
        try_read(self.resolve().join(file_name)).expect("the file should exist")
    }

    pub fn ssz<D: SszDecode>(self, file_name: impl AsRef<Path>) -> D {
        self.try_ssz(file_name).expect("the SSZ file should exist")
    }

    pub fn try_ssz<D: SszDecode>(self, file_name: impl AsRef<Path>) -> Option<D> {
        let file_path = self.resolve().join(file_name).with_extension("ssz");
        let bytes = try_read(file_path)?;
        let value = D::from_ssz_bytes(bytes.as_slice())
            .expect("the file should contain a value encoded in SSZ");
        Some(value)
    }

    pub fn yaml<D: DeserializeOwned>(self, file_name: impl AsRef<Path>) -> D {
        self.try_yaml(file_name)
            .expect("the YAML file should exist")
    }

    fn try_yaml<D: DeserializeOwned>(self, file_name: impl AsRef<Path>) -> Option<D> {
        let file_path = self.resolve().join(file_name).with_extension("yaml");
        let bytes = try_read(file_path)?;
        let value = serde_yaml::from_slice(bytes.as_slice())
            .expect("the file should contain a value encoded in YAML");
        Some(value)
    }

    fn resolve(self) -> PathBuf {
        // Cargo appears to set the working directory to the crate root when running tests.
        PathBuf::from("..").join(self.case_directory_relative_to_repository_root)
    }
}

fn try_read(file_path: impl AsRef<Path>) -> Option<Vec<u8>> {
    match std::fs::read(file_path) {
        Ok(bytes) => Some(bytes),
        Err(error) if error.kind() == ErrorKind::NotFound => None,
        Err(error) => panic!("could not read the file: {:?}", error),
    }
}
