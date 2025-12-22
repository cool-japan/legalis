//! Language Server Protocol (LSP) implementation for Legalis DSL.
//!
//! This module provides LSP support for the Legalis DSL, enabling features like:
//! - Real-time syntax error diagnostics
//! - Hover information for keywords and statutes
//! - Code completion for keywords
//! - Document symbols navigation

use crate::{DslError, LegalDslParser};
use std::collections::HashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

/// The Legalis LSP backend.
pub struct LegalisLspBackend {
    client: Client,
    document_map: tokio::sync::RwLock<HashMap<String, String>>,
}

impl LegalisLspBackend {
    /// Creates a new LSP backend.
    pub fn new(client: Client) -> Self {
        Self {
            client,
            document_map: tokio::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Parses a document and returns diagnostics.
    async fn validate_document(&self, _uri: &Url, text: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let parser = LegalDslParser::new();

        match parser.parse_document(text) {
            Ok(_doc) => {
                // Document parsed successfully
                // Check for warnings
                for warning in parser.warnings() {
                    let diagnostic = Diagnostic {
                        range: Range {
                            start: Position {
                                line: (warning.location().line.saturating_sub(1)) as u32,
                                character: (warning.location().column.saturating_sub(1)) as u32,
                            },
                            end: Position {
                                line: (warning.location().line.saturating_sub(1)) as u32,
                                character: (warning.location().column + 10) as u32,
                            },
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        code: None,
                        source: Some("legalis-dsl".to_string()),
                        message: format!("{}", warning),
                        related_information: None,
                        tags: None,
                        code_description: None,
                        data: None,
                    };
                    diagnostics.push(diagnostic);
                }
            }
            Err(e) => {
                // Parse error occurred
                let (line, column, message) = match &e {
                    DslError::ParseError { location, message } => {
                        if let Some(loc) = location {
                            (loc.line, loc.column, message.clone())
                        } else {
                            (1, 1, message.clone())
                        }
                    }
                    DslError::SyntaxError {
                        location, message, ..
                    } => (location.line, location.column, message.clone()),
                    DslError::UndefinedReference { location, name, .. } => (
                        location.line,
                        location.column,
                        format!("Undefined reference: {}", name),
                    ),
                    DslError::UnclosedComment(location) => {
                        if let Some(loc) = location {
                            (loc.line, loc.column, "Unclosed comment".to_string())
                        } else {
                            (1, 1, "Unclosed comment".to_string())
                        }
                    }
                    _ => (1, 1, format!("{}", e)),
                };

                let diagnostic = Diagnostic {
                    range: Range {
                        start: Position {
                            line: (line.saturating_sub(1)) as u32,
                            character: (column.saturating_sub(1)) as u32,
                        },
                        end: Position {
                            line: (line.saturating_sub(1)) as u32,
                            character: (column + 10) as u32,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    source: Some("legalis-dsl".to_string()),
                    message,
                    related_information: None,
                    tags: None,
                    code_description: None,
                    data: None,
                };
                diagnostics.push(diagnostic);
            }
        }

        diagnostics
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for LegalisLspBackend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "legalis-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![" ".to_string(), "\n".to_string()]),
                    ..Default::default()
                }),
                document_symbol_provider: Some(OneOf::Left(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Legalis LSP server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.text_document.text;

        // Store document
        self.document_map
            .write()
            .await
            .insert(uri.clone(), text.clone());

        // Validate and send diagnostics
        let diagnostics = self
            .validate_document(&params.text_document.uri, &text)
            .await;
        self.client
            .publish_diagnostics(params.text_document.uri, diagnostics, None)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        if let Some(change) = params.content_changes.first() {
            let text = change.text.clone();

            // Update document
            self.document_map
                .write()
                .await
                .insert(uri.clone(), text.clone());

            // Validate and send diagnostics
            let diagnostics = self
                .validate_document(&params.text_document.uri, &text)
                .await;
            self.client
                .publish_diagnostics(params.text_document.uri, diagnostics, None)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        self.document_map.write().await.remove(&uri);
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .to_string();
        let position = params.text_document_position_params.position;

        let doc_map = self.document_map.read().await;
        if let Some(text) = doc_map.get(&uri) {
            // Get the word at cursor position
            let lines: Vec<&str> = text.lines().collect();
            if let Some(line) = lines.get(position.line as usize) {
                let word = get_word_at_position(line, position.character as usize);

                let hover_text = match word.to_uppercase().as_str() {
                    "STATUTE" => "Defines a legal statute with conditions and effects.",
                    "WHEN" => "Specifies conditions that must be met for the statute to apply.",
                    "THEN" => "Defines the effect or outcome when conditions are satisfied.",
                    "UNLESS" => "Specifies negative conditions (inverted WHEN).",
                    "REQUIRES" => "Declares dependencies on other statutes.",
                    "DISCRETION" => "Describes discretionary enforcement or interpretation.",
                    "EXCEPTION" => "Defines exception cases to the main rule.",
                    "AMENDMENT" => "Tracks amendments to other statutes.",
                    "SUPERSEDES" => "Indicates this statute replaces older statutes.",
                    "DEFAULT" => "Specifies default values for attributes.",
                    "AND" => "Logical AND operator for combining conditions.",
                    "OR" => "Logical OR operator for combining conditions.",
                    "NOT" => "Logical NOT operator for negating conditions.",
                    "HAS" => "Checks if an entity has a specific attribute.",
                    "BETWEEN" => "Range operator (e.g., AGE BETWEEN 18 AND 65).",
                    "IN" => "Set membership operator (e.g., status IN (\"active\", \"pending\")).",
                    "LIKE" => "Pattern matching operator for strings.",
                    "MATCHES" => "Regular expression pattern matching.",
                    "IN_RANGE" => "Numeric range with inclusive/exclusive bounds.",
                    "GRANT" => "Grants a right or permission.",
                    "REVOKE" => "Revokes a right or permission.",
                    "OBLIGATION" => "Imposes a duty or requirement.",
                    "PROHIBITION" => "Forbids an action.",
                    "JURISDICTION" => "Specifies the geographic or legal jurisdiction.",
                    "VERSION" => "Specifies the version number of the statute.",
                    "EFFECTIVE_DATE" => "Date when the statute takes effect.",
                    "EXPIRY_DATE" => "Date when the statute expires (sunset clause).",
                    "AGE" => "Built-in attribute for age-based conditions.",
                    "INCOME" => "Built-in attribute for income-based conditions.",
                    _ => return Ok(None),
                };

                return Ok(Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(hover_text.to_string())),
                    range: None,
                }));
            }
        }

        Ok(None)
    }

    async fn completion(&self, _params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let keywords = vec![
            "STATUTE",
            "WHEN",
            "UNLESS",
            "REQUIRES",
            "THEN",
            "DISCRETION",
            "EXCEPTION",
            "AMENDMENT",
            "SUPERSEDES",
            "DEFAULT",
            "AND",
            "OR",
            "NOT",
            "HAS",
            "BETWEEN",
            "IN",
            "LIKE",
            "MATCHES",
            "IN_RANGE",
            "NOT_IN_RANGE",
            "GRANT",
            "REVOKE",
            "OBLIGATION",
            "PROHIBITION",
            "JURISDICTION",
            "VERSION",
            "EFFECTIVE_DATE",
            "EXPIRY_DATE",
            "AGE",
            "INCOME",
        ];

        let completions: Vec<CompletionItem> = keywords
            .iter()
            .map(|&keyword| CompletionItem {
                label: keyword.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some(get_keyword_detail(keyword)),
                ..Default::default()
            })
            .collect();

        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri.to_string();

        let doc_map = self.document_map.read().await;
        if let Some(text) = doc_map.get(&uri) {
            let parser = LegalDslParser::new();
            if let Ok(doc) = parser.parse_document(text) {
                let mut symbols = Vec::new();

                for statute in doc.statutes {
                    #[allow(deprecated)]
                    let symbol = DocumentSymbol {
                        name: statute.id.clone(),
                        detail: Some(statute.title.clone()),
                        kind: SymbolKind::CLASS,
                        tags: None,
                        deprecated: None,
                        range: Range::default(), // Would need source location tracking
                        selection_range: Range::default(),
                        children: None,
                    };
                    symbols.push(symbol);
                }

                return Ok(Some(DocumentSymbolResponse::Nested(symbols)));
            }
        }

        Ok(None)
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = params.text_document.uri.to_string();
        let mut actions = Vec::new();

        // Get diagnostics for this document
        let doc_map = self.document_map.read().await;
        if let Some(text) = doc_map.get(&uri) {
            let parser = LegalDslParser::new();
            let _ = parser.parse_document(text);

            // Generate quick fixes for deprecated syntax warnings
            for warning in parser.warnings() {
                if let crate::DslWarning::DeprecatedSyntax {
                    location,
                    old_syntax,
                    new_syntax,
                    ..
                } = warning
                {
                    let line_idx = location.line.saturating_sub(1);
                    if let Some(line) = text.lines().nth(line_idx) {
                        // Create a code action to replace deprecated syntax
                        if let Some(start_col) = line.find(&old_syntax) {
                            let range = Range {
                                start: Position {
                                    line: line_idx as u32,
                                    character: start_col as u32,
                                },
                                end: Position {
                                    line: line_idx as u32,
                                    character: (start_col + old_syntax.len()) as u32,
                                },
                            };

                            let edit = TextEdit {
                                range,
                                new_text: new_syntax.clone(),
                            };

                            let mut changes = HashMap::new();
                            changes.insert(params.text_document.uri.clone(), vec![edit]);

                            let workspace_edit = WorkspaceEdit {
                                changes: Some(changes),
                                document_changes: None,
                                change_annotations: None,
                            };

                            let action = CodeAction {
                                title: format!("Replace '{}' with '{}'", old_syntax, new_syntax),
                                kind: Some(CodeActionKind::QUICKFIX),
                                diagnostics: None,
                                edit: Some(workspace_edit),
                                command: None,
                                is_preferred: Some(true),
                                disabled: None,
                                data: None,
                            };

                            actions.push(CodeActionOrCommand::CodeAction(action));
                        }
                    }
                }
            }
        }

        if actions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(actions))
        }
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri.to_string();

        let doc_map = self.document_map.read().await;
        if let Some(text) = doc_map.get(&uri) {
            let parser = LegalDslParser::new();
            if let Ok(doc) = parser.parse_document(text) {
                // Use the pretty-printer to format the document
                let formatted = crate::format_document(&doc);

                // Create a single edit that replaces the entire document
                let lines: Vec<&str> = text.lines().collect();
                let last_line = lines.len().saturating_sub(1);
                let last_char = lines.last().map(|l| l.len()).unwrap_or(0);

                let edit = TextEdit {
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 0,
                        },
                        end: Position {
                            line: last_line as u32,
                            character: last_char as u32,
                        },
                    },
                    new_text: formatted,
                };

                return Ok(Some(vec![edit]));
            }
        }

