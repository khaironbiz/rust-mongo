use axum::response::{Html, IntoResponse, Json};
use axum::http::StatusCode;
use serde_json::json;

pub async fn docs_html() -> impl IntoResponse {
    // Simple Swagger UI HTML pointing to /openapi.json
    let html = r#"
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>API Docs</title>
    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@4/swagger-ui.css" />
  </head>
  <body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@4/swagger-ui-bundle.js"></script>
    <script>
      window.ui = SwaggerUIBundle({
        url: '/openapi.json',
        dom_id: '#swagger-ui',
      })
    </script>
  </body>
</html>
"#;

    Html(html)
}

pub async fn openapi_json() -> impl IntoResponse {
    let spec = json!({
        "openapi": "3.0.0",
        "info": { "title": "RME API", "version": "0.1.0" },
        "paths": {
            "/medical-records": {
                "get": { "summary": "List medical records" },
                "post": { "summary": "Create medical record" }
            },
            "/medical-records/{id}": {
                "get": { "summary": "Get medical record" },
                "put": { "summary": "Update medical record" },
                "delete": { "summary": "Delete medical record" }
            },
            "/doctors": { "get": { "summary": "List doctors" }, "post": {"summary": "Create doctor"} },
            "/nurses": { "get": { "summary": "List nurses" } },
            "/medicines": { "get": { "summary": "List medicines" } },
            "/appointments": { "get": { "summary": "List appointments" }, "post": {"summary": "Create appointment"} },
            "/services": { "get": { "summary": "List services" } },
            "/insurances": { "get": { "summary": "List insurances" } }
        }
    });

    (StatusCode::OK, Json(spec))
}
