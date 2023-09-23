# rscx 🪶

It looks like React (JSX) but it's executed in a macro, at build time. Your
runtime will only need to send the pre-formatted string back to the client.

By reducing the number of memory allocations needed, we can say that rscx is
*blazingly fast*.


## Usage

Import it from crates.io:

```
$ cargo add rscx
```

And, use it in your code:

<div style="width:100%;overflow-x:scroll;">

```Rust
use rscx::{component, html, props, CollectFragment};

#[tokio::main]
async fn main() {
    let app = app().await;
    println!("{}", app);
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

#[component]
/// mark functions with #[component] to use them as components inside html! macro
fn Section(
    // you can use `builder` attributes to specify a default value (makes this prop optional)
    #[builder(default = "Default title".into(), setter(into))] title: String,
    #[builder(default)] children: String,
) -> String {
    html! {
        <div>
            <h1>{ title }</h1>
            { children }
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
```

</div>
