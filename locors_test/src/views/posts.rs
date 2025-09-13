use loco_rs::prelude::*;

use crate::models::_entities::posts;

/// Render a list view of `posts`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<posts::Model>) -> Result<Response> {
    format::render().view(v, "posts/list.html", data!({"items": items}))
}

/// Render a single `posts` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &posts::Model) -> Result<Response> {
    format::render().view(v, "posts/show.html", data!({"item": item}))
}

/// Render a `posts` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "posts/create.html", data!({}))
}

/// Render a `posts` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &posts::Model) -> Result<Response> {
    format::render().view(v, "posts/edit.html", data!({"item": item}))
}
