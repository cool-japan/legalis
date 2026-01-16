# StatuteSummary


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**has_discretion** | **bool** | Whether this statute requires judicial discretion | 
**id** | **str** | Unique statute identifier | 
**precondition_count** | **int** | Number of preconditions | 
**title** | **str** | Human-readable statute title | 

## Example

```python
from legalis_client.models.statute_summary import StatuteSummary

# TODO update the JSON string below
json = "{}"
# create an instance of StatuteSummary from a JSON string
statute_summary_instance = StatuteSummary.from_json(json)
# print the JSON string representation of the object
print(StatuteSummary.to_json())

# convert the object into a dict
statute_summary_dict = statute_summary_instance.to_dict()
# create an instance of StatuteSummary from a dict
statute_summary_from_dict = StatuteSummary.from_dict(statute_summary_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


