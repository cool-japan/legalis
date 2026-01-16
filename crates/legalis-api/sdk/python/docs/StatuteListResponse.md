# StatuteListResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | [**StatuteListResponseData**](StatuteListResponseData.md) |  | 

## Example

```python
from legalis_client.models.statute_list_response import StatuteListResponse

# TODO update the JSON string below
json = "{}"
# create an instance of StatuteListResponse from a JSON string
statute_list_response_instance = StatuteListResponse.from_json(json)
# print the JSON string representation of the object
print(StatuteListResponse.to_json())

# convert the object into a dict
statute_list_response_dict = statute_list_response_instance.to_dict()
# create an instance of StatuteListResponse from a dict
statute_list_response_from_dict = StatuteListResponse.from_dict(statute_list_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


