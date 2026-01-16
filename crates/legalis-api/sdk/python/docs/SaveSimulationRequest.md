# SaveSimulationRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **str** | Optional description | [optional] 
**name** | **str** | Name for the saved simulation | 
**simulation_result** | [**SimulationResponse**](SimulationResponse.md) |  | 

## Example

```python
from legalis_client.models.save_simulation_request import SaveSimulationRequest

# TODO update the JSON string below
json = "{}"
# create an instance of SaveSimulationRequest from a JSON string
save_simulation_request_instance = SaveSimulationRequest.from_json(json)
# print the JSON string representation of the object
print(SaveSimulationRequest.to_json())

# convert the object into a dict
save_simulation_request_dict = save_simulation_request_instance.to_dict()
# create an instance of SaveSimulationRequest from a dict
save_simulation_request_from_dict = SaveSimulationRequest.from_dict(save_simulation_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


