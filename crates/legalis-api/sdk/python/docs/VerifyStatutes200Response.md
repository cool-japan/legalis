# VerifyStatutes200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | [**VerifyResponse**](VerifyResponse.md) |  | [optional] 

## Example

```python
from legalis_client.models.verify_statutes200_response import VerifyStatutes200Response

# TODO update the JSON string below
json = "{}"
# create an instance of VerifyStatutes200Response from a JSON string
verify_statutes200_response_instance = VerifyStatutes200Response.from_json(json)
# print the JSON string representation of the object
print(VerifyStatutes200Response.to_json())

# convert the object into a dict
verify_statutes200_response_dict = verify_statutes200_response_instance.to_dict()
# create an instance of VerifyStatutes200Response from a dict
verify_statutes200_response_from_dict = VerifyStatutes200Response.from_dict(verify_statutes200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


