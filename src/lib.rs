mod voicevox_core_bindings;

use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};
use std::ffi::{CStr, CString};

use libloading;

pub struct VoicevoxCore {
    core: voicevox_core_bindings::VoicevoxCore,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoicevoxCoreStyle {
    id: isize,
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoicevoxCoreMeta {
    name: String,
    styles: Vec<VoicevoxCoreStyle>,
    speaker_uuid: String,
    version: String,
}

impl VoicevoxCore {
    pub fn new(libpath: String) -> VoicevoxCore {
        let core = unsafe {
            voicevox_core_bindings::VoicevoxCore::new(libpath).expect("Missing vocevoxlib")
        };
        return VoicevoxCore { core }
    }

    pub fn from_library(lib: libloading::Library) -> VoicevoxCore {
        let core = unsafe {
            voicevox_core_bindings::VoicevoxCore::from_library(lib).expect("Missing vocevoxlib")
        };
        return VoicevoxCore { core }
    }

    pub fn initialize(&self, root_dir_path: String, use_gpu: bool) -> bool {
        return unsafe {
            let c_str = CString::new(root_dir_path).unwrap();
            self.core.initialize(c_str.as_ptr(), use_gpu)
        }
    }

    pub fn finalize(&self) {
        unsafe {
            self.core.finalize()
        }
    }

    pub fn metas(&self) -> Result<Vec<VoicevoxCoreMeta>> {
        let char = unsafe {
            self.core.metas()
        };

        if char.is_null() {
            return Err(anyhow!("Failed to get metas"));
        } else {
            let metas: Vec<VoicevoxCoreMeta> = serde_json::from_str(unsafe {
                CStr::from_ptr(char).to_str().expect("Failed to get metas")
            }).unwrap();
            return Ok(metas);
        }
    }

    pub fn yukarin_s_forward(&self, length: i32, phoneme_list: &mut i64, speaker_id: &mut i64, output: &mut f32) -> bool {
        return unsafe {
            self.core.yukarin_s_forward(
                length,
                phoneme_list,
                speaker_id,
                output
            )
        }
    }

    pub fn yukarin_sa_forward(
        &self,
        length: i32,
        vowel_phoneme_list: &mut i64,
        consonant_phoneme_list: &mut i64,
        start_accent_list: &mut i64,
        end_accent_list: &mut i64,
        start_accent_phrase_list: &mut i64,
        end_accent_phrase_list: &mut i64,
        speaker_id: &mut i64,
        output: &mut f32,
    ) -> bool {
        return unsafe {
            self.core.yukarin_sa_forward(
                length,
                vowel_phoneme_list,
                consonant_phoneme_list,
                start_accent_list,
                end_accent_list,
                start_accent_phrase_list,
                end_accent_phrase_list,
                speaker_id,
                output
            )
        }
    }

    pub fn decode_forward(
        &self,
        length: i32,
        phoneme_size: i32,
        f0: &mut f32,
        phoneme: &mut f32,
        speaker_id: &mut i64,
        output: &mut f32,
    ) -> bool {
        return unsafe {
            self.core.decode_forward(
                length,
                phoneme_size,
                f0,
                phoneme,
                speaker_id,
                output
            )
        }
    }

    pub fn last_error_message(&self) -> String {
        return unsafe {
            let char = self.core.last_error_message();
            CStr::from_ptr(char).to_str().expect("failed to get last error message").to_string()
        };
    }
}