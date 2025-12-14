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
- [ ] Add llama.cpp direct integration

## Streaming

- [x] Implement async stream combinators
- [x] Add progress callbacks for long operations
- [ ] Support cancellation tokens

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
- [ ] Create prompt optimization suggestions

## Law Compiler

- [ ] Implement incremental compilation
- [ ] Create compilation pipeline with stages
- [ ] Add custom pre/post processors

## Validation

- [ ] Create validation rule definitions (beyond JSON schema)

## Testing

- [ ] Add integration tests with mock servers
- [ ] Create recorded response fixtures
- [ ] Add latency/performance tests
- [ ] Test error handling paths
- [ ] Add chaos testing for resilience
