use actix_web::{web, HttpResponse, Responder};
use crate::data::loader::{Map, SurfaceType};
use crate::navigation::astar::{astar_pathfinding, PathSegment}; 
use log::{info, warn};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct PathQuery {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

#[derive(Serialize)]
struct PathResponse {
    success: bool,
    message: String,
    segments: Option<Vec<PathSegment >>,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/navigate", web::get().to(find_path));
}

async fn find_path(query: web::Query<PathQuery>) -> impl Responder {
    let map = Map::load_from_file("data/SurfaceInfo.txt");

    match astar_pathfinding(&map, (query.start_x, query.start_y), (query.end_x, query.end_y)) {
        Some(segments) => HttpResponse::Ok().json(PathResponse {
            success: true,
            message: "Chemin trouvé avec succès".to_string(),
            segments: Some(segments),
        }),
        None => HttpResponse::NotFound().json(PathResponse {
            success: false,
            message: "Aucun chemin trouvé.".to_string(),
            segments: None,
        }),
    }
}















// use actix_web::{web, HttpResponse, Responder};
// use crate::data::loader::{Map, SurfaceType};
// use crate::navigation::astar::{astar_pathfinding, PathSegment }; // Correction : PathStep remplacé par PathSegment 
// use log::{info, warn};
// use serde::{Serialize, Deserialize};

// #[derive(Deserialize)]
// struct PathQuery {
//     start_x: usize,
//     start_y: usize,
//     end_x: usize,
//     end_y: usize,
// }

// #[derive(Serialize)]
// struct PathResponse {
//     success: bool,
//     message: String,
//     path: Option<Vec<(usize, usize)>>,
//     details: Option<Vec<CompressedPathStep>>, // Correction : PathStep → CompressedPathStep
// }

// pub fn configure(cfg: &mut web::ServiceConfig) {
//     cfg.route("/navigate", web::get().to(find_path));
// }

// async fn find_path(query: web::Query<PathQuery>) -> impl Responder {
//     let PathQuery { start_x, start_y, end_x, end_y } = query.into_inner();
//     info!("🛰️ Requête reçue : start=({}, {}), end=({}, {})", start_x, start_y, end_x, end_y);

//     let map = Map::load_from_file("data/SurfaceInfo.txt");

//     let start_surface = map.get_surface(start_x, start_y);
//     let end_surface = map.get_surface(end_x, end_y);

//     if matches!(start_surface, SurfaceType::Mur) || matches!(end_surface, SurfaceType::Mur) {
//         warn!("❌ Départ ou arrivée sur un mur !");
//         return HttpResponse::BadRequest().json(PathResponse {
//             success: false,
//             message: "Le point de départ ou d'arrivée est un mur.".to_string(),
//             path: None,
//             details: None, 
//         });
//     }

//     match astar_pathfinding(&map, (start_x, start_y), (end_x, end_y)) {
//         Some(full_path) => {
//             let simple_path: Vec<(usize, usize)> = full_path.iter().map(|step| step.start).collect(); // Correction : coordinates → start
//             info!("✅ Chemin trouvé avec {} étapes", full_path.len());

//             HttpResponse::Ok().json(PathResponse {
//                 success: true,
//                 message: "Chemin trouvé avec succès".to_string(),
//                 path: Some(simple_path),
//                 details: Some(full_path),
//             })
//         },
//         None => {
//             warn!("❌ Aucun chemin trouvé !");
//             HttpResponse::NotFound().json(PathResponse {
//                 success: false,
//                 message: "Aucun chemin trouvé. Vérifiez que le chemin est possible.".to_string(),
//                 path: None,
//                 details: None,
//             })
//         }
//     }
// }
