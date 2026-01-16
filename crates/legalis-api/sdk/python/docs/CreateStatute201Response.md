# CreateStatute201Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | [**Statute**](Statute.md) |  | [optional] 

## Example

```python
from legalis_client.models.create_statute201_response import CreateStatute201Response

# TODO update the JSON string below
json = "{}"
# create an instance of CreateStatute201Response from a JSON string
create_statute201_response_instance = CreateStatute201Response.from_json(json)
# print the JSON string representation of the object
print(CreateStatute201Response.to_json())

# convert the object into a dict
create_statute201_response_dict = create_statute201_response_instance.to_dict()
# create an instance of CreateStatute201Response from a dict
create_statute201_response_from_dict = CreateStatute201Response.from_dict(create_statute201_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