        Ok(None)
    }
}

/// Gets the word at a specific position in a line.
fn get_word_at_position(line: &str, char_pos: usize) -> String {
    let chars: Vec<char> = line.chars().collect();
    if char_pos >= chars.len() {
        return String::new();
    }

    // Find word boundaries
    let mut start = char_pos;
    let mut end = char_pos;

    // Move start back to word beginning
    while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
        start -= 1;
    }

    // Move end forward to word ending
    while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
        end += 1;
    }

    chars[start..end].iter().collect()
}

/// Gets detailed description for a keyword.
fn get_keyword_detail(keyword: &str) -> String {
    match keyword {
        "STATUTE" => "Statute definition block".to_string(),
        "WHEN" => "Condition clause".to_string(),
        "UNLESS" => "Negative condition clause".to_string(),
        "REQUIRES" => "Statute dependency".to_string(),
        "THEN" => "Effect clause".to_string(),
        "DISCRETION" => "Discretionary interpretation".to_string(),
        "EXCEPTION" => "Exception to the rule".to_string(),
        "AMENDMENT" => "Amendment tracking".to_string(),
        "SUPERSEDES" => "Replaces old statute".to_string(),
        "DEFAULT" => "Default attribute value".to_string(),
        "GRANT" => "Grant permission or right".to_string(),
        "REVOKE" => "Revoke permission or right".to_string(),
        "OBLIGATION" => "Impose duty or requirement".to_string(),
        "PROHIBITION" => "Forbid action".to_string(),
        _ => format!("{} keyword", keyword),
    }
}

/// Starts the LSP server.
pub async fn run_lsp_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(LegalisLspBackend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_word_at_position() {
        let line = "STATUTE test_statute: \"Test\" {";
        assert_eq!(get_word_at_position(line, 0), "STATUTE");
        assert_eq!(get_word_at_position(line, 8), "test_statute");
        assert_eq!(get_word_at_position(line, 9), "test_statute");
    }

    #[test]
    fn test_get_keyword_detail() {
        assert!(get_keyword_detail("STATUTE").contains("Statute"));
        assert!(get_keyword_detail("GRANT").contains("Grant"));
    }
}
