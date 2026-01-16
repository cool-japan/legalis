# VerifyResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | **List[str]** | List of errors found | 
**passed** | **bool** | Whether verification passed | 
**warnings** | **List[str]** | List of warnings | 

## Example

```python
from legalis_client.models.verify_response import VerifyResponse

# TODO update the JSON string below
json = "{}"
# create an instance of VerifyResponse from a JSON string
verify_response_instance = VerifyResponse.from_json(json)
# print the JSON string representation of the object
print(VerifyResponse.to_json())

# convert the object into a dict
verify_response_dict = verify_response_instance.to_dict()
# create an instance of VerifyResponse from a dict
verify_response_from_dict = VerifyResponse.from_dict(verify_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


