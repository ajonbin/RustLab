use actix_files as fs;
use actix_web::{web, App, HttpServer, HttpResponse, Result};
use std::{fs::read_dir, path::PathBuf};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

async fn show_media(req: web::Path<PathBuf>) -> Result<fs::NamedFile> {
    let path: PathBuf = req.into_inner();
    let full_path = format!("media/{}", path.display());
    Ok(fs::NamedFile::open(full_path)?)
}

async fn slideshow(params: web::Query<SlideshowParams>) -> HttpResponse {

    let interval_ms = params.interval.unwrap_or(5000);

    // Get a list of all media files in the directory
    let media_files: Vec<String> = read_dir("media")
        .unwrap()
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .to_str()
                    .map(|s| s.to_string())
                    .filter(|s| s.ends_with(".jpg") || s.ends_with(".jpeg") || s.ends_with(".png") || s.ends_with(".gif") || s.ends_with(".mp4") || s.ends_with(".webm") || s.ends_with(".ogg") || s.ends_with(".avi"))
            })
        })
        .collect();

    // Generate HTML for the slideshow
    let mut html = String::new();
    for file in &media_files {
        if file.ends_with(".mp4") || file.ends_with(".webm") || file.ends_with(".ogg") || file.ends_with(".avi") {
            // If the file is a video, create a video element
            html.push_str(&format!(
                r#"<video src="/{}" style="display: none; width: 100%; height: 100%; object-fit: contain;" muted autoplay onended="nextMedia()"></video>"#,
                file
            ));
        } else {
            // If the file is an image, create an image element
            html.push_str(&format!(
                r#"<img src="/{}" style="display: none; max-width: 100%; max-height: 100%; position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%);" />"#,
                file
            ));
        }
    }

    // Generate JavaScript for the slideshow
    let js = format!(
        r#"
        <script>
        var media = document.querySelectorAll('img, video');
        var index = 0;
        function nextMedia() {{
            media[index].style.display = 'none';
            index = (index + 1) % media.length;
            media[index].style.display = 'block';
            if (media[index].tagName === 'VIDEO') {{
                media[index].play();
            }}
        }}
        setInterval(nextMedia, {});
        </script>
        "#,
        interval_ms
    );

    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("<div style='position: relative; width: 100vw; height: 100vh;'>{}</div>{}", html, js))
}

#[derive(serde::Deserialize)]
struct SlideshowParams {
    interval: Option<u64>,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/slideshow", web::get().to(slideshow))
            .service(web::resource("/media/{file:.*}").to(show_media))
            .service(fs::Files::new("/media", ".").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

