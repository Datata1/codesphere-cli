// in the future refactore this further so that every endpoint can have its own models exported

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}
