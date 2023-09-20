use rscx::{component, html};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app().await;
    println!("{}", app);
    Ok(())
}

// simple function returning a String
async fn app() -> String {
    let s = "ul { color: red; }";
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <style>{s}</style>
            </head>
            <body>
                // call a component with props and children
                <Section title="Hello">
                    <p>"I am a paragraph"</p>
                </Section>
            </body>
        </html>
    }
}

#[component]
/// mark functions with #[component] to use them as components inside html! macro
fn Section(title: String, children: String) -> String {
    html! {
        <div>
            <h1>{ title }</h1>
            { children }
        </div>
    }
}
