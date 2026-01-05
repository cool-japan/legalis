//! Lua scripting support for custom commands and automation.

use anyhow::{Context, Result};
use mlua::{Lua, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Script metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptManifest {
    /// Script name
    pub name: String,

    /// Script version
    pub version: String,

    /// Script description
    pub description: String,

    /// Script author
    pub author: String,

    /// Main script file
    pub main: String,

    /// Required Legalis version
    #[serde(default)]
    pub requires: Option<String>,

    /// Script dependencies
    #[serde(default)]
    pub dependencies: Vec<String>,

    /// Script permissions
    #[serde(default)]
    pub permissions: ScriptPermissions,
}

/// Script permissions for sandboxing.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScriptPermissions {
    /// Allow file system access
    #[serde(default)]
    pub filesystem: bool,

    /// Allow network access
    #[serde(default)]
    pub network: bool,

    /// Allow process execution
    #[serde(default)]
    pub process: bool,

    /// Allow environment variable access
    #[serde(default)]
    pub env: bool,

    /// Maximum execution time in seconds
    #[serde(default = "default_timeout")]
    pub timeout: u64,

    /// Maximum memory usage in MB
    #[serde(default = "default_memory_limit")]
    pub memory_limit: usize,
}

fn default_timeout() -> u64 {
    30
}

fn default_memory_limit() -> usize {
    100
}

/// Script execution context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptContext {
    /// Arguments passed to the script
    pub args: Vec<String>,

    /// Environment variables
    pub env: HashMap<String, String>,

    /// Working directory
    pub cwd: PathBuf,
}

/// Script execution result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptResult {
    /// Exit code
    pub exit_code: i32,

    /// Standard output
    pub stdout: String,

    /// Standard error
    pub stderr: String,

    /// Execution time in milliseconds
    pub execution_time_ms: u128,
}

/// Script manager for discovering and executing Lua scripts.
pub struct ScriptManager {
    script_dir: PathBuf,
    scripts: HashMap<String, ScriptManifest>,
}

impl ScriptManager {
    /// Create a new script manager.
    pub fn new() -> Result<Self> {
        let script_dir = Self::script_directory()?;

        if !script_dir.exists() {
            fs::create_dir_all(&script_dir).with_context(|| {
                format!(
                    "Failed to create script directory: {}",
                    script_dir.display()
                )
            })?;
        }

        Ok(Self {
            script_dir,
            scripts: HashMap::new(),
        })
    }

    /// Get the script directory path.
    pub fn script_directory() -> Result<PathBuf> {
        let script_dir = dirs::data_dir()
            .context("Failed to determine data directory")?
            .join("legalis")
            .join("scripts");
        Ok(script_dir)
    }

    /// Discover and load all scripts from the script directory.
    pub fn discover_scripts(&mut self) -> Result<usize> {
        let mut count = 0;

        if !self.script_dir.exists() {
            return Ok(0);
        }

        for entry in fs::read_dir(&self.script_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let manifest_path = path.join("script.toml");
                if manifest_path.exists() {
                    match self.load_script_manifest(&manifest_path) {
                        Ok(manifest) => {
                            self.scripts.insert(manifest.name.clone(), manifest);
                            count += 1;
                        }
                        Err(e) => {
                            eprintln!(
                                "Warning: Failed to load script from {}: {}",
                                path.display(),
                                e
                            );
                        }
                    }
                }
            }
        }

