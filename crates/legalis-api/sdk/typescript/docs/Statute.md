# Statute


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amendments** | **Array&lt;object&gt;** | Amendment history | [optional] [default to undefined]
**discretion_logic** | **string** | Description of discretionary logic | [optional] [default to undefined]
**effect** | [**StatuteEffect**](StatuteEffect.md) |  | [default to undefined]
**id** | **string** | Unique identifier | [default to undefined]
**jurisdiction** | **string** | Jurisdiction identifier | [optional] [default to undefined]
**preconditions** | **Array&lt;object&gt;** | List of preconditions (If clauses) | [default to undefined]
**relations** | **Array&lt;object&gt;** | Hierarchical relationships to other statutes | [optional] [default to undefined]
**temporal_validity** | **object** | Temporal validity constraints | [optional] [default to undefined]
**title** | **string** | Title of the statute | [default to undefined]
**version** | **number** | Version number | [default to undefined]

## Example

```typescript
import { Statute } from '@legalis/api-client';

const instance: Statute = {
    amendments,
    discretion_logic,
    effect,
    id,
    jurisdiction,
    preconditions,
    relations,
    temporal_validity,
    title,
    version,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
