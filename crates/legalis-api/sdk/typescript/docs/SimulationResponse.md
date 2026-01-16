# SimulationResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completed_at** | **string** | Completion timestamp (RFC3339) | [default to undefined]
**deterministic_outcomes** | **number** | Number of deterministic outcomes | [default to undefined]
**deterministic_rate** | **number** | Percentage of deterministic outcomes | [default to undefined]
**discretionary_outcomes** | **number** | Number of discretionary outcomes | [default to undefined]
**discretionary_rate** | **number** | Percentage of discretionary outcomes | [default to undefined]
**simulation_id** | **string** | Unique simulation identifier | [default to undefined]
**total_entities** | **number** | Total number of entities simulated | [default to undefined]
**void_outcomes** | **number** | Number of void/inapplicable outcomes | [default to undefined]
**void_rate** | **number** | Percentage of void outcomes | [default to undefined]

## Example

```typescript
import { SimulationResponse } from '@legalis/api-client';

const instance: SimulationResponse = {
    completed_at,
    deterministic_outcomes,
    deterministic_rate,
    discretionary_outcomes,
    discretionary_rate,
    simulation_id,
    total_entities,
    void_outcomes,
    void_rate,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
