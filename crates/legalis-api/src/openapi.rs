//! OpenAPI documentation module.
//!
//! This module provides manually-crafted OpenAPI 3.0 specification
//! for the Legalis API endpoints.

use serde_json::{Value, json};

/// Generates the complete OpenAPI 3.0 specification.
pub fn generate_spec() -> Value {
    json!({
        "openapi": "3.0.3",
        "info": {
            "title": "Legalis API",
            "description": "REST API for the Legalis-RS legal framework. Provides CRUD operations for statutes, verification, and simulation endpoints.",
            "version": "0.2.0",
            "contact": {
                "name": "Legalis-RS Project"
            },
            "license": {
                "name": "MIT OR Apache-2.0"
            }
        },
        "servers": [
            {
                "url": "http://localhost:3000",
                "description": "Local development server"
            }
        ],
        "tags": [
            {
                "name": "health",
                "description": "Health check and service status"
            },
            {
                "name": "statutes",
                "description": "Statute management operations"
            },
            {
                "name": "verification",
                "description": "Statute verification and validation"
            }
        ],
        "paths": {
            "/health": {
                "get": {
                    "tags": ["health"],
                    "summary": "Health check",
                    "description": "Returns the service health status",
                    "operationId": "healthCheck",
                    "responses": {
                        "200": {
                            "description": "Service is healthy",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "status": {
                                                "type": "string",
                                                "example": "healthy"
                                            },
                                            "service": {
                                                "type": "string",
                                                "example": "legalis-api"
                                            },
                                            "version": {
                                                "type": "string",
                                                "example": "0.2.0"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/v1/statutes": {
                "get": {
                    "tags": ["statutes"],
                    "summary": "List all statutes",
                    "description": "Returns a list of all statutes with summary information",
                    "operationId": "listStatutes",
                    "responses": {
                        "200": {
                            "description": "List of statutes",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/StatuteListResponse"
                                    }
                                }
                            }
                        }
                    }
                },
                "post": {
                    "tags": ["statutes"],
                    "summary": "Create a new statute",
                    "description": "Creates a new statute in the system",
                    "operationId": "createStatute",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/CreateStatuteRequest"
                                }
                            }
                        }
                    },
                    "responses": {
                        "201": {
                            "description": "Statute created successfully",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "data": {
                                                "$ref": "#/components/schemas/Statute"
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        "400": {
                            "description": "Invalid request",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/ErrorResponse"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/v1/statutes/{id}": {
                "get": {
                    "tags": ["statutes"],
                    "summary": "Get a statute by ID",
                    "description": "Returns detailed information about a specific statute",
                    "operationId": "getStatute",
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "description": "Statute ID",
                            "required": true,
                            "schema": {
                                "type": "string",
                                "example": "civil-code-art-1"
                            }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Statute details",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "data": {
                                                "$ref": "#/components/schemas/Statute"
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        "404": {
                            "description": "Statute not found",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/ErrorResponse"
                                    }
                                }
                            }
                        }
                    }
                },
                "delete": {
                    "tags": ["statutes"],
                    "summary": "Delete a statute",
                    "description": "Deletes a statute from the system",
                    "operationId": "deleteStatute",
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "description": "Statute ID to delete",
                            "required": true,
                            "schema": {
                                "type": "string"
                            }
                        }
                    ],
                    "responses": {
                        "204": {
                            "description": "Statute deleted successfully"
                        },
                        "404": {
                            "description": "Statute not found",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/ErrorResponse"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/v1/verify": {
                "post": {
                    "tags": ["verification"],
                    "summary": "Verify statutes",
                    "description": "Verifies one or more statutes for logical consistency and validity",
                    "operationId": "verifyStatutes",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/VerifyRequest"
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Verification results",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "object",
                                        "properties": {
                                            "data": {
                                                "$ref": "#/components/schemas/VerifyResponse"
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        "400": {
                            "description": "Invalid request",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/ErrorResponse"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        "components": {
            "schemas": {
                "ErrorResponse": {
                    "type": "object",
                    "required": ["error"],
                    "properties": {
                        "error": {
                            "type": "string",
                            "description": "Error message"
                        }
                    }
                },
                "StatuteListResponse": {
                    "type": "object",
                    "required": ["data"],
                    "properties": {
                        "data": {
                            "type": "object",
                            "properties": {
                                "statutes": {
                                    "type": "array",
                                    "items": {
                                        "$ref": "#/components/schemas/StatuteSummary"
                                    }
                                }
                            }
                        }
                    }
                },
                "StatuteSummary": {
                    "type": "object",
                    "required": ["id", "title", "has_discretion", "precondition_count"],
                    "properties": {
                        "id": {
                            "type": "string",
                            "description": "Unique statute identifier",
                            "example": "civil-code-art-1"
                        },
                        "title": {
                            "type": "string",
                            "description": "Human-readable statute title",
                            "example": "Adult Rights"
                        },
                        "has_discretion": {
                            "type": "boolean",
                            "description": "Whether this statute requires judicial discretion"
                        },
                        "precondition_count": {
                            "type": "integer",
                            "description": "Number of preconditions",
                            "minimum": 0
                        }
                    }
                },
                "CreateStatuteRequest": {
                    "type": "object",
                    "required": ["statute"],
                    "properties": {
                        "statute": {
                            "$ref": "#/components/schemas/Statute"
                        }
                    }
                },
                "Statute": {
                    "type": "object",
                    "required": ["id", "title", "preconditions", "effect", "version"],
                    "properties": {
                        "id": {
                            "type": "string",
                            "description": "Unique identifier",
                            "example": "civil-code-article-1"
                        },
                        "title": {
                            "type": "string",
                            "description": "Title of the statute"
                        },
                        "preconditions": {
                            "type": "array",
                            "description": "List of preconditions (If clauses)",
                            "items": {
                                "type": "object",
                                "description": "Condition specification (Age, Income, etc.)"
                            }
                        },
                        "effect": {
                            "type": "object",
                            "description": "Legal effect (Then clause)",
                            "required": ["effect_type", "description"],
                            "properties": {
                                "effect_type": {
                                    "type": "string",
                                    "enum": ["Grant", "Revoke", "Obligation", "Prohibition", "MonetaryTransfer", "StatusChange"]
                                },
                                "description": {
                                    "type": "string"
                                },
                                "parameters": {
                                    "type": "object",
                                    "additionalProperties": {
                                        "type": "string"
                                    }
                                }
                            }
                        },
                        "discretion_logic": {
                            "type": "string",
                            "nullable": true,
                            "description": "Description of discretionary logic"
                        },
                        "temporal_validity": {
                            "type": "object",
                            "description": "Temporal validity constraints"
                        },
                        "version": {
                            "type": "integer",
                            "description": "Version number",
                            "minimum": 1
                        },
                        "jurisdiction": {
                            "type": "string",
                            "nullable": true,
                            "description": "Jurisdiction identifier"
                        },
                        "relations": {
                            "type": "array",
                            "description": "Hierarchical relationships to other statutes",
                            "items": {
                                "type": "object"
                            }
                        },
                        "amendments": {
                            "type": "array",
                            "description": "Amendment history",
                            "items": {
                                "type": "object"
                            }
                        }
                    }
                },
                "VerifyRequest": {
                    "type": "object",
                    "required": ["statute_ids"],
                    "properties": {
                        "statute_ids": {
                            "type": "array",
                            "description": "List of statute IDs to verify (empty array = verify all)",
                            "items": {
                                "type": "string"
                            },
                            "example": ["civil-code-art-1", "tax-code-sec-42"]
                        }
                    }
                },
                "VerifyResponse": {
                    "type": "object",
                    "required": ["passed", "errors", "warnings"],
                    "properties": {
                        "passed": {
                            "type": "boolean",
                            "description": "Whether verification passed"
                        },
                        "errors": {
                            "type": "array",
                            "description": "List of errors found",
                            "items": {
                                "type": "string"
                            }
                        },
                        "warnings": {
                            "type": "array",
                            "description": "List of warnings",
                            "items": {
                                "type": "string"
                            }
                        }
                    }
                }
            }
        }
    })
}
