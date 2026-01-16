# StatuteListResponseData


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**statutes** | [**List[StatuteSummary]**](StatuteSummary.md) |  | [optional] 

## Example

```python
from legalis_client.models.statute_list_response_data import StatuteListResponseData

# TODO update the JSON string below
json = "{}"
# create an instance of StatuteListResponseData from a JSON string
statute_list_response_data_instance = StatuteListResponseData.from_json(json)
# print the JSON string representation of the object
print(StatuteListResponseData.to_json())

# convert the object into a dict
statute_list_response_data_dict = statute_list_response_data_instance.to_dict()
# create an instance of StatuteListResponseData from a dict
statute_list_response_data_from_dict = StatuteListResponseData.from_dict(statute_list_response_data_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


