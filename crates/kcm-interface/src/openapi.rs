/// KCM REST API OpenAPI specification.
pub fn openapi_spec() -> &'static str {
    r#"{
  "openapi": "3.1.0",
  "info": {
    "title": "KCM Knowledge Columnar Model API",
    "version": "0.1.0",
    "description": "REST API for the KCM Knowledge Columnar Model engine"
  },
  "servers": [
    {"url": "http://localhost:8080", "description": "Local development"},
    {"url": "http://localhost:8080/v1", "description": "API v1"}
  ],
  "paths": {
    "/health": {
      "get": {"summary": "Health check", "operationId": "healthCheck", "responses": {"200": {"description": "Healthy"}}}
    },
    "/metrics": {
      "get": {"summary": "Prometheus metrics", "operationId": "getMetrics", "responses": {"200": {"description": "Metrics in Prometheus format"}}}
    },
    "/api/v1/facts": {
      "get": {
        "summary": "Query facts",
        "operationId": "queryFacts",
        "parameters": [
          {"name": "subject", "in": "query", "schema": {"type": "integer"}},
          {"name": "predicate", "in": "query", "schema": {"type": "integer"}},
          {"name": "object", "in": "query", "schema": {"type": "integer"}},
          {"name": "confidence_min", "in": "query", "schema": {"type": "number"}}
        ],
        "responses": {"200": {"description": "Fact list"}}
      },
      "post": {
        "summary": "Insert a fact",
        "operationId": "insertFact",
        "requestBody": {
          "required": true,
          "content": {
            "application/json": {
              "schema": {
                "type": "object",
                "properties": {
                  "subject": {"type": "integer"},
                  "predicate": {"type": "integer"},
                  "object": {"type": "integer"},
                  "confidence": {"type": "number"}
                },
                "required": ["subject", "predicate", "object", "confidence"]
              }
            }
          }
        },
        "responses": {"201": {"description": "Fact created"}}
      }
    },
    "/api/v1/facts/{id}": {
      "get": {"summary": "Get fact by ID", "operationId": "getFact",
        "parameters": [{"name": "id", "in": "path", "required": true, "schema": {"type": "integer"}}],
        "responses": {"200": {"description": "Fact"}, "404": {"description": "Not found"}}
      },
      "put": {"summary": "Update fact", "operationId": "updateFact",
        "parameters": [{"name": "id", "in": "path", "required": true, "schema": {"type": "integer"}}],
        "responses": {"200": {"description": "Updated"}, "404": {"description": "Not found"}}
      },
      "delete": {"summary": "Delete fact", "operationId": "deleteFact",
        "parameters": [{"name": "id", "in": "path", "required": true, "schema": {"type": "integer"}}],
        "responses": {"200": {"description": "Deleted"}, "404": {"description": "Not found"}}
      }
    },
    "/api/v1/stats": {
      "get": {"summary": "Database statistics", "operationId": "getStats",
        "responses": {"200": {"description": "Stats object"}}
      }
    },
    "/api/v1/batch": {
      "post": {
        "summary": "Batch insert facts",
        "operationId": "batchInsert",
        "requestBody": {
          "required": true,
          "content": {
            "application/json": {
              "schema": {
                "type": "array",
                "items": {
                  "type": "object",
                  "properties": {
                    "subject": {"type": "integer"},
                    "predicate": {"type": "integer"},
                    "object": {"type": "integer"},
                    "confidence": {"type": "number"}
                  }
                }
              }
            }
          }
        },
        "responses": {"200": {"description": "Batch result"}}
      }
    }
  }
}"#
}
