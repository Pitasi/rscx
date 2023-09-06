use rscx::{
    component,
    context::{provide_context, spawn_local, use_context},
    html, props,
};
use tokio::task::LocalSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Contexts are local to the tokio task that will render the component.
    // We need to spawn a local task in LocalSet, such a task will be able to
    // use `provide_context` and the children components will be able to use
    // `use_context`.
    let local = LocalSet::new();
    let res = local
        .run_until(async move {
            spawn_local(async {
                // provide a context of type String
                provide_context("John".to_string());
                app().await
            })
            .await
        })
        .await
        .unwrap();

    println!("{}", res);
    Ok(())
}

async fn app() -> String {
    html! {
        <!DOCTYPE html>
        <div>
            <Greetings />
        </div>
    }
}

#[component]
async fn Greetings() -> String {
    // use the context of type String
    let current_user = use_context::<String>();
    html! {
        <p>
        {
            if let Some(user) = current_user {
                format!("Hello, {}!", user)
            } else {
                "Log in first!".to_string()
            }
        }
        </p>
    }
}
