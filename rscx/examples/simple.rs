use rscx::{component, html, props, CollectFragment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app().await;
    println!("{}", app);
    Ok(())
}

// simple function returning a String
// it will call the Items() function
async fn app() -> String {
    html! {
        <!DOCTYPE html>
        <div>
            <Items />
        </div>
    }
}

// mark functions with #[component] to use them as components inside html! macro
#[component]
async fn Items() -> String {
    let data = load_data_async().await;
    html! {
        <ul>
            {
                data
                    .into_iter()
                    .map(|item| html! { <li>{ item }</li> })
                    .collect_fragment() // helper method to collect a list of components into a String
            }
        </ul>
    }
}

// async functions can be easily used in the body of a component
async fn load_data_async() -> Vec<String> {
    vec!["a".to_string(), "b".to_string(), "c".to_string()]
}