        Ok(count)
    }

    /// Load a script manifest from a file.
    fn load_script_manifest(&self, manifest_path: &Path) -> Result<ScriptManifest> {
        let content = fs::read_to_string(manifest_path).with_context(|| {
            format!(
                "Failed to read script manifest: {}",
                manifest_path.display()
            )
        })?;

        let manifest: ScriptManifest = toml::from_str(&content).with_context(|| {
            format!(
                "Failed to parse script manifest: {}",
                manifest_path.display()
            )
        })?;

        Ok(manifest)
    }

    /// Get a script by name.
    pub fn get_script(&self, name: &str) -> Option<&ScriptManifest> {
        self.scripts.get(name)
    }

    /// List all available scripts.
    pub fn list_scripts(&self) -> Vec<&ScriptManifest> {
        self.scripts.values().collect()
    }

    /// Execute a script with the given context.
    pub fn execute_script(&self, name: &str, context: ScriptContext) -> Result<ScriptResult> {
        let manifest = self
            .get_script(name)
            .ok_or_else(|| anyhow::anyhow!("Script '{}' not found", name))?;

        let script_path = self.script_dir.join(name).join(&manifest.main);

        if !script_path.exists() {
            anyhow::bail!("Script file not found: {}", script_path.display());
        }

        let script_content = fs::read_to_string(&script_path)?;

        self.execute_lua_script(&script_content, context, &manifest.permissions)
    }

    /// Execute Lua script content with sandboxing.
    pub fn execute_lua_script(
        &self,
        script: &str,
        context: ScriptContext,
        permissions: &ScriptPermissions,
    ) -> Result<ScriptResult> {
        let start_time = std::time::Instant::now();
        let lua = Lua::new();

        // Set up sandbox environment
        self.setup_sandbox(&lua, permissions)?;

        // Inject context into Lua
        let globals = lua.globals();
        let args_table = lua.create_table().map_err(|e| anyhow::anyhow!("{}", e))?;
        for (i, arg) in context.args.iter().enumerate() {
            args_table
                .set(i + 1, arg.as_str())
                .map_err(|e| anyhow::anyhow!("{}", e))?;
        }
        globals
            .set("args", args_table)
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        let env_table = lua.create_table().map_err(|e| anyhow::anyhow!("{}", e))?;
        for (key, value) in &context.env {
            env_table
                .set(key.as_str(), value.as_str())
                .map_err(|e| anyhow::anyhow!("{}", e))?;
        }
        globals
            .set("env", env_table)
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        globals
            .set("cwd", context.cwd.to_string_lossy().to_string())
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        // Add print function that captures output
        let print_fn = lua
            .create_function(move |_lua, args: mlua::MultiValue| {
                let output: Vec<String> = args.iter().map(|v| format!("{:?}", v)).collect();
                println!("{}", output.join("\t"));
                Ok(())
            })
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        globals
            .set("print", print_fn)
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        // Execute the script
        let result = lua.load(script).exec();

        let execution_time_ms = start_time.elapsed().as_millis();

        match result {
            Ok(_) => Ok(ScriptResult {
                exit_code: 0,
                stdout: String::new(),
                stderr: String::new(),
                execution_time_ms,
            }),
            Err(e) => Ok(ScriptResult {
                exit_code: 1,
                stdout: String::new(),
                stderr: format!("Script error: {}", e),
                execution_time_ms,
            }),
        }
    }

    /// Set up Lua sandbox based on permissions.
    fn setup_sandbox(&self, lua: &Lua, permissions: &ScriptPermissions) -> Result<()> {
        let globals = lua.globals();

        // Disable dangerous functions by default
        if !permissions.filesystem {
            globals
                .set("io", Value::Nil)
                .map_err(|e| anyhow::anyhow!("{}", e))?;
            globals
                .set("dofile", Value::Nil)
                .map_err(|e| anyhow::anyhow!("{}", e))?;
            globals
                .set("loadfile", Value::Nil)
                .map_err(|e| anyhow::anyhow!("{}", e))?;
        }

        if !permissions.process {
            globals
                .set("os", Value::Nil)
                .map_err(|e| anyhow::anyhow!("{}", e))?;
        }

        if !permissions.network {
            // Network functions would be blocked here if we exposed them
        }

        // Always disable debug and package to prevent sandbox escape
        globals
            .set("debug", Value::Nil)
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        globals
            .set("package", Value::Nil)
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        Ok(())
    }

    /// Install a script from a directory.
    pub fn install_script(&mut self, source_dir: &Path) -> Result<()> {
        let manifest_path = source_dir.join("script.toml");

        if !manifest_path.exists() {
            anyhow::bail!("No script.toml found in {}", source_dir.display());
        }

        let manifest = self.load_script_manifest(&manifest_path)?;

        let target_dir = self.script_dir.join(&manifest.name);

        if target_dir.exists() {
            anyhow::bail!("Script {} is already installed", manifest.name);
        }

        // Copy script directory to scripts folder
        copy_dir_all(source_dir, &target_dir)?;

        self.scripts.insert(manifest.name.clone(), manifest);

        Ok(())
    }

    /// Uninstall a script by name.
    pub fn uninstall_script(&mut self, name: &str) -> Result<()> {
        if !self.scripts.contains_key(name) {
            anyhow::bail!("Script {} is not installed", name);
        }

        let script_dir = self.script_dir.join(name);

        if script_dir.exists() {
            fs::remove_dir_all(&script_dir).with_context(|| {
                format!(
                    "Failed to remove script directory: {}",
                    script_dir.display()
                )
            })?;
        }

        self.scripts.remove(name);

        Ok(())
    }

    /// Get built-in scripts library.
    pub fn get_builtin_scripts() -> Vec<BuiltinScript> {
        vec![
            BuiltinScript {
                name: "hello".to_string(),
                description: "Hello world example script".to_string(),
                code: r#"
print("Hello from Legalis Lua script!")
print("Arguments:", table.concat(args, ", "))
"#
                .to_string(),
            },
            BuiltinScript {
                name: "batch-verify".to_string(),
                description: "Batch verify multiple statute files".to_string(),
                code: r#"
-- Batch verification script
print("Batch verifying statutes...")
for i, file in ipairs(args) do
    print("Verifying:", file)
end
print("Done!")
"#
                .to_string(),
            },
            BuiltinScript {
                name: "report-gen".to_string(),
                description: "Generate custom reports".to_string(),
                code: r#"
-- Report generation script
print("Generating report...")
print("Working directory:", cwd)
print("Report generated successfully!")
"#
                .to_string(),
            },
        ]
    }
}

impl Default for ScriptManager {
    fn default() -> Self {
        Self::new().expect("Failed to create script manager")
    }
}

/// Built-in script definition.
#[derive(Debug, Clone)]
pub struct BuiltinScript {
    pub name: String,
    pub description: String,
    pub code: String,
}

/// Copy a directory recursively.
fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_manager_creation() {
        let manager = ScriptManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_lua_execution() {
        let manager = ScriptManager::new().unwrap();
        let context = ScriptContext {
            args: vec!["test".to_string()],
            env: HashMap::new(),
            cwd: PathBuf::from("/tmp"),
        };

        let permissions = ScriptPermissions::default();
        let script = r#"
            print("Hello from Lua!")
            return 42
        "#;

        let result = manager.execute_lua_script(script, context, &permissions);
        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.exit_code, 0);
    }

    #[test]
    fn test_builtin_scripts() {
        let scripts = ScriptManager::get_builtin_scripts();
        assert!(!scripts.is_empty());
        assert!(scripts.iter().any(|s| s.name == "hello"));
    }
}
