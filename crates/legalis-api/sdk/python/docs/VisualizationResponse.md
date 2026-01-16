# VisualizationResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content** | **str** | Generated visualization content | 
**discretionary_count** | **int** | Number of discretionary nodes | 
**format** | **str** | Output format used | 
**node_count** | **int** | Number of nodes in the decision tree | 
**statute_id** | **str** | ID of the visualized statute | 

## Example

```python
from legalis_client.models.visualization_response import VisualizationResponse

# TODO update the JSON string below
json = "{}"
# create an instance of VisualizationResponse from a JSON string
visualization_response_instance = VisualizationResponse.from_json(json)
# print the JSON string representation of the object
print(VisualizationResponse.to_json())

# convert the object into a dict
visualization_response_dict = visualization_response_instance.to_dict()
# create an instance of VisualizationResponse from a dict
visualization_response_from_dict = VisualizationResponse.from_dict(visualization_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


