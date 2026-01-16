# SimulationApi

All URIs are relative to *http://localhost:3000*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**listSavedSimulations**](#listsavedsimulations) | **GET** /api/v1/simulate/saved | List saved simulations|
|[**runSimulation**](#runsimulation) | **POST** /api/v1/simulate | Run a simulation|
|[**saveSimulation**](#savesimulation) | **POST** /api/v1/simulate/saved | Save a simulation|
|[**streamSimulation**](#streamsimulation) | **POST** /api/v1/simulate/stream | Stream simulation results|

# **listSavedSimulations**
> Array<SavedSimulation> listSavedSimulations()

Retrieve a list of saved simulation results

### Example

```typescript
import {
    SimulationApi,
    Configuration
} from '@legalis/api-client';

const configuration = new Configuration();
const apiInstance = new SimulationApi(configuration);

let limit: number; // (optional) (default to 100)
let offset: number; // (optional) (default to 0)

const { status, data } = await apiInstance.listSavedSimulations(
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **limit** | [**number**] |  | (optional) defaults to 100|
| **offset** | [**number**] |  | (optional) defaults to 0|


### Return type

**Array<SavedSimulation>**

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List of saved simulations |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **runSimulation**
> SimulationResponse runSimulation(simulationRequest)

Simulates the application of statutes to a generated population

### Example

```typescript
import {
    SimulationApi,
    Configuration,
    SimulationRequest
} from '@legalis/api-client';

const configuration = new Configuration();
const apiInstance = new SimulationApi(configuration);

let simulationRequest: SimulationRequest; //

const { status, data } = await apiInstance.runSimulation(
    simulationRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **simulationRequest** | **SimulationRequest**|  | |


### Return type

**SimulationResponse**

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Simulation completed successfully |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **saveSimulation**
> SavedSimulation saveSimulation(saveSimulationRequest)

Save simulation results for later retrieval

### Example

```typescript
import {
    SimulationApi,
    Configuration,
    SaveSimulationRequest
} from '@legalis/api-client';

const configuration = new Configuration();
const apiInstance = new SimulationApi(configuration);

let saveSimulationRequest: SaveSimulationRequest; //

const { status, data } = await apiInstance.saveSimulation(
    saveSimulationRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **saveSimulationRequest** | **SaveSimulationRequest**|  | |


### Return type

**SavedSimulation**

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Simulation saved successfully |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **streamSimulation**
> string streamSimulation(simulationRequest)

Run simulation with real-time progress updates via Server-Sent Events (SSE)

### Example

```typescript
import {
    SimulationApi,
    Configuration,
    SimulationRequest
} from '@legalis/api-client';

const configuration = new Configuration();
const apiInstance = new SimulationApi(configuration);

let simulationRequest: SimulationRequest; //

const { status, data } = await apiInstance.streamSimulation(
    simulationRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **simulationRequest** | **SimulationRequest**|  | |


### Return type

**string**

### Authorization

[ApiKeyHeader](../README.md#ApiKeyHeader), [ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: text/event-stream


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | SSE stream established |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

