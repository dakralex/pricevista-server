use crate::models::Brand;
use crate::schema::FilterOptions;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

pub async fn health_handler() -> impl IntoResponse {
    const MESSAGE: &str = "As happy and alive as one could be";

    Json(serde_json::json!({
        "status": "success",
        "message": MESSAGE
    }))
}

// pub async fn brands_list_handler(
//     opts: Option<Query<FilterOptions>>,
//     State(data): State<Arc<AppState>>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let Query(opts) = opts.unwrap_or_default();
//
//     let session = &data.db.get_session().await?;
//
//     let stmt = session
//         .prepare(
//             "
//         select b.brand_id,
//                c.company_name as name,
//                c.country,
//                c.admin_area,
//                c.locality
//         from brand b
//                  left join company c on b.company_id = c.id
//     ",
//         )
//         .await?;
//
//     let rows = stmt.query(()).await?;
//
//     while let Some(row) = rows.next()? {
//         let res = Brand {
//
//         }
//     }
// }
