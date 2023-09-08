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
    let s = "ul { color: red; }";
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <style>{s}</style>
            </head>
            <body>
                // call a component with no props
                <Section />

                // call a component with props and children
                <Section title="Hello">
                    <Items />
                </Section>
            </body>
        </html>
    }
}

#[props]
/// mark a struct with #[props] to use it as props in a component.
/// #[builder] can customize single props, marking them as option or setting a default value.
struct SectionProps {
    #[builder(setter(into), default = "Default Title".to_string())]
    title: String,
    #[builder(default)]
    children: String,
}

#[component]
/// mark functions with #[component] to use them as components inside html! macro
fn Section(props: SectionProps) -> String {
    html! {
        <div>
            <h1>{ props.title }</h1>
            { props.children }
        </div>
    }
}

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

/// async functions can be easily used in the body of a component, as every component is an async
/// function
async fn load_data_async() -> Vec<String> {
    vec!["a".to_string(), "b".to_string(), "c".to_string()]
}
