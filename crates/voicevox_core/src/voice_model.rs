use enum_map::EnumMap;
use futures::future::join3;
use serde::{de::DeserializeOwned, Deserialize};
use std::io::Read;
use zip::{read::ZipFile, ZipArchive};

use super::*;
use crate::infer::domain::InferenceOperationImpl;
use std::{
    collections::{BTreeMap, HashMap},
    io,
    path::{Path, PathBuf},
};

/// [`VoiceModelId`]の実体。
///
/// [`VoiceModelId`]: VoiceModelId
pub type RawVoiceModelId = String;

/// 音声モデルID。
#[derive(
    PartialEq, Eq, Clone, Ord, PartialOrd, Deserialize, new, Getters, derive_more::Display, Debug,
)]
pub struct VoiceModelId {
    raw_voice_model_id: RawVoiceModelId,
}

/// 音声モデル。
///
/// VVMファイルと対応する。
#[derive(Getters, Clone)]
pub struct VoiceModel {
    /// ID。
    id: VoiceModelId,
    manifest: Manifest,
    /// メタ情報。
    metas: VoiceModelMeta,
    path: PathBuf,
}

impl VoiceModel {
    pub(crate) async fn read_inference_models(
        &self,
    ) -> LoadModelResult<EnumMap<InferenceOperationImpl, Vec<u8>>> {
        let mut reader = VvmEntryReader::open(&self.path).await?;

        let predict_duration_model_result = reader
            .read_vvm_entry(self.manifest.predict_duration_filename())
            .await?;
        let predict_intonation_model_result = reader
            .read_vvm_entry(self.manifest.predict_intonation_filename())
            .await?;
        let decode_model_result = reader
            .read_vvm_entry(self.manifest.decode_filename())
            .await?;

        Ok(EnumMap::from_array([
            predict_duration_model_result,
            predict_intonation_model_result,
            decode_model_result,
        ]))
    }
    /// VVMファイルから`VoiceModel`をコンストラクトする。
    pub async fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let mut reader = VvmEntryReader::open(path.as_ref()).await?;
        let manifest = reader.read_vvm_json::<Manifest>("manifest.json").await?;
        let metas = reader
            .read_vvm_json::<VoiceModelMeta>(manifest.metas_filename())
            .await?;
        let id = VoiceModelId::new(nanoid!());

        Ok(Self {
            id,
            metas,
            manifest,
            path: path.as_ref().into(),
        })
    }

    /// モデル内のすべてのスタイルに対するモデル内IDを取得する。
    ///
    /// モデル内IDのマッピングが存在しない場合はそのままスタイルIDを返す。
    pub(crate) fn model_inner_ids(&self) -> BTreeMap<StyleId, ModelInnerId> {
        self.metas
            .iter()
            .flat_map(SpeakerMeta::styles)
            .map(StyleMeta::id)
            .map(|&style_id| {
                let model_inner_id = self
                    .manifest
                    .style_id_to_model_inner_id()
                    .get(&style_id)
                    .copied()
                    .unwrap_or_else(|| ModelInnerId::new(style_id.raw_id()));
                (style_id, model_inner_id)
            })
            .collect()
    }
}

struct VvmEntry<'a> {
    index: usize,
    entry: ZipFile<'a>,
}

#[derive(new)]
struct VvmEntryReader {
    reader: ZipArchive<std::io::Cursor<Vec<u8>>>,
    path: PathBuf,
}

impl VvmEntryReader {
    async fn open(path: &Path) -> LoadModelResult<Self> {
        let file = std::fs::read(path).map_err(|source| LoadModelError {
            path: path.to_owned(),
            context: LoadModelErrorKind::OpenZipFile,
            source: Some(source.into()),
        })?;
        let reader = ZipArchive::new(io::Cursor::new(file)).map_err(|source| LoadModelError {
            path: path.to_owned(),
            context: LoadModelErrorKind::OpenZipFile,
            source: Some(source.into()),
        })?;
        Ok(VvmEntryReader::new(reader, path.to_owned()))
    }
    async fn read_vvm_json<T: DeserializeOwned>(&mut self, filename: &str) -> LoadModelResult<T> {
        let bytes = self.read_vvm_entry(filename).await?;
        serde_json::from_slice(&bytes).map_err(|source| LoadModelError {
            path: self.path.to_owned(),
            context: LoadModelErrorKind::ReadZipEntry {
                filename: filename.to_owned(),
            },
            source: Some(source.into()),
        })
    }

    async fn read_vvm_entry(&mut self, filename: &str) -> LoadModelResult<Vec<u8>> {
        async {
            let mut zip_file: Option<usize> = None;
            for i in 0..self.reader.len() {
                let entry = self.reader.by_index(i)?;
                if entry.name() == filename {
                    zip_file = Some(i);
                    break;
                }
            }

            let mut zip_file = self
                .reader
                .by_index(zip_file.ok_or_else(|| LoadModelError {
                    path: self.path.to_owned(),
                    context: LoadModelErrorKind::ReadZipEntry {
                        filename: filename.to_owned(),
                    },
                    source: None,
                })?)?;

            let mut buf = Vec::with_capacity(zip_file.size() as usize);
            zip_file.read_to_end(&mut buf)?;
            Ok::<_, anyhow::Error>(buf)
        }
        .await
        .map_err(|source| LoadModelError {
            path: self.path.to_owned(),
            context: LoadModelErrorKind::ReadZipEntry {
                filename: filename.to_owned(),
            },
            source: Some(source),
        })
    }
}
