# SimulationRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entity_params** | **{ [key: string]: string; }** | Additional parameters for entity generation | [default to undefined]
**population_size** | **number** | Number of entities to generate for simulation | [default to undefined]
**statute_ids** | **Array&lt;string&gt;** | IDs of statutes to simulate (empty &#x3D; all statutes) | [default to undefined]

## Example

```typescript
import { SimulationRequest } from '@legalis/api-client';

const instance: SimulationRequest = {
    entity_params,
    population_size,
    statute_ids,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
