use axum::{
    Router, extract::{Path, State}, routing::get,
};

use crate::{
    error::{AppJson, AppResult, ErrorResponse}, modules::auth::{
        dtos::roles::RolesResponse, services::roles as roles_service,
    }, state::AppState,
};



pub fn router () -> Router<AppState>{
    Router::new()
    .route("/", get(list_all_roles))
    .route("/{id}", get(find_role_by_id))
}


#[utoipa::path(
    get,
    path = "/roles",
    tag ="roles",
    responses(
        (status = 200 , description = "All roles" , body = Vec<RolesResponse>)
    )
)]
async  fn list_all_roles(State(state):State<AppState>) ->AppResult<AppJson<Vec<RolesResponse>>>{
    let  roles = roles_service::get_all_roles(&state.db).await?;

    Ok(AppJson(roles.into_iter().map(Into::into).collect()))

}



#[utoipa::path(
    get,
    path="/roles/{id}",
    tag ="roles",
    params(
        ("id"=i32 , Path , description= "User id"),
    ),
    responses (
        (status=200 , description = "User Found" , body = RolesResponse),
        (status=404 , description = "user not Found" , body = ErrorResponse)
    )
)]
async  fn  find_role_by_id(State(state):State<AppState> , Path(id): Path<i32>) -> AppResult<AppJson<RolesResponse>>{

    let role = roles_service::get_role_by_id(&state.db, id).await?;

    Ok(AppJson(role.into()))
}