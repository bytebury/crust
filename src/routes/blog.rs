use axum::{
    Router,
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    routing::{delete, get, patch, put},
};
use axum_extra::extract::Form;

use crate::{
    extract::{admin_user::AdminUser, maybe_current_user::MaybeCurrentUser},
    prelude::*,
    util::htmx::HTMX,
};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/blog", get(index))
        .route("/blog", put(new_blog))
        .route("/blog/create", get(new_blog_page))
        .route("/blog/{slug}", delete(delete_blog))
        .route("/blog/{slug}", get(blog))
        .route("/blog/{slug}", patch(edit_blog))
        .route("/blog/{id}/edit", get(edit_blog_page))
}

#[derive(Template, WebTemplate)]
#[template(path = "blog/blogs.html")]
struct BlogsTemplate {
    shared: SharedContext,
    blogs: Vec<BlogPost>,
}

#[derive(Template, WebTemplate)]
#[template(path = "blog/blog.html")]
struct BlogTemplate {
    shared: SharedContext,
    blog: BlogPost,
}

#[derive(Template, WebTemplate)]
#[template(path = "blog/new_blog.html")]
struct NewBlogTemplate {
    form: NewBlogPost,
    error_message: Option<String>,
}

#[derive(Template, WebTemplate)]
#[template(path = "blog/edit_blog.html")]
struct EditBlogTemplate {
    form: EditBlogPost,
    error_message: Option<String>,
}

async fn index(
    State(state): State<SharedState>,
    MaybeCurrentUser(user): MaybeCurrentUser,
) -> BlogsTemplate {
    BlogsTemplate {
        shared: SharedContext::new(&state.app_info, user),
        blogs: state.blog_service.find_all().await.unwrap_or_default(),
    }
}

async fn new_blog_page(AdminUser(_): AdminUser) -> NewBlogTemplate {
    NewBlogTemplate {
        form: NewBlogPost::default(),
        error_message: None,
    }
}

async fn new_blog(
    State(state): State<SharedState>,
    AdminUser(user): AdminUser,
    Form(mut form): Form<NewBlogPost>,
) -> impl IntoResponse {
    form.author_id = user.id;

    match state.blog_service.create(&form).await {
        Ok(_) => HTMX::refresh().into_response(),
        Err(err) => NewBlogTemplate {
            form,
            error_message: Some(err.to_string()),
        }
        .into_response(),
    }
}

async fn blog(
    State(state): State<SharedState>,
    MaybeCurrentUser(user): MaybeCurrentUser,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    match state.blog_service.find_by_slug(&slug).await {
        Ok(blog) => BlogTemplate {
            shared: SharedContext::new(&state.app_info, user),
            blog,
        }
        .into_response(),
        Err(_) => Redirect::to("/404").into_response(),
    }
}

async fn edit_blog_page(
    State(state): State<SharedState>,
    AdminUser(_): AdminUser,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match state.blog_service.find_by_id(id).await {
        Ok(blog) => EditBlogTemplate {
            form: blog.into(),
            error_message: None,
        }
        .into_response(),
        Err(err) => EditBlogTemplate {
            form: EditBlogPost::default(),
            error_message: Some(err.to_string()),
        }
        .into_response(),
    }
}

async fn edit_blog(
    State(state): State<SharedState>,
    AdminUser(_): AdminUser,
    Path(id): Path<i64>,
    Form(mut form): Form<EditBlogPost>,
) -> impl IntoResponse {
    form.id = id;

    match state.blog_service.update(&form).await {
        Ok(blog) => HTMX::redirect(&format!("/blog/{}", blog.slug)).into_response(),
        Err(err) => EditBlogTemplate {
            form,
            error_message: Some(err.to_string()),
        }
        .into_response(),
    }
}

async fn delete_blog(
    State(state): State<SharedState>,
    AdminUser(_): AdminUser,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let _ = state.blog_service.delete(id).await;
    HTMX::redirect("/blog")
}
