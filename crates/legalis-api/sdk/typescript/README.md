## @legalis/api-client@0.2.0

This generator creates TypeScript/JavaScript client that utilizes [axios](https://github.com/axios/axios). The generated Node module can be used in the following environments:

Environment
* Node.js
* Webpack
* Browserify

Language level
* ES5 - you must have a Promises/A+ library installed
* ES6

Module system
* CommonJS
* ES6 module system

It can be used in both TypeScript and JavaScript. In TypeScript, the definition will be automatically resolved via `package.json`. ([Reference](https://www.typescriptlang.org/docs/handbook/declaration-files/consumption.html))

### Building

To build and compile the typescript sources to javascript use:
```
npm install
npm run build
```

### Publishing

First build the package then run `npm publish`

### Consuming

navigate to the folder of your consuming project and run one of the following commands.

_published:_

```
npm install @legalis/api-client@0.2.0 --save
```

_unPublished (not recommended):_

```
npm install PATH_TO_GENERATED_PACKAGE --save
```

### Documentation for API Endpoints

All URIs are relative to *http://localhost:3000*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*HealthApi* | [**healthCheck**](docs/HealthApi.md#healthcheck) | **GET** /health | Health check
*MetricsApi* | [**metrics**](docs/MetricsApi.md#metrics) | **GET** /metrics | Prometheus metrics
*SimulationApi* | [**listSavedSimulations**](docs/SimulationApi.md#listsavedsimulations) | **GET** /api/v1/simulate/saved | List saved simulations
*SimulationApi* | [**runSimulation**](docs/SimulationApi.md#runsimulation) | **POST** /api/v1/simulate | Run a simulation
*SimulationApi* | [**saveSimulation**](docs/SimulationApi.md#savesimulation) | **POST** /api/v1/simulate/saved | Save a simulation
*SimulationApi* | [**streamSimulation**](docs/SimulationApi.md#streamsimulation) | **POST** /api/v1/simulate/stream | Stream simulation results
*StatutesApi* | [**createStatute**](docs/StatutesApi.md#createstatute) | **POST** /api/v1/statutes | Create a new statute
*StatutesApi* | [**deleteStatute**](docs/StatutesApi.md#deletestatute) | **DELETE** /api/v1/statutes/{id} | Delete a statute
*StatutesApi* | [**getStatute**](docs/StatutesApi.md#getstatute) | **GET** /api/v1/statutes/{id} | Get a statute by ID
*StatutesApi* | [**listStatutes**](docs/StatutesApi.md#liststatutes) | **GET** /api/v1/statutes | List all statutes
*VerificationApi* | [**verifyStatutes**](docs/VerificationApi.md#verifystatutes) | **POST** /api/v1/verify | Verify statutes
*VisualizationApi* | [**visualizeStatute**](docs/VisualizationApi.md#visualizestatute) | **GET** /api/v1/visualize/{id} | Visualize a statute


### Documentation For Models

 - [CreateStatute201Response](docs/CreateStatute201Response.md)
 - [CreateStatuteRequest](docs/CreateStatuteRequest.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [HealthCheck200Response](docs/HealthCheck200Response.md)
 - [SaveSimulationRequest](docs/SaveSimulationRequest.md)
 - [SavedSimulation](docs/SavedSimulation.md)
 - [SimulationRequest](docs/SimulationRequest.md)
 - [SimulationResponse](docs/SimulationResponse.md)
 - [Statute](docs/Statute.md)
 - [StatuteEffect](docs/StatuteEffect.md)
 - [StatuteListResponse](docs/StatuteListResponse.md)
 - [StatuteListResponseData](docs/StatuteListResponseData.md)
 - [StatuteSummary](docs/StatuteSummary.md)
 - [VerifyRequest](docs/VerifyRequest.md)
 - [VerifyResponse](docs/VerifyResponse.md)
 - [VerifyStatutes200Response](docs/VerifyStatutes200Response.md)
 - [VisualizationResponse](docs/VisualizationResponse.md)


<a id="documentation-for-authorization"></a>
## Documentation For Authorization


Authentication schemes defined for the API:
<a id="ApiKeyAuth"></a>
### ApiKeyAuth

- **Type**: API key
- **API key parameter name**: Authorization
- **Location**: HTTP header

<a id="ApiKeyHeader"></a>
### ApiKeyHeader

- **Type**: API key
- **API key parameter name**: X-API-Key
- **Location**: HTTP header

<a id="BearerAuth"></a>
### BearerAuth

- **Type**: Bearer authentication (JWT)

