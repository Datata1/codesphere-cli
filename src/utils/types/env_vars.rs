use crate::api::EnvVar;

#[derive(Debug)]
pub struct EnvVarsConfig {
    pub vars: Vec<EnvVar>,
}

impl EnvVarsConfig {
    pub fn get_var(&self, name: &str) -> Option<&EnvVar> {
        self.vars.iter().find(|var| var.name == name)
    }
}
