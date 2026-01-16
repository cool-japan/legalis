# CreateStatuteRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**statute** | [**Statute**](Statute.md) |  | 

## Example

```python
from legalis_client.models.create_statute_request import CreateStatuteRequest

# TODO update the JSON string below
json = "{}"
# create an instance of CreateStatuteRequest from a JSON string
create_statute_request_instance = CreateStatuteRequest.from_json(json)
# print the JSON string representation of the object
print(CreateStatuteRequest.to_json())

# convert the object into a dict
create_statute_request_dict = create_statute_request_instance.to_dict()
# create an instance of CreateStatuteRequest from a dict
create_statute_request_from_dict = CreateStatuteRequest.from_dict(create_statute_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


