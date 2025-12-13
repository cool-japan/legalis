# legalis-llm TODO

## Completed

- [x] LLMProvider trait abstraction
- [x] OpenAI provider implementation
- [x] Anthropic Claude provider implementation
- [x] LLMConfig with temperature, max_tokens, system prompt
- [x] Structured JSON response generation
- [x] Law compiler for natural language to statute
- [x] Multi-provider switching support

## Providers

### New Implementations
- [ ] Add Google Gemini provider
- [ ] Add Azure OpenAI provider
- [ ] Add Mistral AI provider
- [ ] Add local LLM support (Ollama)
- [ ] Add llama.cpp direct integration
- [ ] Add HuggingFace Inference API support

### Provider Features
- [ ] Implement provider fallback chain
- [ ] Add automatic retry with exponential backoff
- [ ] Add circuit breaker pattern
- [ ] Implement request queuing with rate limiting
- [ ] Add provider health checking

## Streaming

- [ ] Add streaming response support
- [ ] Implement async stream combinators
- [ ] Add progress callbacks for long operations
- [ ] Support cancellation tokens

## Caching

- [ ] Implement response caching layer
- [ ] Add semantic caching (similar prompts)
- [ ] Support cache invalidation strategies
- [ ] Add cache persistence (disk, Redis)

## Token Management

- [ ] Add token counting before request
- [ ] Implement token usage tracking
- [ ] Add cost estimation per provider
- [ ] Create usage reports and analytics
- [ ] Implement budget/quota management

## Prompts

- [ ] Create prompt template system
- [ ] Add domain-specific prompt libraries
- [ ] Implement prompt versioning
- [ ] Add A/B testing for prompts
- [ ] Create prompt optimization suggestions

## Law Compiler

- [ ] Add batch compilation support
- [ ] Implement incremental compilation
- [ ] Add compilation cache
- [ ] Create compilation pipeline with stages
- [ ] Add custom pre/post processors

## Validation

- [ ] Add JSON schema validation for responses
- [ ] Implement retry on malformed responses
- [ ] Add confidence scoring for outputs
- [ ] Create validation rule definitions

## Testing

- [ ] Add integration tests with mock servers
- [ ] Create recorded response fixtures
- [ ] Add latency/performance tests
- [ ] Test error handling paths
- [ ] Add chaos testing for resilience
