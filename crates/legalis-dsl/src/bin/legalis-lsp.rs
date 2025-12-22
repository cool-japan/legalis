//! Legalis DSL Language Server
//!
//! This binary provides Language Server Protocol (LSP) support for the Legalis DSL.
//! It can be used with any LSP-compatible editor (VS Code, Neovim, Emacs, etc.).
//!
//! ## Usage
//!
//! ```bash
//! legalis-lsp
//! ```
//!
//! The server communicates via stdin/stdout using the JSON-RPC protocol.
//!
//! ## Features
//!
//! - Real-time syntax error diagnostics
//! - Hover information for keywords
//! - Keyword completion
//! - Document symbols (outline view)
//!
//! ## Editor Configuration
//!
//! ### VS Code
//!
//! Add to your VS Code settings.json:
//! ```json
//! {
//!   "legalis-dsl.server.path": "/path/to/legalis-lsp"
//! }
//! ```
//!
//! ### Neovim (with nvim-lspconfig)
//!
//! Add to your init.lua:
//! ```lua
//! require'lspconfig'.legalis_lsp.setup{
//!   cmd = {'/path/to/legalis-lsp'},
//!   filetypes = {'legalis'},
//! }
//! ```

use legalis_dsl::lsp::run_lsp_server;

#[tokio::main]
async fn main() {
    env_logger::init();
    run_lsp_server().await;
}
