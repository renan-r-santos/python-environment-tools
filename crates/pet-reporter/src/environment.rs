// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use pet_core::{
    arch::Architecture,
    python_environment::{PythonEnvironment, PythonEnvironmentCategory},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::manager::Manager;

// We want to maintain full control over serialization instead of relying on the enums or the like.
// Else its too easy to break the API by changing the enum variants.
fn python_category_to_string(category: &PythonEnvironmentCategory) -> &'static str {
    match category {
        PythonEnvironmentCategory::System => "system",
        PythonEnvironmentCategory::Homebrew => "homebrew",
        PythonEnvironmentCategory::Conda => "conda",
        PythonEnvironmentCategory::Pyenv => "pyenv",
        PythonEnvironmentCategory::PyenvVirtualEnv => "pyenv-virtualenv",
        PythonEnvironmentCategory::WindowsStore => "windows-store",
        PythonEnvironmentCategory::WindowsRegistry => "windows-registry",
        PythonEnvironmentCategory::Pipenv => "pipenv",
        PythonEnvironmentCategory::VirtualEnvWrapper => "virtualenvwrapper",
        PythonEnvironmentCategory::Venv => "venv",
        PythonEnvironmentCategory::VirtualEnv => "virtualenv",
        PythonEnvironmentCategory::Unknown => "unknown",
    }
}

// We want to maintain full control over serialization instead of relying on the enums or the like.
// Else its too easy to break the API by changing the enum variants.
fn architecture_to_string(arch: &Architecture) -> &'static str {
    match arch {
        Architecture::X64 => "x64",
        Architecture::X86 => "x86",
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct Environment {
    pub display_name: Option<String>,
    pub name: Option<String>,
    pub executable: Option<PathBuf>,
    pub category: &'static str,
    pub version: Option<String>,
    pub prefix: Option<PathBuf>,
    pub manager: Option<Manager>,
    pub project: Option<PathBuf>,
    pub arch: Option<&'static str>,
    pub symlinks: Option<Vec<PathBuf>>,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Environment ({})", self.category).unwrap_or_default();
        if let Some(name) = &self.display_name {
            writeln!(f, "   Display-Name: {}", name).unwrap_or_default();
        }
        if let Some(name) = &self.name {
            writeln!(f, "   Name        : {}", name).unwrap_or_default();
        }
        if let Some(exe) = &self.executable {
            writeln!(f, "   Executable  : {}", exe.to_str().unwrap_or_default())
                .unwrap_or_default();
        }
        if let Some(version) = &self.version {
            writeln!(f, "   Version     : {}", version).unwrap_or_default();
        }
        if let Some(prefix) = &self.prefix {
            writeln!(
                f,
                "   Prefix      : {}",
                prefix.to_str().unwrap_or_default()
            )
            .unwrap_or_default();
        }
        if let Some(project) = &self.project {
            writeln!(f, "   Project     : {}", project.to_str().unwrap()).unwrap_or_default();
        }
        if let Some(arch) = &self.arch {
            writeln!(f, "   Architecture: {}", arch).unwrap_or_default();
        }
        if let Some(manager) = &self.manager {
            writeln!(
                f,
                "   Manager     : {}, {}",
                manager.tool,
                manager.executable.to_str().unwrap_or_default()
            )
            .unwrap_or_default();
        }
        Ok(())
    }
}

impl Environment {
    pub fn from(env: &PythonEnvironment) -> Environment {
        Environment {
            display_name: env.display_name.clone(),
            name: env.name.clone(),
            executable: env.executable.clone(),
            category: python_category_to_string(&env.category),
            version: env.version.clone(),
            prefix: env.prefix.clone(),
            manager: match &env.manager {
                Some(manager) => Manager::from(manager).into(),
                None => None,
            },
            project: env.project.clone(),
            arch: env.arch.as_ref().map(architecture_to_string),
            symlinks: env.symlinks.clone(),
        }
    }
}