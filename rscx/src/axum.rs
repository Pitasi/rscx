use axum::response::Html;
use std::{future::Future, sync::OnceLock, thread::available_parallelism};
use tokio_util::task::LocalPoolHandle;

use crate::context;

fn get_rendering_pool() -> LocalPoolHandle {
    static LOCAL_POOL: OnceLock<LocalPoolHandle> = OnceLock::new();
    LOCAL_POOL
        .get_or_init(|| LocalPoolHandle::new(available_parallelism().map(Into::into).unwrap_or(1)))
        .clone()
}

pub async fn render<F, O>(f: F) -> Html<O>
where
    F: Future<Output = O> + Send + 'static,
    O: Send + 'static,
{
    get_rendering_pool()
        .spawn_pinned(move || async {
            let h = context::spawn_local(f).await.unwrap();
            Html(h)
        })
        .await
        .unwrap()
}
