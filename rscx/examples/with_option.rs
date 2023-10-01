use rscx::html;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app().await;
    println!("{}", app);
    Ok(())
}

// simple function returning a String
// it will call the Items() function
async fn app() -> String {
    let s = Some("ul { color: red; }");
    let none: Option<&str> = None;
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <style>{ s }</style>
            </head>
            <body>
                { none }
                // call a component with no props
            </body>
        </html>
    }
}
