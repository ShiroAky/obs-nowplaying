use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use warp::{Filter, http::Method, reply::Reply};

#[derive(Debug, Serialize, Deserialize)]
struct NowPlaying {
    title: String,
    artist: String,
    duration: Option<String>, // duración en segundos o minutos según lo envíe la extensión
    cover: Option<String>,    // base64 de la portada
}

fn get_videos_nowplaying() -> PathBuf {
    let home = dirs::video_dir().unwrap_or(dirs::home_dir().unwrap());
    let path = home.join("nowplaying");

    if !path.exists() {
        let _ = std::fs::create_dir_all(&path);
    }

    path
}

#[tokio::main]
async fn main() {
    let ruta = get_videos_nowplaying();
    println!("Carpeta de salida: {}", ruta.display());

    let ruta_clone = ruta.clone();

    // POST /update
    let update = warp::path("update")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |np: NowPlaying| {
            println!("POST /update recibido:");
            println!("🎵 Título: {}", np.title);
            println!("🎤 Artista: {}", np.artist);
            if let Some(ref duration) = np.duration {
                println!("⏱ Duración: {} minutos", duration.trim());
            } else {
                println!("⏱ Duración: no disponible");
            }
            println!(
                "🖼 Portada incluida: {}",
                if np.cover.is_some() { "sí" } else { "no" }
            );

            let json_path = ruta.join("nowplaying.json");
            let cover_path = ruta.join("cover.jpg");

            // Guardar JSON
            if let Ok(json) = serde_json::to_string_pretty(&np) {
                let _ = fs::write(&json_path, json);
            }

            // Guardar portada
            if let Some(cover_b64) = &np.cover {
                if let Ok(bytes) = general_purpose::STANDARD.decode(cover_b64) {
                    let _ = fs::write(&cover_path, bytes);
                }
            }

            // Guardar archivos individuales TXT
            let title_path = ruta.join("title.txt");
            let artist_path = ruta.join("artist.txt");
            let duration_path = ruta.join("duration.txt");

            let _ = fs::write(&title_path, &np.title);
            let _ = fs::write(&artist_path, &np.artist);
            let _ = fs::write(
                &duration_path,
                np.duration.clone().unwrap_or_else(|| "0".to_string()),
            );

            warp::reply::json(&"OK")
        });

    // GET /nowplaying
    let ruta_clone2 = ruta_clone.clone();
    let nowplaying = warp::path("nowplaying").and(warp::get()).map(move || {
        println!("GET /nowplaying recibido");

        let json_path = ruta_clone2.join("nowplaying.json");

        if json_path.exists() {
            match fs::read_to_string(&json_path) {
                Ok(content) => {
                    warp::reply::with_header(content, "Content-Type", "application/json")
                        .into_response()
                }
                Err(_) => {
                    println!("Error leyendo archivo nowplaying.json");
                    warp::reply::with_status(
                        "Error leyendo archivo".to_string(),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    )
                    .into_response()
                }
            }
        } else {
            println!("No hay información de ahora reproduciendo");
            warp::reply::with_status(
                "No hay información de ahora reproduciendo".to_string(),
                warp::http::StatusCode::NOT_FOUND,
            )
            .into_response()
        }
    });

    // CORS
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET, Method::POST])
        .allow_headers(vec!["Content-Type"]);

    let routes = update.or(nowplaying).with(cors);

    println!("Servidor corriendo en http://127.0.0.1:7539/");
    warp::serve(routes).run(([127, 0, 0, 1], 7539)).await;
}
