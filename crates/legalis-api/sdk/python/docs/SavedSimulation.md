# SavedSimulation


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **datetime** |  | 
**created_by** | **str** | Username of creator | 
**description** | **str** | Optional description | [optional] 
**deterministic_outcomes** | **int** | Number of deterministic outcomes | [optional] 
**deterministic_rate** | **float** |  | 
**discretionary_outcomes** | **int** | Number of discretionary outcomes | [optional] 
**discretionary_rate** | **float** |  | 
**id** | **str** | Unique saved simulation identifier | 
**name** | **str** | User-provided name for the simulation | 
**population_size** | **int** | Population size | 
**statute_ids** | **List[str]** | Statute IDs used in simulation | [optional] 
**void_outcomes** | **int** | Number of void outcomes | [optional] 
**void_rate** | **float** |  | 

## Example

```python
from legalis_client.models.saved_simulation import SavedSimulation

# TODO update the JSON string below
json = "{}"
# create an instance of SavedSimulation from a JSON string
saved_simulation_instance = SavedSimulation.from_json(json)
# print the JSON string representation of the object
print(SavedSimulation.to_json())

# convert the object into a dict
saved_simulation_dict = saved_simulation_instance.to_dict()
# create an instance of SavedSimulation from a dict
saved_simulation_from_dict = SavedSimulation.from_dict(saved_simulation_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


