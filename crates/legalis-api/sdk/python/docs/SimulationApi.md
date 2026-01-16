# legalis_client.SimulationApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_saved_simulations**](SimulationApi.md#list_saved_simulations) | **GET** /api/v1/simulate/saved | List saved simulations
[**run_simulation**](SimulationApi.md#run_simulation) | **POST** /api/v1/simulate | Run a simulation
[**save_simulation**](SimulationApi.md#save_simulation) | **POST** /api/v1/simulate/saved | Save a simulation
[**stream_simulation**](SimulationApi.md#stream_simulation) | **POST** /api/v1/simulate/stream | Stream simulation results


# **list_saved_simulations**
> List[SavedSimulation] list_saved_simulations(limit=limit, offset=offset)

List saved simulations

Retrieve a list of saved simulation results

### Example

* Api Key Authentication (ApiKeyHeader):
* Api Key Authentication (ApiKeyAuth):
* Bearer (JWT) Authentication (BearerAuth):

```python
import legalis_client
from legalis_client.models.saved_simulation import SavedSimulation
from legalis_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost:3000
# See configuration.py for a list of all supported configuration parameters.
configuration = legalis_client.Configuration(
    host = "http://localhost:3000"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: ApiKeyHeader
configuration.api_key['ApiKeyHeader'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['ApiKeyHeader'] = 'Bearer'

# Configure API key authorization: ApiKeyAuth
configuration.api_key['ApiKeyAuth'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['ApiKeyAuth'] = 'Bearer'

# Configure Bearer authorization (JWT): BearerAuth
configuration = legalis_client.Configuration(
    access_token = os.environ["BEARER_TOKEN"]
)

# Enter a context with an instance of the API client
with legalis_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = legalis_client.SimulationApi(api_client)
    limit = 100 # int |  (optional) (default to 100)
    offset = 0 # int |  (optional) (default to 0)

    try:
        # List saved simulations
        api_response = api_instance.list_saved_simulations(limit=limit, offset=offset)
        print("The response of SimulationApi->list_saved_simulations:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SimulationApi->list_saved_simulations: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **int**|  | [optional] [default to 100]
 **offset** | **int**|  | [optional] [default to 0]

### Return type

[**List[SavedSimulation]**](SavedSimulation.md)

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | List of saved simulations |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **run_simulation**
> SimulationResponse run_simulation(simulation_request)

Run a simulation

Simulates the application of statutes to a generated population

### Example

* Api Key Authentication (ApiKeyHeader):
* Api Key Authentication (ApiKeyAuth):
* Bearer (JWT) Authentication (BearerAuth):

```python
import legalis_client
from legalis_client.models.simulation_request import SimulationRequest
from legalis_client.models.simulation_response import SimulationResponse
from legalis_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost:3000
# See configuration.py for a list of all supported configuration parameters.
configuration = legalis_client.Configuration(
    host = "http://localhost:3000"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: ApiKeyHeader
configuration.api_key['ApiKeyHeader'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['ApiKeyHeader'] = 'Bearer'

# Configure API key authorization: ApiKeyAuth
configuration.api_key['ApiKeyAuth'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['ApiKeyAuth'] = 'Bearer'

# Configure Bearer authorization (JWT): BearerAuth
configuration = legalis_client.Configuration(
    access_token = os.environ["BEARER_TOKEN"]
)

# Enter a context with an instance of the API client
with legalis_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = legalis_client.SimulationApi(api_client)
    simulation_request = {"entity_params":{"region":"EU"},"population_size":1000,"statute_ids":["tax-code-sec-1","tax-code-sec-2"]} # SimulationRequest | 

    try:
        # Run a simulation
        api_response = api_instance.run_simulation(simulation_request)
        print("The response of SimulationApi->run_simulation:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SimulationApi->run_simulation: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **simulation_request** | [**SimulationRequest**](SimulationRequest.md)|  | 

### Return type

[**SimulationResponse**](SimulationResponse.md)

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Simulation completed successfully |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **save_simulation**
> SavedSimulation save_simulation(save_simulation_request)

Save a simulation

Save simulation results for later retrieval

### Example

* Api Key Authentication (ApiKeyHeader):
* Api Key Authentication (ApiKeyAuth):
* Bearer (JWT) Authentication (BearerAuth):

```python
import legalis_client
from legalis_client.models.save_simulation_request import SaveSimulationRequest
from legalis_client.models.saved_simulation import SavedSimulation
from legalis_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost:3000
# See configuration.py for a list of all supported configuration parameters.
configuration = legalis_client.Configuration(
    host = "http://localhost:3000"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: ApiKeyHeader
configuration.api_key['ApiKeyHeader'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['ApiKeyHeader'] = 'Bearer'

# Configure API key authorization: ApiKeyAuth
configuration.api_key['ApiKeyAuth'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['ApiKeyAuth'] = 'Bearer'

# Configure Bearer authorization (JWT): BearerAuth
configuration = legalis_client.Configuration(
    access_token = os.environ["BEARER_TOKEN"]
)

# Enter a context with an instance of the API client
with legalis_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = legalis_client.SimulationApi(api_client)
    save_simulation_request = legalis_client.SaveSimulationRequest() # SaveSimulationRequest | 

    try:
        # Save a simulation
        api_response = api_instance.save_simulation(save_simulation_request)
        print("The response of SimulationApi->save_simulation:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SimulationApi->save_simulation: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **save_simulation_request** | [**SaveSimulationRequest**](SaveSimulationRequest.md)|  | 

### Return type

[**SavedSimulation**](SavedSimulation.md)

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**201** | Simulation saved successfully |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **stream_simulation**
> str stream_simulation(simulation_request)

Stream simulation results

Run simulation with real-time progress updates via Server-Sent Events (SSE)

### Example

* Api Key Authentication (ApiKeyHeader):
* Api Key Authentication (ApiKeyAuth):
* Bearer (JWT) Authentication (BearerAuth):

```python
import legalis_client
from legalis_client.models.simulation_request import SimulationRequest
from legalis_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost:3000
# See configuration.py for a list of all supported configuration parameters.
configuration = legalis_client.Configuration(
    host = "http://localhost:3000"
)

# The client must configure the authentication and authorization parameters
# in accordance with the API server security policy.
# Examples for each auth method are provided below, use the example that
# satisfies your auth use case.

# Configure API key authorization: ApiKeyHeader
configuration.api_key['ApiKeyHeader'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['ApiKeyHeader'] = 'Bearer'

# Configure API key authorization: ApiKeyAuth
configuration.api_key['ApiKeyAuth'] = os.environ["API_KEY"]

# Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
# configuration.api_key_prefix['ApiKeyAuth'] = 'Bearer'

# Configure Bearer authorization (JWT): BearerAuth
configuration = legalis_client.Configuration(
    access_token = os.environ["BEARER_TOKEN"]
)

# Enter a context with an instance of the API client
with legalis_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = legalis_client.SimulationApi(api_client)
    simulation_request = legalis_client.SimulationRequest() # SimulationRequest | 

    try:
        # Stream simulation results
        api_response = api_instance.stream_simulation(simulation_request)
        print("The response of SimulationApi->stream_simulation:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling SimulationApi->stream_simulation: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **simulation_request** | [**SimulationRequest**](SimulationRequest.md)|  | 

### Return type

**str**

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: text/event-stream

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | SSE stream established |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

