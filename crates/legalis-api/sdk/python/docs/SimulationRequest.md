# SimulationRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**entity_params** | **Dict[str, str]** | Additional parameters for entity generation | 
**population_size** | **int** | Number of entities to generate for simulation | 
**statute_ids** | **List[str]** | IDs of statutes to simulate (empty &#x3D; all statutes) | 

## Example

```python
from legalis_client.models.simulation_request import SimulationRequest

# TODO update the JSON string below
json = "{}"
# create an instance of SimulationRequest from a JSON string
simulation_request_instance = SimulationRequest.from_json(json)
# print the JSON string representation of the object
print(SimulationRequest.to_json())

# convert the object into a dict
simulation_request_dict = simulation_request_instance.to_dict()
# create an instance of SimulationRequest from a dict
simulation_request_from_dict = SimulationRequest.from_dict(simulation_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


