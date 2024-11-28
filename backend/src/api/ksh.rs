use axum::{
  routing::get, 
  Router, 
  Json,
};
use serde::Serialize;

pub fn ksh_router() -> Router {
  Router::new()
    .route("/check_update", get(check_update))

}

#[derive(Serialize)]
struct KshStatus {
    now_version: String,
    latest_version: String,
}

async fn check_update() -> Json<KshStatus> {
  let ksh_status = KshStatus{
    now_version:"0.0.0".to_string(), 
    latest_version: "1.0.0".to_string()
  };

  return Json(ksh_status);
}