use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};

use crate::{
    error::{AppJson, AppResult, ErrorResponse},
    modules::settings::{
        dtos::email_settings::{EmailSettingsResponse , EmailSettingsInputDto},
        services::email_settings as email_settings_service,
    },
    state::AppState,
};


pub fn  email_settings_router() -> Router<AppState>
{

    Router::new()
    .route("/" , get(list_all_email_settings))
        .route("/{id}", get(find_email_settings))

}


#[utoipa::path(
    get,
    path = "/email_settings",
    tag = "email_settings",
    responses(
    (status = 200 , description= "all email settings found" , body = Vec<EmailSettingsResponse> ),
    )
)]
async  fn list_all_email_settings(State(state):State<AppState>) -> AppResult<AppJson<Vec<EmailSettingsResponse>>>{

    let email_settings = email_settings_service::list(&state.db).await?;

    Ok(AppJson(email_settings.into_iter().map(EmailSettingsResponse::from).collect::<Vec<EmailSettingsResponse>>()))

}



#[utoipa::path(
get,
path = "/email_settings/{id}",
tag = "email_settings",
responses (
(status = 200,  description="all email settings found" ,  body = EmailSettingsResponse),
(status = 400 , description= "email settings not found" , body = ErrorResponse)
)
)]
async  fn find_email_settings(
    State(state):State<AppState>,
    Path(id):Path<i32> 
) -> AppResult<AppJson<EmailSettingsResponse>> {
    
    let email_setting = email_settings_service::get(&state.db, id).await?;
    
    Ok(AppJson(EmailSettingsResponse::from(email_setting)))
}