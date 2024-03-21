use actix_web::{web, App, HttpResponse, HttpServer, Result};
use std::{fs::read_dir, path::PathBuf, time::Duration};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

async fn show_media(req: web::Path<PathBuf>) -> Result<actix_files::NamedFile> {
    let path: PathBuf = req.into_inner();
    let full_path = format!("media/{}", path.display());
    Ok(actix_files::NamedFile::open(full_path)?)
}

async fn slideshow(params: web::Query<SlideshowParams>) -> HttpResponse {
    // Get the interval from the query parameters or use a default value
    let interval_ms = params.interval.unwrap_or(5000);
    let interval_snap = 1*60*1000;

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
            let video_duration = get_video_duration(file); // Get the duration of the video
            html.push_str(&format!(
                r#"<video src="/{}" style="display: none; width: 100%; height: 100%; object-fit: contain;" muted autoplay duration="{}"></video>"#,
                file, video_duration.as_secs()
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
        var loop_end = false;
        function nextMedia() {{
            if (loop_end == true) {{
                showBlackScreen();
                loop_end = false;
            }}else{{
                media[index].style.display = 'none';
                index = (index + 1) % media.length;
                media[index].style.display = 'block';
                if (media[index].tagName === 'VIDEO') {{
                    media[index].play();
                    var duration = parseInt(media[index].getAttribute('duration'), 10);
                    setTimeout(nextMedia, duration * 1000); // Adjust interval based on video duration
                }} else {{
                    setTimeout(nextMedia, {});
                }}
                if (index == 0) loop_end = true;
            }}
        }}
        function showBlackScreen() {{
            media.forEach(function(element) {{
                element.style.display = 'none';
            }});
            var blackScreen = document.createElement('div');
            blackScreen.style.backgroundColor = 'black';
            blackScreen.style.position = 'absolute';
            blackScreen.style.top = '0';
            blackScreen.style.left = '0';
            blackScreen.style.width = '100%';
            blackScreen.style.height = '100%';
            blackScreen.style.zIndex = '9999';
            document.body.appendChild(blackScreen);
            setTimeout(function() {{
                document.body.removeChild(blackScreen);
                nextMedia(); // Start next loop after black screen
            }}, {}); // Duration of black screen in milliseconds
        }}
        nextMedia(); // Start slideshow immediately
        </script>
        "#,
        interval_ms, // Initial interval for the first loop
        interval_snap // Time to show black screen after one loop
    );

    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("<div style='position: relative;'>{}</div>{}", html, js))
}

fn get_video_duration(file_path: &str) -> Duration {
    // Mock implementation for getting video duration
    // You need to implement actual logic to get video duration
    // Here, we'll just return a fixed duration of 10 seconds
    Duration::from_secs(10)
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
            .service(web::resource("/{file:.*}").to(show_media))
            .service(actix_files::Files::new("/media", "media").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

