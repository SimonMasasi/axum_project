use axum::{
    Router, extract::{Path, State}, http::StatusCode, routing::get,
};

use crate::{
    error::{AppJson, AppResult, ErrorResponse}, modules::auth::{
        dtos::{roles::{CreateRoleInput, RolesResponse}}, services::roles as roles_service,
    }, state::AppState,
};



pub fn router () -> Router<AppState>{
    Router::new()
    .route("/", get(list_all_roles).post(create_role))
    .route("/{id}", get(find_role_by_id).patch(update_role))
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
        (status=200 , description = "Role Found" , body = RolesResponse),
        (status=404 , description = "role not Found" , body = ErrorResponse)
    )
)]
async  fn  find_role_by_id(State(state):State<AppState> , Path(id): Path<i32>) -> AppResult<AppJson<RolesResponse>>{

    let role = roles_service::get_role_by_id(&state.db, id).await?;

    Ok(AppJson(role.into()))
}



#[utoipa::path(
    post,
    path = "/roles",
    tag = "roles",
    request_body= CreateRoleInput,
    responses (
        (status = 201 , description = " Role create" , body = RolesResponse),
        (status = 400 , description = "Validation data error" , body = ErrorResponse),
        (status = 422, description = "Body is not valid JSON for this schema", body = ErrorResponse),
    )


)]
async  fn create_role(State(state):State<AppState> , AppJson(input):AppJson<CreateRoleInput>) -> AppResult<(StatusCode , AppJson<RolesResponse>)>{

let created_role = roles_service::create_role(&state.db, input).await?;

Ok((StatusCode::CREATED , AppJson(created_role.into())))

}




#[utoipa::path(
    patch,
    path = "/roles/{id}",
    tag = "roles",
    params(
        ("id" = i32, Path, description = "Role id"),
    ),
    request_body = CreateRoleInput,
    responses(
        (status = 200, description = "Role user", body = RolesResponse),
        (status = 400, description = "Validation failed", body = ErrorResponse),
        (status = 404, description = "Role not found", body = ErrorResponse),
    )
)]
async  fn update_role(State(state):State<AppState> , Path(id):Path<i32> , AppJson(input):AppJson<CreateRoleInput>) -> AppResult<AppJson<RolesResponse>>{
    let updated_role = roles_service::update_role(&state.db, id, input).await?;

    Ok(AppJson(updated_role.into()))
}