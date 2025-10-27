// use crate::han
pub fn router(state: BlogState) -> Router{
    Router::new().route("/list_blogs", get(list_blogs))
}