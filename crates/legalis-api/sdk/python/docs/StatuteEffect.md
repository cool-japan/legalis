# StatuteEffect

Legal effect (Then clause)

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **str** |  | 
**effect_type** | **str** |  | 
**parameters** | **Dict[str, str]** |  | [optional] 

## Example

```python
from legalis_client.models.statute_effect import StatuteEffect

# TODO update the JSON string below
json = "{}"
# create an instance of StatuteEffect from a JSON string
statute_effect_instance = StatuteEffect.from_json(json)
# print the JSON string representation of the object
print(StatuteEffect.to_json())

# convert the object into a dict
statute_effect_dict = statute_effect_instance.to_dict()
# create an instance of StatuteEffect from a dict
statute_effect_from_dict = StatuteEffect.from_dict(statute_effect_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


