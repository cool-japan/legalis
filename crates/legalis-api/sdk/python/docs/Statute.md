# Statute


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amendments** | **List[object]** | Amendment history | [optional] 
**discretion_logic** | **str** | Description of discretionary logic | [optional] 
**effect** | [**StatuteEffect**](StatuteEffect.md) |  | 
**id** | **str** | Unique identifier | 
**jurisdiction** | **str** | Jurisdiction identifier | [optional] 
**preconditions** | **List[object]** | List of preconditions (If clauses) | 
**relations** | **List[object]** | Hierarchical relationships to other statutes | [optional] 
**temporal_validity** | **object** | Temporal validity constraints | [optional] 
**title** | **str** | Title of the statute | 
**version** | **int** | Version number | 

## Example

```python
from legalis_client.models.statute import Statute

# TODO update the JSON string below
json = "{}"
# create an instance of Statute from a JSON string
statute_instance = Statute.from_json(json)
# print the JSON string representation of the object
print(Statute.to_json())

# convert the object into a dict
statute_dict = statute_instance.to_dict()
# create an instance of Statute from a dict
statute_from_dict = Statute.from_dict(statute_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


