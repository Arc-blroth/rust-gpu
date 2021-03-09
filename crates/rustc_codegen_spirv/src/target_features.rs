use rustc_session::Session;
use rustc_span::symbol::Symbol;

use rspirv::spirv::MemoryModel;

use crate::symbols::Symbols;

/// An extension trait for `rustc_session::Session` to make it easier to pull
/// features we care about out of rustc.
#[derive(Copy, Clone, PartialEq)]
pub struct TargetFeatures {
    pub kernel_mode: bool,
    pub spirv_version: Option<(u8, u8)>,
    pub target_env: Option<spirv_tools::TargetEnv>,
    pub memory_model: Option<MemoryModel>,
}

impl TargetFeatures {
    pub fn from_session(sess: &Session) -> Self {
        let sym = Symbols::get();
        let mut kernel_mode = false;
        let mut spirv_version = None;
        let mut target_env = None;
        let mut memory_model = None;

        for &feature in &sess.target_features {
            if feature == sym.kernel {
                kernel_mode = true;
            } else if feature == sym.spirv10 {
                spirv_version = Some((1, 0));
            } else if feature == sym.spirv11 {
                spirv_version = Some((1, 1));
            } else if feature == sym.spirv12 {
                spirv_version = Some((1, 2));
            } else if feature == sym.spirv13 {
                spirv_version = Some((1, 3));
            } else if feature == sym.spirv14 {
                spirv_version = Some((1, 4));
            } else if feature == sym.spirv15 {
                spirv_version = Some((1, 5));
            } else if feature == sym.simple {
                memory_model = Some(MemoryModel::Simple);
            } else if feature == sym.vulkan {
                memory_model = Some(MemoryModel::Vulkan);
            } else if feature == sym.glsl450 {
                memory_model = Some(MemoryModel::GLSL450);
            } else if let Some(env) = TargetFeatures::parse_target_env_value(sess, feature) {
                target_env = Some(env);
            } else {
                sess.err(&format!("Unknown feature {}", feature));
            }
        }

        Self {
            kernel_mode,
            spirv_version,
            target_env,
            memory_model,
        }
    }

    fn parse_target_env_value(sess: &Session, feature: Symbol) -> Option<spirv_tools::TargetEnv> {
        let feature = feature.as_str();
        if feature.starts_with("target_env") {
            let value = match feature.split('=').nth(1) {
                Some(value) => value,
                None => {
                    sess.err("`target_env` requires a value.");
                    return None;
                }
            };

            match value.parse() {
                Ok(val) => Some(val),
                Err(_) => {
                    sess.err("Invalid `target_env` value.");
                    None
                }
            }
        } else {
            None
        }
    }
}
