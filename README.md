![crates.io](https://img.shields.io/crates/v/rscx.svg)

# RSCx - Rust Server Components

RSCx is a server-side HTML rendering engine library with a neat developer
experience and great performance.

Features:

- all components are async functions
- JSX-like syntax called RSX parsed with [rstml](https://github.com/rs-tml/rstml)
- contexts, to easily pass values down the components tree ([example](https://github.com/Pitasi/rscx/blob/main/rscx/examples/context.rs))
- inspired by [Maud](https://maud.lambda.xyz/) and [Leptos](https://leptos.dev/)


## Usage

All the examples can be found in [rscx/examples/](https://github.com/Pitasi/rscx/tree/main/rscx/examples).

```rs
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
```


## Benchmarks

*Disclaimer*: RSCx is for servers, as the name suggests. Therefore the following
comparisons with Leptos are unfair. This library contains only a fraction of
Leptos' features.

*Disclaimer 2*: The benchmarks are pretty basics and should not influence your
decision on whether to use or not this library. Focus on the DX. They are
included as I kept running them to make sure I didn't fall too much behind
alternatives.

The time in the middle of the three is the average.

### Run the benchmarks locally

```
cd bench
# cargo install criterion
cargo criterion
```


### Benchmark 1: single element, lots of HTML attributes

```
many_attrs/maud_many_attrs
                        time:   [232.11 ns 238.52 ns 247.28 ns]
many_attrs/html_node_many_attrs
                        time:   [65.056 µs 65.355 µs 65.686 µs]
many_attrs/leptos_many_attrs
                        time:   [929.47 ns 936.84 ns 945.02 ns]
many_attrs/rscx_many_attrs
                        time:   [231.12 ns 238.95 ns 249.42 ns]
```

RSCx and Maud pretty much are the same as their macros output is effectively a
static string with the result.


### Benchmark 2: little element with props and child

```
small_fragment/maud_small_fragment
                        time:   [104.95 ns 105.03 ns 105.15 ns]
small_fragment/leptos_small_fragment
                        time:   [1.7115 µs 1.7139 µs 1.7169 µs]
small_fragment/rscx_small_fragment
                        time:   [99.514 ns 99.588 ns 99.673 ns]
```

RSCx offers a better DX than Maud, as the syntax is nicer and values such as i32
can be passed as props/attributes, while in Maud every attribute must be a
static string.


### Benchmark 3: async component rendering a list of 100 items

```
async_list/maud_async_list
                        time:   [2.3114 µs 2.3241 µs 2.3377 µs]
async_list/leptos_async_list
                        time:   [55.149 µs 55.228 µs 55.315 µs]
async_list/rscx_async_list
                        time:   [5.4809 µs 5.4987 µs 5.5151 µs]
```

I'll reiterate the disclaimer: Leptos is not specifically made for SSR. Going
through its reactivity system (using async resources) adds overhead.

