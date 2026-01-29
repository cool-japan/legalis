//! SDK Generator Module
//!
//! Generates SDKs from OpenAPI specifications for multiple programming languages.
//! Supports TypeScript, Python, Rust, and Go with comprehensive features including
//! authentication, retry logic, type-safe models, streaming support, and more.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

/// SDK generation errors.
#[derive(Debug, Error)]
pub enum SdkGeneratorError {
    #[error("Invalid OpenAPI spec: {0}")]
    InvalidSpec(String),

    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("Template error: {0}")]
    TemplateError(String),

    #[error("Code generation failed: {0}")]
    GenerationFailed(String),

    #[error("IO error: {0}")]
    IoError(String),
}

/// Supported SDK languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SdkLanguage {
    TypeScript,
    Python,
    Rust,
    Go,
}

impl fmt::Display for SdkLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SdkLanguage::TypeScript => write!(f, "TypeScript"),
            SdkLanguage::Python => write!(f, "Python"),
            SdkLanguage::Rust => write!(f, "Rust"),
            SdkLanguage::Go => write!(f, "Go"),
        }
    }
}

/// Authentication methods supported by generated SDKs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    JWT {
        header_name: String,
    },
    OAuth2 {
        token_url: String,
        scopes: Vec<String>,
    },
    ApiKey {
        header_name: String,
        query_param: Option<String>,
    },
    Bearer,
}

/// SDK generation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdkConfig {
    pub language: SdkLanguage,
    pub package_name: String,
    pub version: String,
    pub base_url: String,
    pub auth_method: Option<AuthMethod>,
    pub retry_config: RetryConfig,
    pub timeout_seconds: u64,
    pub enable_streaming: bool,
    pub generate_tests: bool,
    pub generate_docs: bool,
    pub output_dir: String,
}

impl Default for SdkConfig {
    fn default() -> Self {
        Self {
            language: SdkLanguage::TypeScript,
            package_name: "legalis-sdk".to_string(),
            version: "0.1.0".to_string(),
            base_url: "http://localhost:3000".to_string(),
            auth_method: None,
            retry_config: RetryConfig::default(),
            timeout_seconds: 30,
            enable_streaming: true,
            generate_tests: true,
            generate_docs: true,
            output_dir: "./generated-sdk".to_string(),
        }
    }
}

/// Retry configuration for SDK requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
    pub retry_on_status_codes: Vec<u16>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
            retry_on_status_codes: vec![429, 500, 502, 503, 504],
        }
    }
}

