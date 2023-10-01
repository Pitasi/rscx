use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use csm::csm;
use rscx::{component, html};
use rscx_mdx::mdx::{Mdx, MdxComponentProps};

#[tokio::main]
async fn main() {
    let pages_dir = Path::new("pages");
    let dist_dir = Path::new("dist");
    let _ = fs::remove_dir_all(&dist_dir);
    render(&pages_dir, &dist_dir).await;
}

#[async_recursion::async_recursion]
async fn render(dir: &Path, dist_dir: &Path) {
    fs::create_dir_all(&dist_dir).unwrap();

    for entry in dir.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if entry.file_type().unwrap().is_dir() {
            render(&path, &dist_dir.join(entry.file_name())).await;
            continue;
        }

        let file_name = path.file_name().unwrap().to_str().unwrap();
        if !file_name.ends_with(".md") {
            continue;
        }

        println!("building {}", path.as_path().display());
        let file_name = file_name.trim_end_matches(".md");

        let mut file = File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut file = File::create(dist_dir.join(format!("{}.html", file_name))).unwrap();

        let html = render_markdown(contents).await;
        file.write_all(html.as_bytes()).unwrap();
    }
}

async fn render_markdown(md: String) -> String {
    html! {
        <Shell>
            <Mdx source=md handler=handler />
        </Shell>
    }
}

async fn handler(name: String, _props: MdxComponentProps) -> String {
    match name {
        _ => html! {},
    }
}

#[component]
async fn Shell(children: String) -> String {
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <title>rscx</title>
                <link rel="stylesheet" href="/bundle.css" />
            </head>
            <body class=csm!{Body,
                max-width: 600px,
                margin: auto,
            }>
                {children}
            </body>
        </html>
    }
}
