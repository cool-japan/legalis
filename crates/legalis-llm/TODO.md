# legalis-llm TODO

## Completed

- [x] LLMProvider trait abstraction
- [x] OpenAI provider implementation
- [x] Anthropic Claude provider implementation
- [x] LLMConfig with temperature, max_tokens, system prompt
- [x] Structured JSON response generation
- [x] Law compiler for natural language to statute
- [x] Multi-provider switching support
- [x] Google Gemini provider implementation
- [x] Streaming response support (OpenAI, Anthropic, Gemini)
- [x] Response caching layer with LRU eviction
- [x] Token usage tracking and cost estimation
- [x] Provider fallback chain
- [x] Automatic retry with exponential backoff
- [x] Circuit breaker pattern
- [x] Request queuing with rate limiting
- [x] Provider health checking
- [x] Prompt template system with variable substitution
- [x] Domain-specific prompt libraries (legal, coding)
- [x] Prompt versioning support
- [x] JSON schema validation for responses
- [x] Retry on malformed responses
- [x] Confidence scoring for outputs
- [x] Ollama provider for local LLM support
- [x] Budget/quota management with alerts
- [x] Batch compilation support for law compiler
- [x] Compilation cache for law compiler
- [x] Azure OpenAI provider implementation
- [x] Mistral AI provider implementation
- [x] Async stream combinators (map, filter, take, skip, etc.)
- [x] Progress callbacks for long operations
- [x] Semantic caching for similar prompts
- [x] Cache persistence (disk)
- [x] HuggingFace Inference API support
- [x] Cache invalidation strategies (time, version, pattern, tag-based)
- [x] A/B testing for prompts with statistics
- [x] Token estimation and counting

## Providers

### New Implementations
- [x] Add Azure OpenAI provider
- [x] Add Mistral AI provider
- [x] Add HuggingFace Inference API support
- [x] Add llama.cpp direct integration

## Streaming

- [x] Implement async stream combinators
- [x] Add progress callbacks for long operations
- [x] Support cancellation tokens

## Caching

- [x] Add semantic caching (similar prompts)
- [x] Support cache invalidation strategies
- [x] Add cache persistence (disk, Redis)

## Token Management

- [x] Add token counting before request (estimation-based)
- [x] Add model token limits
- [x] Add token truncation utilities

## Prompts

- [x] Add A/B testing for prompts
- [x] Create prompt optimization suggestions

## Law Compiler

- [x] Implement incremental compilation
- [x] Create compilation pipeline with stages
- [x] Add custom pre/post processors

## Validation

- [x] Create validation rule definitions (beyond JSON schema)

## Testing

- [x] Add integration tests with mock servers
- [x] Create recorded response fixtures
- [x] Add latency/performance tests
- [x] Test error handling paths
- [x] Add chaos testing for resilience

## New Features (2025)

### Embeddings
- [x] Embedding abstraction trait
- [x] OpenAI embeddings provider
- [x] Local embeddings provider (Ollama)
- [x] Vector similarity operations (cosine, euclidean, dot product)
- [x] K-means clustering for embeddings
- [x] Top-k similarity search

### Function Calling
- [x] Function definition and registration
- [x] Function parameter schemas (JSON Schema)
- [x] Function execution and result handling
- [x] Built-in helper functions (calculator, datetime)
- [x] Function call orchestration

### Model Routing
- [x] Routing strategies (cost-optimized, latency-optimized, balanced, complexity-based, round-robin)
- [x] Task complexity estimation
- [x] Provider scoring and selection
- [x] Load balancing across providers
- [x] Provider capability metadata

### Batch Processing
- [x] Batch request processing with concurrency control
- [x] Fail-fast and continue-on-error modes
- [x] Batch statistics and metrics
- [x] Parallel map operations
- [x] Configurable batch sizes and concurrency limits
