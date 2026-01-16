# SimulationResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completed_at** | **datetime** | Completion timestamp (RFC3339) | 
**deterministic_outcomes** | **int** | Number of deterministic outcomes | 
**deterministic_rate** | **float** | Percentage of deterministic outcomes | 
**discretionary_outcomes** | **int** | Number of discretionary outcomes | 
**discretionary_rate** | **float** | Percentage of discretionary outcomes | 
**simulation_id** | **str** | Unique simulation identifier | 
**total_entities** | **int** | Total number of entities simulated | 
**void_outcomes** | **int** | Number of void/inapplicable outcomes | 
**void_rate** | **float** | Percentage of void outcomes | 

## Example

```python
from legalis_client.models.simulation_response import SimulationResponse

# TODO update the JSON string below
json = "{}"
# create an instance of SimulationResponse from a JSON string
simulation_response_instance = SimulationResponse.from_json(json)
# print the JSON string representation of the object
print(SimulationResponse.to_json())

# convert the object into a dict
simulation_response_dict = simulation_response_instance.to_dict()
# create an instance of SimulationResponse from a dict
simulation_response_from_dict = SimulationResponse.from_dict(simulation_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


