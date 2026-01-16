# SavedSimulation


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **string** |  | [default to undefined]
**created_by** | **string** | Username of creator | [default to undefined]
**description** | **string** | Optional description | [optional] [default to undefined]
**deterministic_outcomes** | **number** | Number of deterministic outcomes | [optional] [default to undefined]
**deterministic_rate** | **number** |  | [default to undefined]
**discretionary_outcomes** | **number** | Number of discretionary outcomes | [optional] [default to undefined]
**discretionary_rate** | **number** |  | [default to undefined]
**id** | **string** | Unique saved simulation identifier | [default to undefined]
**name** | **string** | User-provided name for the simulation | [default to undefined]
**population_size** | **number** | Population size | [default to undefined]
**statute_ids** | **Array&lt;string&gt;** | Statute IDs used in simulation | [optional] [default to undefined]
**void_outcomes** | **number** | Number of void outcomes | [optional] [default to undefined]
**void_rate** | **number** |  | [default to undefined]

## Example

```typescript
import { SavedSimulation } from '@legalis/api-client';

const instance: SavedSimulation = {
    created_at,
    created_by,
    description,
    deterministic_outcomes,
    deterministic_rate,
    discretionary_outcomes,
    discretionary_rate,
    id,
    name,
    population_size,
    statute_ids,
    void_outcomes,
    void_rate,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
