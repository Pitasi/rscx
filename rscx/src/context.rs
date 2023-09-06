use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    future::Future,
};

tokio::task_local! {
    pub(crate) static CONTEXT: RefCell<HashMap<TypeId, Box<dyn Any>>>;
}

pub fn spawn_local<F: Future<Output = O> + 'static, O: 'static>(
    fut: F,
) -> tokio::task::JoinHandle<O> {
    tokio::task::spawn_local(async move { CONTEXT.scope(RefCell::new(HashMap::new()), fut).await })
}

pub fn provide_context<T: 'static>(value: T) {
    let _ = CONTEXT.with(|context| {
        context
            .borrow_mut()
            .insert(TypeId::of::<T>(), Box::new(value));
    });
}

pub fn use_context<T>() -> Option<T>
where
    T: Clone + 'static,
{
    CONTEXT.with(|context| {
        context
            .borrow()
            .get(&TypeId::of::<T>())
            .map(|any| {
                any.downcast_ref::<T>().unwrap_or_else(|| {
                    panic!(
                        "Context type mismatch for type: {}",
                        std::any::type_name::<T>()
                    )
                })
            })
            .cloned()
    })
}

pub fn expect_context<T>() -> T
where
    T: Clone + 'static,
{
    use_context::<T>()
        .unwrap_or_else(|| panic!("Context not found for type: {}", std::any::type_name::<T>()))
}
