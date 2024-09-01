use zed::settings::LspSettings;
use zed::{serde_json, LanguageServerId};
use zed_extension_api::{self as zed, Result};

struct RubySorbetExtension {}

impl zed::Extension for RubySorbetExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if let Some(path) = worktree.which("bundle") {
            return Ok(zed::Command {
                command: path,
                args: ["exec", "srb", "typecheck", "--lsp"]
                    .iter()
                    .map(|&s| s.to_string())
                    .collect(),
                env: worktree.shell_env(),
            });
        }

        Err("Bundle & Sorbet need to be installed in this project!".to_string())
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let initialization_options =
            LspSettings::for_worktree(language_server_id.as_ref(), worktree)
                .ok()
                .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
                .unwrap_or_default();

        Ok(Some(serde_json::json!(initialization_options)))
    }
}

zed::register_extension!(RubySorbetExtension);