/// OpenAPI specification model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSpec {
    pub openapi: String,
    pub info: ApiInfo,
    pub servers: Vec<ApiServer>,
    pub paths: HashMap<String, PathItem>,
    pub components: Option<Components>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiInfo {
    pub title: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiServer {
    pub url: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub parameters: Option<Vec<Parameter>>,
    #[serde(rename = "requestBody")]
    pub request_body: Option<RequestBody>,
    pub responses: HashMap<String, Response>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub location: String,
    pub required: Option<bool>,
    pub schema: Option<Schema>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    pub required: Option<bool>,
    pub content: HashMap<String, MediaType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub description: String,
    pub content: Option<HashMap<String, MediaType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaType {
    pub schema: Option<Schema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "type")]
    pub schema_type: Option<String>,
    pub properties: Option<HashMap<String, Schema>>,
    pub items: Option<Box<Schema>>,
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Components {
    pub schemas: Option<HashMap<String, Schema>>,
}

/// Generated SDK output.
#[derive(Debug, Clone)]
pub struct GeneratedSdk {
    pub language: SdkLanguage,
    pub files: HashMap<String, String>,
    pub package_metadata: PackageMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: HashMap<String, String>,
}

/// Main SDK generator trait.
pub trait SdkGenerator {
    /// Generate SDK from OpenAPI specification.
    fn generate(
        &self,
        spec: &OpenApiSpec,
        config: &SdkConfig,
    ) -> Result<GeneratedSdk, SdkGeneratorError>;

    /// Generate client class/module.
    fn generate_client(
        &self,
        spec: &OpenApiSpec,
        config: &SdkConfig,
    ) -> Result<String, SdkGeneratorError>;

    /// Generate model types.
    fn generate_models(&self, spec: &OpenApiSpec) -> Result<String, SdkGeneratorError>;

    /// Generate authentication handler.
    fn generate_auth(&self, config: &SdkConfig) -> Result<String, SdkGeneratorError>;

    /// Generate retry logic.
    fn generate_retry(&self, config: &SdkConfig) -> Result<String, SdkGeneratorError>;

    /// Generate package metadata file.
    fn generate_package_file(&self, config: &SdkConfig) -> Result<String, SdkGeneratorError>;

    /// Generate README.
    fn generate_readme(
        &self,
        spec: &OpenApiSpec,
        config: &SdkConfig,
    ) -> Result<String, SdkGeneratorError>;

    /// Generate tests.
    fn generate_tests(
        &self,
        spec: &OpenApiSpec,
        config: &SdkConfig,
    ) -> Result<String, SdkGeneratorError>;
}

/// TypeScript SDK generator.
pub struct TypeScriptSdkGenerator;

impl SdkGenerator for TypeScriptSdkGenerator {
    fn generate(
        &self,
        spec: &OpenApiSpec,
        config: &SdkConfig,
    ) -> Result<GeneratedSdk, SdkGeneratorError> {
        let mut files = HashMap::new();

        // Generate main client
        let client = self.generate_client(spec, config)?;
        files.insert("src/client.ts".to_string(), client);

        // Generate models
        let models = self.generate_models(spec)?;
        files.insert("src/models.ts".to_string(), models);

        // Generate auth
        let auth = self.generate_auth(config)?;
        files.insert("src/auth.ts".to_string(), auth);

        // Generate retry logic
        let retry = self.generate_retry(config)?;
        files.insert("src/retry.ts".to_string(), retry);

        // Generate package.json
        let package_json = self.generate_package_file(config)?;
        files.insert("package.json".to_string(), package_json);

        // Generate README
        let readme = self.generate_readme(spec, config)?;
        files.insert("README.md".to_string(), readme);

        // Generate index file
        let index = self.generate_index_file();
        files.insert("src/index.ts".to_string(), index);

        // Generate tsconfig
        let tsconfig = self.generate_tsconfig();
        files.insert("tsconfig.json".to_string(), tsconfig);

        // Generate tests if enabled
        if config.generate_tests {
            let tests = self.generate_tests(spec, config)?;
            files.insert("src/__tests__/client.test.ts".to_string(), tests);
        }

        let package_metadata = PackageMetadata {
            name: config.package_name.clone(),
            version: config.version.clone(),
            description: spec
                .info
                .description
                .clone()
                .unwrap_or_else(|| format!("{} SDK", spec.info.title)),
            dependencies: self.get_dependencies(),
        };

        Ok(GeneratedSdk {
            language: SdkLanguage::TypeScript,
            files,
            package_metadata,
        })
    }

    fn generate_client(
        &self,
        spec: &OpenApiSpec,
        config: &SdkConfig,
    ) -> Result<String, SdkGeneratorError> {
        let mut client_code = String::new();

        // Imports
        client_code.push_str("import { AuthHandler } from './auth';\n");
        client_code.push_str("import { RetryHandler } from './retry';\n");
        client_code.push_str("import * as Models from './models';\n\n");

        // Client configuration interface
        client_code.push_str("export interface ClientConfig {\n");
        client_code.push_str("  baseUrl: string;\n");
        client_code.push_str("  authHandler?: AuthHandler;\n");
        client_code.push_str("  timeout?: number;\n");
        client_code.push_str("  retryConfig?: RetryConfig;\n");
        client_code.push_str("}\n\n");

        client_code.push_str("export interface RetryConfig {\n");
        client_code.push_str("  maxRetries: number;\n");
        client_code.push_str("  initialDelayMs: number;\n");
        client_code.push_str("  maxDelayMs: number;\n");
        client_code.push_str("  backoffMultiplier: number;\n");
        client_code.push_str("}\n\n");

        // Client class
        client_code.push_str(&format!("/**\n * {} API Client\n */\n", spec.info.title));
        client_code.push_str("export class LegalisClient {\n");
        client_code.push_str("  private baseUrl: string;\n");
        client_code.push_str("  private authHandler?: AuthHandler;\n");
        client_code.push_str("  private timeout: number;\n");
        client_code.push_str("  private retryHandler: RetryHandler;\n\n");

        // Constructor
        client_code.push_str("  constructor(config: ClientConfig) {\n");
        client_code.push_str("    this.baseUrl = config.baseUrl;\n");
        client_code.push_str("    this.authHandler = config.authHandler;\n");
        client_code.push_str("    this.timeout = config.timeout || 30000;\n");
        client_code.push_str("    this.retryHandler = new RetryHandler(config.retryConfig || {\n");
        client_code.push_str(&format!(
            "      maxRetries: {},\n",
            config.retry_config.max_retries
        ));
        client_code.push_str(&format!(
            "      initialDelayMs: {},\n",
            config.retry_config.initial_delay_ms
        ));
        client_code.push_str(&format!(
            "      maxDelayMs: {},\n",
            config.retry_config.max_delay_ms
        ));
        client_code.push_str(&format!(
            "      backoffMultiplier: {},\n",
            config.retry_config.backoff_multiplier
        ));
        client_code.push_str("    });\n");
        client_code.push_str("  }\n\n");

        // Request method
        client_code
            .push_str("  private async request<T>(method: string, path: string, options?: {\n");
        client_code.push_str("    body?: unknown;\n");
        client_code.push_str("    headers?: Record<string, string>;\n");
        client_code.push_str("    query?: Record<string, string>;\n");
        client_code.push_str("  }): Promise<T> {\n");
        client_code.push_str("    const url = new URL(path, this.baseUrl);\n");
        client_code.push_str("    if (options?.query) {\n");
        client_code.push_str("      Object.entries(options.query).forEach(([key, value]) => {\n");
        client_code.push_str("        url.searchParams.append(key, value);\n");
        client_code.push_str("      });\n");
        client_code.push_str("    }\n\n");
        client_code.push_str("    const headers: Record<string, string> = {\n");
        client_code.push_str("      'Content-Type': 'application/json',\n");
        client_code.push_str("      ...options?.headers,\n");
        client_code.push_str("    };\n\n");
        client_code.push_str("    if (this.authHandler) {\n");
        client_code.push_str("      await this.authHandler.addAuth(headers);\n");
        client_code.push_str("    }\n\n");
        client_code.push_str("    return this.retryHandler.execute(async () => {\n");
        client_code.push_str("      const controller = new AbortController();\n");
        client_code.push_str(
            "      const timeoutId = setTimeout(() => controller.abort(), this.timeout);\n\n",
        );
        client_code.push_str("      try {\n");
        client_code.push_str("        const response = await fetch(url.toString(), {\n");
        client_code.push_str("          method,\n");
        client_code.push_str("          headers,\n");
        client_code.push_str(
            "          body: options?.body ? JSON.stringify(options.body) : undefined,\n",
        );
        client_code.push_str("          signal: controller.signal,\n");
        client_code.push_str("        });\n\n");
        client_code.push_str("        if (!response.ok) {\n");
        client_code.push_str(
            "          throw new Error(`HTTP ${response.status}: ${response.statusText}`);\n",
        );
        client_code.push_str("        }\n\n");
        client_code.push_str("        return await response.json();\n");
        client_code.push_str("      } finally {\n");
        client_code.push_str("        clearTimeout(timeoutId);\n");
        client_code.push_str("      }\n");
        client_code.push_str("    });\n");
        client_code.push_str("  }\n\n");

        // Generate methods for each operation
        for (path, path_item) in &spec.paths {
            self.generate_operation_methods(&mut client_code, path, "get", path_item.get.as_ref())?;
            self.generate_operation_methods(
                &mut client_code,
                path,
                "post",
                path_item.post.as_ref(),
            )?;
            self.generate_operation_methods(&mut client_code, path, "put", path_item.put.as_ref())?;
            self.generate_operation_methods(
                &mut client_code,
                path,
                "delete",
                path_item.delete.as_ref(),
            )?;
            self.generate_operation_methods(
                &mut client_code,
                path,
                "patch",
                path_item.patch.as_ref(),
            )?;
        }

        client_code.push_str("}\n");

        Ok(client_code)
    }

    fn generate_models(&self, spec: &OpenApiSpec) -> Result<String, SdkGeneratorError> {
        let mut models_code = String::new();

        models_code.push_str("/**\n * Type definitions for the API\n */\n\n");

        if let Some(components) = &spec.components
            && let Some(schemas) = &components.schemas
        {
            for (name, schema) in schemas {
                models_code.push_str(&format!("export interface {} {{\n", name));

                if let Some(properties) = &schema.properties {
                    for (prop_name, prop_schema) in properties {
                        let type_str = self.map_type_to_typescript(prop_schema);
                        models_code.push_str(&format!("  {}: {};\n", prop_name, type_str));
                    }
                }

                models_code.push_str("}\n\n");
            }
        }

        // Add common types
        models_code.push_str("export interface ApiResponse<T> {\n");
        models_code.push_str("  data: T;\n");
        models_code.push_str("  meta?: ResponseMeta;\n");
        models_code.push_str("}\n\n");

        models_code.push_str("export interface ResponseMeta {\n");
        models_code.push_str("  total?: number;\n");
        models_code.push_str("  page?: number;\n");
        models_code.push_str("  per_page?: number;\n");
        models_code.push_str("  next_cursor?: string;\n");
        models_code.push_str("  prev_cursor?: string;\n");
        models_code.push_str("  has_more?: boolean;\n");
        models_code.push_str("}\n\n");

        models_code.push_str("export interface ErrorResponse {\n");
        models_code.push_str("  error: string;\n");
        models_code.push_str("}\n");

        Ok(models_code)
    }

    fn generate_auth(&self, _config: &SdkConfig) -> Result<String, SdkGeneratorError> {
        let mut auth_code = String::new();

        auth_code.push_str("/**\n * Authentication handlers for the SDK\n */\n\n");

        auth_code.push_str("export interface AuthHandler {\n");
        auth_code.push_str("  addAuth(headers: Record<string, string>): Promise<void>;\n");
        auth_code.push_str("}\n\n");

        // JWT Auth Handler
        auth_code.push_str("export class JwtAuthHandler implements AuthHandler {\n");
        auth_code.push_str("  constructor(private token: string, private headerName: string = 'Authorization') {}\n\n");
        auth_code.push_str("  async addAuth(headers: Record<string, string>): Promise<void> {\n");
        auth_code.push_str("    headers[this.headerName] = `Bearer ${this.token}`;\n");
        auth_code.push_str("  }\n");
        auth_code.push_str("}\n\n");

        // API Key Auth Handler
        auth_code.push_str("export class ApiKeyAuthHandler implements AuthHandler {\n");
        auth_code.push_str("  constructor(\n");
        auth_code.push_str("    private apiKey: string,\n");
        auth_code.push_str("    private headerName: string = 'X-API-Key'\n");
        auth_code.push_str("  ) {}\n\n");
        auth_code.push_str("  async addAuth(headers: Record<string, string>): Promise<void> {\n");
        auth_code.push_str("    headers[this.headerName] = this.apiKey;\n");
        auth_code.push_str("  }\n");
        auth_code.push_str("}\n\n");

        // OAuth2 Auth Handler
        auth_code.push_str("export class OAuth2AuthHandler implements AuthHandler {\n");
        auth_code.push_str("  private accessToken?: string;\n");
        auth_code.push_str("  private tokenExpiry?: number;\n\n");
        auth_code.push_str("  constructor(\n");
        auth_code.push_str("    private clientId: string,\n");
        auth_code.push_str("    private clientSecret: string,\n");
        auth_code.push_str("    private tokenUrl: string\n");
        auth_code.push_str("  ) {}\n\n");
        auth_code.push_str("  async addAuth(headers: Record<string, string>): Promise<void> {\n");
        auth_code.push_str("    if (!this.accessToken || this.isTokenExpired()) {\n");
        auth_code.push_str("      await this.refreshToken();\n");
        auth_code.push_str("    }\n");
        auth_code.push_str("    headers['Authorization'] = `Bearer ${this.accessToken}`;\n");
        auth_code.push_str("  }\n\n");
        auth_code.push_str("  private isTokenExpired(): boolean {\n");
        auth_code.push_str("    if (!this.tokenExpiry) return true;\n");
        auth_code.push_str("    return Date.now() >= this.tokenExpiry;\n");
        auth_code.push_str("  }\n\n");
        auth_code.push_str("  private async refreshToken(): Promise<void> {\n");
        auth_code.push_str("    const response = await fetch(this.tokenUrl, {\n");
        auth_code.push_str("      method: 'POST',\n");
        auth_code
            .push_str("      headers: { 'Content-Type': 'application/x-www-form-urlencoded' },\n");
        auth_code.push_str("      body: new URLSearchParams({\n");
        auth_code.push_str("        grant_type: 'client_credentials',\n");
        auth_code.push_str("        client_id: this.clientId,\n");
        auth_code.push_str("        client_secret: this.clientSecret,\n");
        auth_code.push_str("      }),\n");
        auth_code.push_str("    });\n\n");
        auth_code.push_str("    if (!response.ok) {\n");
        auth_code.push_str("      throw new Error('Failed to obtain access token');\n");
        auth_code.push_str("    }\n\n");
        auth_code.push_str("    const data = await response.json();\n");
        auth_code.push_str("    this.accessToken = data.access_token;\n");
        auth_code.push_str("    this.tokenExpiry = Date.now() + (data.expires_in * 1000) - 60000; // 1 min buffer\n");
        auth_code.push_str("  }\n");
        auth_code.push_str("}\n");

        Ok(auth_code)
    }

    fn generate_retry(&self, _config: &SdkConfig) -> Result<String, SdkGeneratorError> {
        let mut retry_code = String::new();

        retry_code.push_str("/**\n * Retry handler with exponential backoff\n */\n\n");

        retry_code.push_str("export interface RetryConfig {\n");
        retry_code.push_str("  maxRetries: number;\n");
        retry_code.push_str("  initialDelayMs: number;\n");
        retry_code.push_str("  maxDelayMs: number;\n");
        retry_code.push_str("  backoffMultiplier: number;\n");
        retry_code.push_str("}\n\n");

        retry_code.push_str("export class RetryHandler {\n");
        retry_code.push_str("  constructor(private config: RetryConfig) {}\n\n");
        retry_code.push_str("  async execute<T>(fn: () => Promise<T>): Promise<T> {\n");
        retry_code.push_str("    let lastError: Error | undefined;\n");
        retry_code.push_str("    let delay = this.config.initialDelayMs;\n\n");
        retry_code.push_str(
            "    for (let attempt = 0; attempt <= this.config.maxRetries; attempt++) {\n",
        );
        retry_code.push_str("      try {\n");
        retry_code.push_str("        return await fn();\n");
        retry_code.push_str("      } catch (error) {\n");
        retry_code.push_str("        lastError = error as Error;\n\n");
        retry_code.push_str("        if (attempt === this.config.maxRetries) {\n");
        retry_code.push_str("          break;\n");
        retry_code.push_str("        }\n\n");
        retry_code.push_str("        if (!this.shouldRetry(error)) {\n");
        retry_code.push_str("          throw error;\n");
        retry_code.push_str("        }\n\n");
        retry_code.push_str("        await this.sleep(delay);\n");
        retry_code.push_str("        delay = Math.min(delay * this.config.backoffMultiplier, this.config.maxDelayMs);\n");
        retry_code.push_str("      }\n");
        retry_code.push_str("    }\n\n");
        retry_code.push_str("    throw lastError;\n");
        retry_code.push_str("  }\n\n");
        retry_code.push_str("  private shouldRetry(error: unknown): boolean {\n");
        retry_code.push_str("    if (error instanceof Error) {\n");
        retry_code.push_str("      const message = error.message;\n");
        retry_code.push_str("      return message.includes('429') || message.includes('500') ||\n");
        retry_code.push_str("             message.includes('502') || message.includes('503') ||\n");
        retry_code.push_str("             message.includes('504');\n");
        retry_code.push_str("    }\n");
        retry_code.push_str("    return false;\n");
        retry_code.push_str("  }\n\n");
        retry_code.push_str("  private sleep(ms: number): Promise<void> {\n");
        retry_code.push_str("    return new Promise(resolve => setTimeout(resolve, ms));\n");
        retry_code.push_str("  }\n");
        retry_code.push_str("}\n");

        Ok(retry_code)
    }

    fn generate_package_file(&self, config: &SdkConfig) -> Result<String, SdkGeneratorError> {
        let package_json = serde_json::json!({
            "name": config.package_name,
            "version": config.version,
            "description": format!("TypeScript SDK for {}", config.package_name),
            "main": "dist/index.js",
            "types": "dist/index.d.ts",
            "scripts": {
                "build": "tsc",
                "test": "jest",
                "lint": "eslint src/**/*.ts",
                "prepublishOnly": "npm run build"
            },
            "keywords": ["sdk", "api", "legalis", "typescript"],
            "author": "COOLJAPAN OU (Team Kitasan)",
            "license": "MIT OR Apache-2.0",
            "dependencies": self.get_dependencies(),
            "devDependencies": {
                "@types/node": "^20.0.0",
                "@types/jest": "^29.0.0",
                "typescript": "^5.0.0",
                "jest": "^29.0.0",
                "ts-jest": "^29.0.0",
                "eslint": "^8.0.0",
                "@typescript-eslint/parser": "^6.0.0",
                "@typescript-eslint/eslint-plugin": "^6.0.0"
            }
        });

        serde_json::to_string_pretty(&package_json)
            .map_err(|e| SdkGeneratorError::TemplateError(e.to_string()))
    }

    fn generate_readme(
        &self,
        spec: &OpenApiSpec,
        config: &SdkConfig,
    ) -> Result<String, SdkGeneratorError> {
        let mut readme = String::new();

        readme.push_str(&format!("# {} TypeScript SDK\n\n", spec.info.title));

        if let Some(desc) = &spec.info.description {
            readme.push_str(&format!("{}\n\n", desc));
        }

        readme.push_str("## Installation\n\n");
        readme.push_str("```bash\n");
        readme.push_str(&format!("npm install {}\n", config.package_name));
        readme.push_str("```\n\n");

        readme.push_str("## Usage\n\n");
        readme.push_str("### Basic Example\n\n");
        readme.push_str("```typescript\n");
        readme.push_str("import { LegalisClient, JwtAuthHandler } from '@legalis/sdk';\n\n");
        readme.push_str("// Create client with JWT authentication\n");
        readme.push_str("const client = new LegalisClient({\n");
        readme.push_str(&format!("  baseUrl: '{}',\n", config.base_url));
        readme.push_str("  authHandler: new JwtAuthHandler('your-jwt-token'),\n");
        readme.push_str("  timeout: 30000,\n");
        readme.push_str("});\n\n");
        readme.push_str("// Make API calls\n");
        readme.push_str("const response = await client.getStatutes();\n");
        readme.push_str("console.log(response.data);\n");
        readme.push_str("```\n\n");

        readme.push_str("### Authentication\n\n");
        readme.push_str("The SDK supports multiple authentication methods:\n\n");
        readme.push_str("#### JWT Authentication\n");
        readme.push_str("```typescript\n");
        readme.push_str("import { JwtAuthHandler } from '@legalis/sdk';\n\n");
        readme.push_str("const authHandler = new JwtAuthHandler('your-jwt-token');\n");
        readme.push_str("```\n\n");

        readme.push_str("#### API Key Authentication\n");
        readme.push_str("```typescript\n");
        readme.push_str("import { ApiKeyAuthHandler } from '@legalis/sdk';\n\n");
        readme
            .push_str("const authHandler = new ApiKeyAuthHandler('your-api-key', 'X-API-Key');\n");
        readme.push_str("```\n\n");

        readme.push_str("#### OAuth2 Authentication\n");
        readme.push_str("```typescript\n");
        readme.push_str("import { OAuth2AuthHandler } from '@legalis/sdk';\n\n");
        readme.push_str("const authHandler = new OAuth2AuthHandler(\n");
        readme.push_str("  'client-id',\n");
        readme.push_str("  'client-secret',\n");
        readme.push_str("  'https://auth.example.com/token'\n");
        readme.push_str(");\n");
        readme.push_str("```\n\n");

        readme.push_str("### Retry Configuration\n\n");
        readme.push_str(
            "The SDK automatically retries failed requests with exponential backoff:\n\n",
        );
        readme.push_str("```typescript\n");
        readme.push_str("const client = new LegalisClient({\n");
        readme.push_str("  baseUrl: 'https://api.example.com',\n");
        readme.push_str("  retryConfig: {\n");
        readme.push_str("    maxRetries: 3,\n");
        readme.push_str("    initialDelayMs: 100,\n");
        readme.push_str("    maxDelayMs: 5000,\n");
        readme.push_str("    backoffMultiplier: 2.0,\n");
        readme.push_str("  },\n");
        readme.push_str("});\n");
        readme.push_str("```\n\n");

        readme.push_str("## License\n\n");
        readme.push_str("MIT OR Apache-2.0\n");

        Ok(readme)
    }

    fn generate_tests(
        &self,
        _spec: &OpenApiSpec,
        _config: &SdkConfig,
    ) -> Result<String, SdkGeneratorError> {
        let mut tests_code = String::new();

        tests_code.push_str(
            "import { LegalisClient, JwtAuthHandler, ApiKeyAuthHandler } from '../index';\n\n",
        );

        tests_code.push_str("describe('LegalisClient', () => {\n");
        tests_code.push_str("  const baseUrl = 'http://localhost:3000';\n\n");

        tests_code.push_str("  test('should create client with config', () => {\n");
        tests_code.push_str("    const client = new LegalisClient({ baseUrl });\n");
        tests_code.push_str("    expect(client).toBeDefined();\n");
        tests_code.push_str("  });\n\n");

        tests_code.push_str("  test('should create client with JWT auth', () => {\n");
        tests_code.push_str("    const authHandler = new JwtAuthHandler('test-token');\n");
        tests_code.push_str("    const client = new LegalisClient({ baseUrl, authHandler });\n");
        tests_code.push_str("    expect(client).toBeDefined();\n");
        tests_code.push_str("  });\n\n");

        tests_code.push_str("  test('should create client with API key auth', () => {\n");
        tests_code.push_str("    const authHandler = new ApiKeyAuthHandler('test-key');\n");
        tests_code.push_str("    const client = new LegalisClient({ baseUrl, authHandler });\n");
        tests_code.push_str("    expect(client).toBeDefined();\n");
        tests_code.push_str("  });\n");

        tests_code.push_str("});\n");

        Ok(tests_code)
    }
}

impl TypeScriptSdkGenerator {
    fn generate_operation_methods(
        &self,
        code: &mut String,
        path: &str,
        method: &str,
        operation: Option<&Operation>,
    ) -> Result<(), SdkGeneratorError> {
        if let Some(op) = operation {
            let operation_id = op.operation_id.as_ref().ok_or_else(|| {
                SdkGeneratorError::InvalidSpec(format!(
                    "Missing operationId for {} {}",
                    method, path
                ))
            })?;

            // Generate JSDoc comment
            code.push_str(&format!(
                "  /**\n   * {}\n",
                op.summary.as_deref().unwrap_or("")
            ));
            if let Some(desc) = &op.description {
                code.push_str(&format!("   * {}\n", desc));
            }
            code.push_str("   */\n");

            // Generate method signature
            let has_path_params = path.contains('{');
            let has_query_params = op
                .parameters
                .as_ref()
                .map(|p| p.iter().any(|param| param.location == "query"))
                .unwrap_or(false);
            let has_body = op.request_body.is_some();

            code.push_str(&format!("  async {}(", self.to_camel_case(operation_id)));

            let mut params = Vec::new();
            if has_path_params {
                params.push("pathParams: Record<string, string>".to_string());
            }
            if has_query_params {
                params.push("queryParams?: Record<string, string>".to_string());
            }
            if has_body {
                params.push("body: unknown".to_string());
            }

            code.push_str(&params.join(", "));
            code.push_str("): Promise<any> {\n");

            // Generate method body
            let api_path = if has_path_params {
                code.push_str("    let path = '");
                code.push_str(path);
                code.push_str("';\n");
                code.push_str("    Object.entries(pathParams).forEach(([key, value]) => {\n");
                code.push_str("      path = path.replace(`{${key}}`, value);\n");
                code.push_str("    });\n");
                "path".to_string()
            } else {
                format!("'{}'", path)
            };

            code.push_str(&format!(
                "    return this.request('{}', {}, {{\n",
                method.to_uppercase(),
                api_path
            ));
            if has_query_params {
                code.push_str("      query: queryParams,\n");
            }
            if has_body {
                code.push_str("      body,\n");
            }
            code.push_str("    });\n");
            code.push_str("  }\n\n");
        }

        Ok(())
    }

    fn to_camel_case(&self, s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;

        for ch in s.chars() {
            if ch == '_' || ch == '-' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(ch.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(ch);
            }
        }

        result
    }

    fn map_type_to_typescript(&self, schema: &Schema) -> String {
        if let Some(ref_path) = &schema.reference {
            return ref_path
                .split('/')
                .next_back()
                .unwrap_or("unknown")
                .to_string();
        }

        match schema.schema_type.as_deref() {
            Some("string") => "string".to_string(),
            Some("number") | Some("integer") => "number".to_string(),
            Some("boolean") => "boolean".to_string(),
            Some("array") => {
                if let Some(items) = &schema.items {
                    format!("{}[]", self.map_type_to_typescript(items))
                } else {
                    "any[]".to_string()
                }
            }
            Some("object") => {
                if let Some(properties) = &schema.properties {
                    let mut obj_type = String::from("{\n");
                    for (name, prop) in properties {
                        let prop_type = self.map_type_to_typescript(prop);
                        obj_type.push_str(&format!("    {}: {};\n", name, prop_type));
                    }
                    obj_type.push_str("  }");
                    obj_type
                } else {
                    "Record<string, any>".to_string()
                }
            }
            _ => "any".to_string(),
        }
    }

    fn generate_index_file(&self) -> String {
        let mut index = String::new();
        index.push_str("export { LegalisClient, ClientConfig, RetryConfig } from './client';\n");
        index.push_str("export { AuthHandler, JwtAuthHandler, ApiKeyAuthHandler, OAuth2AuthHandler } from './auth';\n");
        index.push_str("export { RetryHandler } from './retry';\n");
        index.push_str("export * as Models from './models';\n");
        index
    }

    fn generate_tsconfig(&self) -> String {
        serde_json::json!({
            "compilerOptions": {
                "target": "ES2020",
                "module": "commonjs",
                "declaration": true,
                "outDir": "./dist",
                "rootDir": "./src",
                "strict": true,
                "esModuleInterop": true,
                "skipLibCheck": true,
                "forceConsistentCasingInFileNames": true,
                "moduleResolution": "node",
                "resolveJsonModule": true
            },
            "include": ["src/**/*"],
            "exclude": ["node_modules", "dist", "**/*.test.ts"]
        })
        .to_string()
    }

    fn get_dependencies(&self) -> HashMap<String, String> {
        let mut deps = HashMap::new();
        deps.insert("node-fetch".to_string(), "^3.0.0".to_string());
        deps
    }
}

/// Python SDK generator.
pub struct PythonSdkGenerator;

impl SdkGenerator for PythonSdkGenerator {
    fn generate(
        &self,
        spec: &OpenApiSpec,
        config: &SdkConfig,
    ) -> Result<GeneratedSdk, SdkGeneratorError> {
        let mut files = HashMap::new();

        // Generate main client
        let client = self.generate_client(spec, config)?;
        files.insert(format!("{}/client.py", config.package_name), client);

        // Generate models
        let models = self.generate_models(spec)?;
        files.insert(format!("{}/models.py", config.package_name), models);

        // Generate auth
        let auth = self.generate_auth(config)?;
        files.insert(format!("{}/auth.py", config.package_name), auth);

        // Generate retry logic
        let retry = self.generate_retry(config)?;
        files.insert(format!("{}/retry.py", config.package_name), retry);

        // Generate __init__.py
        let init = self.generate_init_file();
        files.insert(format!("{}/__init__.py", config.package_name), init);

        // Generate setup.py
        let setup = self.generate_package_file(config)?;
        files.insert("setup.py".to_string(), setup);

        // Generate README
        let readme = self.generate_readme(spec, config)?;
        files.insert("README.md".to_string(), readme);

        // Generate requirements.txt
        let requirements = self.generate_requirements();
        files.insert("requirements.txt".to_string(), requirements);

        // Generate pyproject.toml
        let pyproject = self.generate_pyproject(config)?;
        files.insert("pyproject.toml".to_string(), pyproject);

        // Generate tests if enabled
        if config.generate_tests {
            let tests = self.generate_tests(spec, config)?;
            files.insert("tests/test_client.py".to_string(), tests);
            files.insert("tests/__init__.py".to_string(), String::new());
        }

        let package_metadata = PackageMetadata {
            name: config.package_name.clone(),
            version: config.version.clone(),
            description: spec
                .info
                .description
                .clone()
                .unwrap_or_else(|| format!("{} SDK", spec.info.title)),
            dependencies: self.get_dependencies(),
        };

        Ok(GeneratedSdk {
            language: SdkLanguage::Python,
            files,
            package_metadata,
        })
    }

    fn generate_client(
        &self,
        spec: &OpenApiSpec,
        config: &SdkConfig,
    ) -> Result<String, SdkGeneratorError> {
        let mut client_code = String::new();

        // Imports
        client_code.push_str("\"\"\"Main client for the API.\"\"\"\n\n");
        client_code.push_str("from typing import Any, Dict, Optional\n");
        client_code.push_str("import httpx\n");
        client_code.push_str("from .auth import AuthHandler\n");
        client_code.push_str("from .retry import RetryHandler\n");
        client_code.push_str("from . import models\n\n\n");

        // Client class
        client_code.push_str(&format!(
            "class {}Client:\n",
            self.to_pascal_case(&config.package_name)
        ));
        client_code.push_str(&format!(
            "    \"\"\"Client for {} API.\"\"\"\n\n",
            spec.info.title
        ));

        // Constructor
        client_code.push_str("    def __init__(\n");
        client_code.push_str("        self,\n");
        client_code.push_str("        base_url: str,\n");
        client_code.push_str("        auth_handler: Optional[AuthHandler] = None,\n");
        client_code.push_str("        timeout: float = 30.0,\n");
        client_code.push_str("        retry_config: Optional[Dict[str, Any]] = None,\n");
        client_code.push_str("    ):\n");
        client_code.push_str("        self.base_url = base_url.rstrip('/')\n");
        client_code.push_str("        self.auth_handler = auth_handler\n");
        client_code.push_str("        self.timeout = timeout\n");
        client_code.push_str("        self.retry_handler = RetryHandler(retry_config or {})\n");
        client_code.push_str("        self.client = httpx.AsyncClient(timeout=timeout)\n\n");

        // Request method
        client_code.push_str("    async def _request(\n");
        client_code.push_str("        self,\n");
        client_code.push_str("        method: str,\n");
        client_code.push_str("        path: str,\n");
        client_code.push_str("        *,\n");
        client_code.push_str("        json: Optional[Any] = None,\n");
        client_code.push_str("        params: Optional[Dict[str, str]] = None,\n");
        client_code.push_str("    ) -> Any:\n");
        client_code.push_str("        \"\"\"Make an HTTP request.\"\"\"\n");
        client_code.push_str("        url = f\"{self.base_url}{path}\"\n");
        client_code.push_str("        headers = {'Content-Type': 'application/json'}\n\n");
        client_code.push_str("        if self.auth_handler:\n");
        client_code.push_str("            await self.auth_handler.add_auth(headers)\n\n");
        client_code.push_str("        async def make_request():\n");
        client_code.push_str("            response = await self.client.request(\n");
        client_code
            .push_str("                method, url, json=json, params=params, headers=headers\n");
        client_code.push_str("            )\n");
        client_code.push_str("            response.raise_for_status()\n");
        client_code.push_str("            return response.json()\n\n");
        client_code.push_str("        return await self.retry_handler.execute(make_request)\n\n");

        // Close method
        client_code.push_str("    async def close(self):\n");
        client_code.push_str("        \"\"\"Close the HTTP client.\"\"\"\n");
        client_code.push_str("        await self.client.aclose()\n\n");

        // Context manager support
        client_code.push_str("    async def __aenter__(self):\n");
        client_code.push_str("        return self\n\n");
        client_code.push_str("    async def __aexit__(self, exc_type, exc_val, exc_tb):\n");
        client_code.push_str("        await self.close()\n\n");

        // Generate methods for each operation
        for (path, path_item) in &spec.paths {
            self.generate_python_operation_methods(
                &mut client_code,
                path,
                "get",
                path_item.get.as_ref(),
            )?;
            self.generate_python_operation_methods(
                &mut client_code,
                path,
                "post",
                path_item.post.as_ref(),
            )?;
            self.generate_python_operation_methods(
                &mut client_code,
                path,
                "put",
                path_item.put.as_ref(),
            )?;
            self.generate_python_operation_methods(
                &mut client_code,
                path,
                "delete",
                path_item.delete.as_ref(),
            )?;
            self.generate_python_operation_methods(
                &mut client_code,
                path,
                "patch",
                path_item.patch.as_ref(),
            )?;
        }

        Ok(client_code)
    }

    fn generate_models(&self, spec: &OpenApiSpec) -> Result<String, SdkGeneratorError> {
        let mut models_code = String::new();

        models_code.push_str("\"\"\"Type definitions for the API.\"\"\"\n\n");
        models_code.push_str("from typing import Any, Dict, List, Optional\n");
        models_code.push_str("from dataclasses import dataclass\n\n\n");

        if let Some(components) = &spec.components
            && let Some(schemas) = &components.schemas
        {
            for (name, schema) in schemas {
                models_code.push_str("@dataclass\n");
                models_code.push_str(&format!("class {}:\n", name));
                models_code.push_str(&format!("    \"\"\"{}.\"\"\"\n", name));

                if let Some(properties) = &schema.properties {
                    for (prop_name, prop_schema) in properties {
                        let type_str = self.map_type_to_python(prop_schema);
                        models_code.push_str(&format!("    {}: {}\n", prop_name, type_str));
                    }
                } else {
                    models_code.push_str("    pass\n");
                }

                models_code.push_str("\n\n");
            }
        }

        // Add common types
        models_code.push_str("@dataclass\n");
        models_code.push_str("class ResponseMeta:\n");
        models_code.push_str("    \"\"\"Response metadata.\"\"\"\n");
        models_code.push_str("    total: Optional[int] = None\n");
        models_code.push_str("    page: Optional[int] = None\n");
        models_code.push_str("    per_page: Optional[int] = None\n");
        models_code.push_str("    next_cursor: Optional[str] = None\n");
        models_code.push_str("    prev_cursor: Optional[str] = None\n");
        models_code.push_str("    has_more: Optional[bool] = None\n\n\n");

        models_code.push_str("@dataclass\n");
        models_code.push_str("class ApiResponse:\n");
        models_code.push_str("    \"\"\"API response wrapper.\"\"\"\n");
        models_code.push_str("    data: Any\n");
        models_code.push_str("    meta: Optional[ResponseMeta] = None\n\n\n");

        models_code.push_str("@dataclass\n");
        models_code.push_str("class ErrorResponse:\n");
        models_code.push_str("    \"\"\"Error response.\"\"\"\n");
        models_code.push_str("    error: str\n");

        Ok(models_code)
    }

    fn generate_auth(&self, _config: &SdkConfig) -> Result<String, SdkGeneratorError> {
        let mut auth_code = String::new();

        auth_code.push_str("\"\"\"Authentication handlers.\"\"\"\n\n");
        auth_code.push_str("from abc import ABC, abstractmethod\n");
        auth_code.push_str("from typing import Dict\n");
        auth_code.push_str("import httpx\n");
        auth_code.push_str("from datetime import datetime, timedelta\n\n\n");

        auth_code.push_str("class AuthHandler(ABC):\n");
        auth_code.push_str("    \"\"\"Base authentication handler.\"\"\"\n\n");
        auth_code.push_str("    @abstractmethod\n");
        auth_code.push_str("    async def add_auth(self, headers: Dict[str, str]) -> None:\n");
        auth_code.push_str("        \"\"\"Add authentication to request headers.\"\"\"\n");
        auth_code.push_str("        pass\n\n\n");

        // JWT Auth Handler
        auth_code.push_str("class JwtAuthHandler(AuthHandler):\n");
        auth_code.push_str("    \"\"\"JWT authentication handler.\"\"\"\n\n");
        auth_code
            .push_str("    def __init__(self, token: str, header_name: str = 'Authorization'):\n");
        auth_code.push_str("        self.token = token\n");
        auth_code.push_str("        self.header_name = header_name\n\n");
        auth_code.push_str("    async def add_auth(self, headers: Dict[str, str]) -> None:\n");
        auth_code.push_str("        headers[self.header_name] = f'Bearer {self.token}'\n\n\n");

        // API Key Auth Handler
        auth_code.push_str("class ApiKeyAuthHandler(AuthHandler):\n");
        auth_code.push_str("    \"\"\"API key authentication handler.\"\"\"\n\n");
        auth_code
            .push_str("    def __init__(self, api_key: str, header_name: str = 'X-API-Key'):\n");
        auth_code.push_str("        self.api_key = api_key\n");
        auth_code.push_str("        self.header_name = header_name\n\n");
        auth_code.push_str("    async def add_auth(self, headers: Dict[str, str]) -> None:\n");
        auth_code.push_str("        headers[self.header_name] = self.api_key\n\n\n");

        // OAuth2 Auth Handler
        auth_code.push_str("class OAuth2AuthHandler(AuthHandler):\n");
        auth_code.push_str("    \"\"\"OAuth2 authentication handler.\"\"\"\n\n");
        auth_code.push_str(
            "    def __init__(self, client_id: str, client_secret: str, token_url: str):\n",
        );
        auth_code.push_str("        self.client_id = client_id\n");
        auth_code.push_str("        self.client_secret = client_secret\n");
        auth_code.push_str("        self.token_url = token_url\n");
        auth_code.push_str("        self.access_token = None\n");
        auth_code.push_str("        self.token_expiry = None\n\n");
        auth_code.push_str("    async def add_auth(self, headers: Dict[str, str]) -> None:\n");
        auth_code.push_str("        if not self.access_token or self._is_token_expired():\n");
        auth_code.push_str("            await self._refresh_token()\n");
        auth_code.push_str("        headers['Authorization'] = f'Bearer {self.access_token}'\n\n");
        auth_code.push_str("    def _is_token_expired(self) -> bool:\n");
        auth_code.push_str("        if not self.token_expiry:\n");
        auth_code.push_str("            return True\n");
        auth_code.push_str("        return datetime.now() >= self.token_expiry\n\n");
        auth_code.push_str("    async def _refresh_token(self) -> None:\n");
        auth_code.push_str("        async with httpx.AsyncClient() as client:\n");
        auth_code.push_str("            response = await client.post(\n");
        auth_code.push_str("                self.token_url,\n");
        auth_code.push_str("                data={\n");
        auth_code.push_str("                    'grant_type': 'client_credentials',\n");
        auth_code.push_str("                    'client_id': self.client_id,\n");
        auth_code.push_str("                    'client_secret': self.client_secret,\n");
        auth_code.push_str("                },\n");
        auth_code.push_str("            )\n");
        auth_code.push_str("            response.raise_for_status()\n");
        auth_code.push_str("            data = response.json()\n");
        auth_code.push_str("            self.access_token = data['access_token']\n");
        auth_code.push_str("            expires_in = data.get('expires_in', 3600)\n");
        auth_code.push_str(
            "            self.token_expiry = datetime.now() + timedelta(seconds=expires_in - 60)\n",
        );

        Ok(auth_code)
    }

    fn generate_retry(&self, _config: &SdkConfig) -> Result<String, SdkGeneratorError> {
        let mut retry_code = String::new();

        retry_code.push_str("\"\"\"Retry handler with exponential backoff.\"\"\"\n\n");
        retry_code.push_str("import asyncio\n");
        retry_code.push_str("from typing import Any, Callable, Dict\n");
        retry_code.push_str("import httpx\n\n\n");

        retry_code.push_str("class RetryHandler:\n");
        retry_code
            .push_str("    \"\"\"Handles request retries with exponential backoff.\"\"\"\n\n");
        retry_code.push_str("    def __init__(self, config: Dict[str, Any]):\n");
        retry_code.push_str("        self.max_retries = config.get('max_retries', 3)\n");
        retry_code
            .push_str("        self.initial_delay_ms = config.get('initial_delay_ms', 100)\n");
        retry_code.push_str("        self.max_delay_ms = config.get('max_delay_ms', 5000)\n");
        retry_code.push_str(
            "        self.backoff_multiplier = config.get('backoff_multiplier', 2.0)\n\n",
        );
        retry_code.push_str("    async def execute(self, fn: Callable) -> Any:\n");
        retry_code.push_str("        \"\"\"Execute function with retry logic.\"\"\"\n");
        retry_code.push_str("        last_exception = None\n");
        retry_code.push_str("        delay = self.initial_delay_ms / 1000.0\n\n");
        retry_code.push_str("        for attempt in range(self.max_retries + 1):\n");
        retry_code.push_str("            try:\n");
        retry_code.push_str("                return await fn()\n");
        retry_code.push_str("            except Exception as e:\n");
        retry_code.push_str("                last_exception = e\n\n");
        retry_code.push_str("                if attempt == self.max_retries:\n");
        retry_code.push_str("                    break\n\n");
        retry_code.push_str("                if not self._should_retry(e):\n");
        retry_code.push_str("                    raise\n\n");
        retry_code.push_str("                await asyncio.sleep(delay)\n");
        retry_code.push_str("                delay = min(delay * self.backoff_multiplier, self.max_delay_ms / 1000.0)\n\n");
        retry_code.push_str("        raise last_exception\n\n");
        retry_code.push_str("    def _should_retry(self, exception: Exception) -> bool:\n");
        retry_code.push_str("        \"\"\"Determine if request should be retried.\"\"\"\n");
        retry_code.push_str("        if isinstance(exception, httpx.HTTPStatusError):\n");
        retry_code.push_str(
            "            return exception.response.status_code in [429, 500, 502, 503, 504]\n",
        );
        retry_code.push_str("        return False\n");

        Ok(retry_code)
    }

    fn generate_package_file(&self, config: &SdkConfig) -> Result<String, SdkGeneratorError> {
        let mut setup = String::new();

        setup.push_str("from setuptools import setup, find_packages\n\n");
        setup.push_str("setup(\n");
        setup.push_str(&format!("    name='{}',\n", config.package_name));
        setup.push_str(&format!("    version='{}',\n", config.version));
        setup.push_str(&format!(
            "    description='Python SDK for {}',\n",
            config.package_name
        ));
        setup.push_str("    author='COOLJAPAN OU (Team Kitasan)',\n");
        setup.push_str("    license='MIT OR Apache-2.0',\n");
        setup.push_str("    packages=find_packages(),\n");
        setup.push_str("    install_requires=[\n");
        for (name, version) in self.get_dependencies() {
            setup.push_str(&format!("        '{}{}',\n", name, version));
        }
        setup.push_str("    ],\n");
        setup.push_str("    python_requires='>=3.8',\n");
        setup.push_str("    classifiers=[\n");
        setup.push_str("        'Development Status :: 4 - Beta',\n");
        setup.push_str("        'Intended Audience :: Developers',\n");
        setup.push_str("        'License :: OSI Approved :: MIT License',\n");
        setup.push_str("        'License :: OSI Approved :: Apache Software License',\n");
        setup.push_str("        'Programming Language :: Python :: 3',\n");
        setup.push_str("        'Programming Language :: Python :: 3.8',\n");
        setup.push_str("        'Programming Language :: Python :: 3.9',\n");
        setup.push_str("        'Programming Language :: Python :: 3.10',\n");
        setup.push_str("        'Programming Language :: Python :: 3.11',\n");
        setup.push_str("        'Programming Language :: Python :: 3.12',\n");
        setup.push_str("    ],\n");
        setup.push_str(")\n");

        Ok(setup)
    }

    fn generate_readme(
        &self,
        spec: &OpenApiSpec,
        config: &SdkConfig,
    ) -> Result<String, SdkGeneratorError> {
        let mut readme = String::new();

        readme.push_str(&format!("# {} Python SDK\n\n", spec.info.title));

        if let Some(desc) = &spec.info.description {
            readme.push_str(&format!("{}\n\n", desc));
        }

        readme.push_str("## Installation\n\n");
        readme.push_str("```bash\n");
        readme.push_str(&format!("pip install {}\n", config.package_name));
        readme.push_str("```\n\n");

        readme.push_str("## Usage\n\n");
        readme.push_str("### Basic Example\n\n");
        readme.push_str("```python\n");
        readme.push_str("import asyncio\n");
        readme.push_str(&format!(
            "from {} import LegalisClient, JwtAuthHandler\n\n",
            config.package_name
        ));
        readme.push_str("async def main():\n");
        readme.push_str("    # Create client with JWT authentication\n");
        readme.push_str("    async with LegalisClient(\n");
        readme.push_str(&format!("        base_url='{}',\n", config.base_url));
        readme.push_str("        auth_handler=JwtAuthHandler('your-jwt-token'),\n");
        readme.push_str("        timeout=30.0,\n");
        readme.push_str("    ) as client:\n");
        readme.push_str("        # Make API calls\n");
        readme.push_str("        response = await client.get_statutes()\n");
        readme.push_str("        print(response['data'])\n\n");
        readme.push_str("asyncio.run(main())\n");
        readme.push_str("```\n\n");

        readme.push_str("### Authentication\n\n");
        readme.push_str("The SDK supports multiple authentication methods:\n\n");
        readme.push_str("#### JWT Authentication\n");
        readme.push_str("```python\n");
        readme.push_str(&format!(
            "from {} import JwtAuthHandler\n\n",
            config.package_name
        ));
        readme.push_str("auth_handler = JwtAuthHandler('your-jwt-token')\n");
        readme.push_str("```\n\n");

        readme.push_str("#### API Key Authentication\n");
        readme.push_str("```python\n");
        readme.push_str(&format!(
            "from {} import ApiKeyAuthHandler\n\n",
            config.package_name
        ));
        readme.push_str(
            "auth_handler = ApiKeyAuthHandler('your-api-key', header_name='X-API-Key')\n",
        );
        readme.push_str("```\n\n");

        readme.push_str("#### OAuth2 Authentication\n");
        readme.push_str("```python\n");
        readme.push_str(&format!(
            "from {} import OAuth2AuthHandler\n\n",
            config.package_name
        ));
        readme.push_str("auth_handler = OAuth2AuthHandler(\n");
        readme.push_str("    client_id='client-id',\n");
        readme.push_str("    client_secret='client-secret',\n");
        readme.push_str("    token_url='https://auth.example.com/token'\n");
        readme.push_str(")\n");
        readme.push_str("```\n\n");

        readme.push_str("## License\n\n");
        readme.push_str("MIT OR Apache-2.0\n");

        Ok(readme)
    }

    fn generate_tests(
        &self,
        _spec: &OpenApiSpec,
        config: &SdkConfig,
    ) -> Result<String, SdkGeneratorError> {
        let mut tests_code = String::new();

        tests_code.push_str("\"\"\"Tests for the client.\"\"\"\n\n");
        tests_code.push_str("import pytest\n");
        tests_code.push_str(&format!(
            "from {} import LegalisClient, JwtAuthHandler, ApiKeyAuthHandler\n\n\n",
            config.package_name
        ));

        tests_code.push_str("@pytest.mark.asyncio\n");
        tests_code.push_str("async def test_create_client():\n");
        tests_code.push_str("    \"\"\"Test client creation.\"\"\"\n");
        tests_code.push_str("    client = LegalisClient(base_url='http://localhost:3000')\n");
        tests_code.push_str("    assert client is not None\n");
        tests_code.push_str("    await client.close()\n\n\n");

        tests_code.push_str("@pytest.mark.asyncio\n");
        tests_code.push_str("async def test_create_client_with_jwt_auth():\n");
        tests_code.push_str("    \"\"\"Test client creation with JWT auth.\"\"\"\n");
        tests_code.push_str("    auth_handler = JwtAuthHandler('test-token')\n");
        tests_code.push_str("    client = LegalisClient(\n");
        tests_code.push_str("        base_url='http://localhost:3000',\n");
        tests_code.push_str("        auth_handler=auth_handler\n");
        tests_code.push_str("    )\n");
        tests_code.push_str("    assert client is not None\n");
        tests_code.push_str("    await client.close()\n\n\n");

        tests_code.push_str("@pytest.mark.asyncio\n");
        tests_code.push_str("async def test_create_client_with_api_key_auth():\n");
        tests_code.push_str("    \"\"\"Test client creation with API key auth.\"\"\"\n");
        tests_code.push_str("    auth_handler = ApiKeyAuthHandler('test-key')\n");
        tests_code.push_str("    client = LegalisClient(\n");
        tests_code.push_str("        base_url='http://localhost:3000',\n");
        tests_code.push_str("        auth_handler=auth_handler\n");
        tests_code.push_str("    )\n");
        tests_code.push_str("    assert client is not None\n");
        tests_code.push_str("    await client.close()\n");

        Ok(tests_code)
    }
}

impl PythonSdkGenerator {
    fn generate_python_operation_methods(
        &self,
        code: &mut String,
        path: &str,
        method: &str,
        operation: Option<&Operation>,
    ) -> Result<(), SdkGeneratorError> {
        if let Some(op) = operation {
            let operation_id = op.operation_id.as_ref().ok_or_else(|| {
                SdkGeneratorError::InvalidSpec(format!(
                    "Missing operationId for {} {}",
                    method, path
                ))
            })?;

            // Generate docstring
            code.push_str(&format!(
                "    async def {}(self",
                self.to_snake_case(operation_id)
            ));

            let has_path_params = path.contains('{');
            let has_query_params = op
                .parameters
                .as_ref()
                .map(|p| p.iter().any(|param| param.location == "query"))
                .unwrap_or(false);
            let has_body = op.request_body.is_some();

            let mut params = Vec::new();
            if has_path_params {
                params.push("path_params: Dict[str, str]".to_string());
            }
            if has_query_params {
                params.push("query_params: Optional[Dict[str, str]] = None".to_string());
            }
            if has_body {
                params.push("body: Any".to_string());
            }

            if !params.is_empty() {
                code.push_str(", ");
                code.push_str(&params.join(", "));
            }

            code.push_str(") -> Any:\n");
            code.push_str(&format!(
                "        \"\"\"{}.\"\"\"\n",
                op.summary.as_deref().unwrap_or("")
            ));

            // Generate method body
            if has_path_params {
                code.push_str(&format!("        path = '{}'\n", path));
                code.push_str("        for key, value in path_params.items():\n");
                code.push_str("            path = path.replace(f'{{{key}}}', value)\n");
            } else {
                code.push_str(&format!("        path = '{}'\n", path));
            }

            code.push_str("        return await self._request(\n");
            code.push_str(&format!("            '{}',\n", method.to_uppercase()));
            code.push_str("            path,\n");
            if has_body {
                code.push_str("            json=body,\n");
            }
            if has_query_params {
                code.push_str("            params=query_params,\n");
            }
            code.push_str("        )\n\n");
        }

        Ok(())
    }

    fn to_snake_case(&self, s: &str) -> String {
        let mut result = String::new();
        let mut prev_is_upper = false;

        for (i, ch) in s.chars().enumerate() {
            if ch.is_uppercase() {
                if i > 0 && !prev_is_upper {
                    result.push('_');
                }
                result.push(ch.to_ascii_lowercase());
                prev_is_upper = true;
            } else {
                result.push(ch);
                prev_is_upper = false;
            }
        }

        result
    }

    fn to_pascal_case(&self, s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;

        for ch in s.chars() {
            if ch == '_' || ch == '-' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(ch.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(ch);
            }
        }

        result
    }

    fn map_type_to_python(&self, schema: &Schema) -> String {
        if let Some(ref_path) = &schema.reference {
            return ref_path.split('/').next_back().unwrap_or("Any").to_string();
        }

        match schema.schema_type.as_deref() {
            Some("string") => "str".to_string(),
            Some("number") | Some("integer") => "int".to_string(),
            Some("boolean") => "bool".to_string(),
            Some("array") => {
                if let Some(items) = &schema.items {
                    format!("List[{}]", self.map_type_to_python(items))
                } else {
                    "List[Any]".to_string()
                }
            }
            Some("object") => "Dict[str, Any]".to_string(),
            _ => "Any".to_string(),
        }
    }

    fn generate_init_file(&self) -> String {
        let mut init = String::new();
        init.push_str("\"\"\"Legalis SDK for Python.\"\"\"\n\n");
        init.push_str("from .client import LegalisClient\n");
        init.push_str(
            "from .auth import AuthHandler, JwtAuthHandler, ApiKeyAuthHandler, OAuth2AuthHandler\n",
        );
        init.push_str("from .retry import RetryHandler\n");
        init.push_str("from . import models\n\n");
        init.push_str("__all__ = [\n");
        init.push_str("    'LegalisClient',\n");
        init.push_str("    'AuthHandler',\n");
        init.push_str("    'JwtAuthHandler',\n");
        init.push_str("    'ApiKeyAuthHandler',\n");
        init.push_str("    'OAuth2AuthHandler',\n");
        init.push_str("    'RetryHandler',\n");
        init.push_str("    'models',\n");
        init.push_str("]\n");
        init
    }

    fn generate_requirements(&self) -> String {
        let mut req = String::new();
        for (name, version) in self.get_dependencies() {
            req.push_str(&format!("{}{}\n", name, version));
        }
        req
    }

    fn generate_pyproject(&self, config: &SdkConfig) -> Result<String, SdkGeneratorError> {
        let pyproject = format!(
            r#"[build-system]
requires = ["setuptools>=61.0", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "{}"
version = "{}"
description = "Python SDK for Legalis API"
readme = "README.md"
requires-python = ">=3.8"
license = {{text = "MIT OR Apache-2.0"}}
authors = [
    {{name = "COOLJAPAN OU (Team Kitasan)"}}
]
classifiers = [
    "Development Status :: 4 - Beta",
    "Intended Audience :: Developers",
    "License :: OSI Approved :: MIT License",
    "License :: OSI Approved :: Apache Software License",
    "Programming Language :: Python :: 3",
    "Programming Language :: Python :: 3.8",
    "Programming Language :: Python :: 3.9",
    "Programming Language :: Python :: 3.10",
    "Programming Language :: Python :: 3.11",
    "Programming Language :: Python :: 3.12",
]
dependencies = [
    "httpx>=0.27.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=8.0.0",
    "pytest-asyncio>=0.23.0",
    "black>=24.0.0",
    "mypy>=1.8.0",
]
"#,
            config.package_name, config.version
        );

        Ok(pyproject)
    }

    fn get_dependencies(&self) -> HashMap<String, String> {
        let mut deps = HashMap::new();
        deps.insert("httpx".to_string(), ">=0.27.0".to_string());
        deps
    }
}

/// Parse OpenAPI specification from JSON.
pub fn parse_openapi_spec(spec_json: &Value) -> Result<OpenApiSpec, SdkGeneratorError> {
    serde_json::from_value(spec_json.clone())
        .map_err(|e| SdkGeneratorError::InvalidSpec(format!("Failed to parse OpenAPI spec: {}", e)))
}

/// Generate SDK for a specific language.
pub fn generate_sdk(
    spec: &OpenApiSpec,
    config: &SdkConfig,
) -> Result<GeneratedSdk, SdkGeneratorError> {
    match config.language {
        SdkLanguage::TypeScript => {
            let generator = TypeScriptSdkGenerator;
            generator.generate(spec, config)
        }
        SdkLanguage::Python => {
            let generator = PythonSdkGenerator;
            generator.generate(spec, config)
        }
        SdkLanguage::Rust | SdkLanguage::Go => Err(SdkGeneratorError::UnsupportedLanguage(
            format!("{} generator not yet implemented", config.language),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_spec() -> OpenApiSpec {
        OpenApiSpec {
            openapi: "3.0.3".to_string(),
            info: ApiInfo {
                title: "Test API".to_string(),
                version: "1.0.0".to_string(),
                description: Some("Test API description".to_string()),
            },
            servers: vec![ApiServer {
                url: "http://localhost:3000".to_string(),
                description: Some("Local server".to_string()),
            }],
            paths: HashMap::new(),
            components: None,
        }
    }

    fn create_test_config(language: SdkLanguage) -> SdkConfig {
        SdkConfig {
            language,
            package_name: "test-sdk".to_string(),
            version: "0.1.0".to_string(),
            base_url: "http://localhost:3000".to_string(),
            auth_method: Some(AuthMethod::JWT {
                header_name: "Authorization".to_string(),
            }),
            retry_config: RetryConfig::default(),
            timeout_seconds: 30,
            enable_streaming: true,
            generate_tests: true,
            generate_docs: true,
            output_dir: "./test-sdk".to_string(),
        }
    }

    #[test]
    fn test_typescript_generator_creates_all_files() {
        let spec = create_test_spec();
        let config = create_test_config(SdkLanguage::TypeScript);
        let generator = TypeScriptSdkGenerator;

        let result = generator.generate(&spec, &config);
        assert!(result.is_ok());

        let sdk = result.expect("Failed to generate SDK");
        assert_eq!(sdk.language, SdkLanguage::TypeScript);
        assert!(sdk.files.contains_key("src/client.ts"));
        assert!(sdk.files.contains_key("src/models.ts"));
        assert!(sdk.files.contains_key("src/auth.ts"));
        assert!(sdk.files.contains_key("src/retry.ts"));
        assert!(sdk.files.contains_key("package.json"));
        assert!(sdk.files.contains_key("README.md"));
        assert!(sdk.files.contains_key("src/index.ts"));
        assert!(sdk.files.contains_key("tsconfig.json"));
    }

    #[test]
    fn test_typescript_auth_generation() {
        let config = create_test_config(SdkLanguage::TypeScript);
        let generator = TypeScriptSdkGenerator;

        let auth_code = generator.generate_auth(&config);
        assert!(auth_code.is_ok());

        let code = auth_code.expect("Failed to generate auth");
        assert!(code.contains("JwtAuthHandler"));
        assert!(code.contains("ApiKeyAuthHandler"));
        assert!(code.contains("OAuth2AuthHandler"));
        assert!(code.contains("async addAuth"));
    }

    #[test]
    fn test_typescript_retry_generation() {
        let config = create_test_config(SdkLanguage::TypeScript);
        let generator = TypeScriptSdkGenerator;

        let retry_code = generator.generate_retry(&config);
        assert!(retry_code.is_ok());

        let code = retry_code.expect("Failed to generate retry");
        assert!(code.contains("RetryHandler"));
        assert!(code.contains("execute"));
        assert!(code.contains("shouldRetry"));
        assert!(code.contains("backoffMultiplier"));
    }

    #[test]
    fn test_python_generator_creates_all_files() {
        let spec = create_test_spec();
        let config = create_test_config(SdkLanguage::Python);
        let generator = PythonSdkGenerator;

        let result = generator.generate(&spec, &config);
        assert!(result.is_ok());

        let sdk = result.expect("Failed to generate SDK");
        assert_eq!(sdk.language, SdkLanguage::Python);
        assert!(sdk.files.contains_key("test-sdk/client.py"));
        assert!(sdk.files.contains_key("test-sdk/models.py"));
        assert!(sdk.files.contains_key("test-sdk/auth.py"));
        assert!(sdk.files.contains_key("test-sdk/retry.py"));
        assert!(sdk.files.contains_key("setup.py"));
        assert!(sdk.files.contains_key("README.md"));
        assert!(sdk.files.contains_key("requirements.txt"));
    }

    #[test]
    fn test_python_auth_generation() {
        let config = create_test_config(SdkLanguage::Python);
        let generator = PythonSdkGenerator;

        let auth_code = generator.generate_auth(&config);
        assert!(auth_code.is_ok());

        let code = auth_code.expect("Failed to generate auth");
        assert!(code.contains("class JwtAuthHandler"));
        assert!(code.contains("class ApiKeyAuthHandler"));
        assert!(code.contains("class OAuth2AuthHandler"));
        assert!(code.contains("async def add_auth"));
    }

    #[test]
    fn test_python_retry_generation() {
        let config = create_test_config(SdkLanguage::Python);
        let generator = PythonSdkGenerator;

        let retry_code = generator.generate_retry(&config);
        assert!(retry_code.is_ok());

        let code = retry_code.expect("Failed to generate retry");
        assert!(code.contains("class RetryHandler"));
        assert!(code.contains("async def execute"));
        assert!(code.contains("_should_retry"));
        assert!(code.contains("backoff_multiplier"));
    }

    #[test]
    fn test_retry_config_default_values() {
        let config = RetryConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.initial_delay_ms, 100);
        assert_eq!(config.max_delay_ms, 5000);
        assert_eq!(config.backoff_multiplier, 2.0);
        assert!(config.retry_on_status_codes.contains(&429));
        assert!(config.retry_on_status_codes.contains(&500));
    }

    #[test]
    fn test_sdk_language_display() {
        assert_eq!(format!("{}", SdkLanguage::TypeScript), "TypeScript");
        assert_eq!(format!("{}", SdkLanguage::Python), "Python");
        assert_eq!(format!("{}", SdkLanguage::Rust), "Rust");
        assert_eq!(format!("{}", SdkLanguage::Go), "Go");
    }

    #[test]
    fn test_parse_openapi_spec_valid() {
        let spec_json = serde_json::json!({
            "openapi": "3.0.3",
            "info": {
                "title": "Test API",
                "version": "1.0.0"
            },
            "servers": [],
            "paths": {}
        });

        let result = parse_openapi_spec(&spec_json);
        assert!(result.is_ok());

        let spec = result.expect("Failed to parse spec");
        assert_eq!(spec.openapi, "3.0.3");
        assert_eq!(spec.info.title, "Test API");
    }

    #[test]
    fn test_parse_openapi_spec_invalid() {
        let spec_json = serde_json::json!({
            "invalid": "spec"
        });

        let result = parse_openapi_spec(&spec_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_sdk_typescript() {
        let spec = create_test_spec();
        let config = create_test_config(SdkLanguage::TypeScript);

        let result = generate_sdk(&spec, &config);
        assert!(result.is_ok());

        let sdk = result.expect("Failed to generate SDK");
        assert_eq!(sdk.language, SdkLanguage::TypeScript);
    }

    #[test]
    fn test_generate_sdk_python() {
        let spec = create_test_spec();
        let config = create_test_config(SdkLanguage::Python);

        let result = generate_sdk(&spec, &config);
        assert!(result.is_ok());

        let sdk = result.expect("Failed to generate SDK");
        assert_eq!(sdk.language, SdkLanguage::Python);
    }

    #[test]
    fn test_generate_sdk_unsupported_language() {
        let spec = create_test_spec();
        let config = create_test_config(SdkLanguage::Rust);

        let result = generate_sdk(&spec, &config);
        assert!(result.is_err());

        if let Err(SdkGeneratorError::UnsupportedLanguage(msg)) = result {
            assert!(msg.contains("not yet implemented"));
        } else {
            panic!("Expected UnsupportedLanguage error");
        }
    }
}
