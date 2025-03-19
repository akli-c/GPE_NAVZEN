use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use crate::data::loader::{Map, SurfaceType};
use log::{info, warn};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PathSegment {
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub surface: String,
    pub cost: f32,
    pub line_number: usize,
}

#[derive(Debug, PartialEq)]
struct Node {
    cost: f32,
    position: (usize, usize),
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.total_cmp(&other.cost).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Convertit un `SurfaceType` en String
fn surface_to_string(surface: SurfaceType) -> String {
    match surface {
        SurfaceType::Mur => "Mur".to_string(),
        SurfaceType::Couloir => "Couloir".to_string(),
        SurfaceType::Exterieur => "Exterieur".to_string(),
        SurfaceType::Escalier => "Escalier".to_string(),
        SurfaceType::Ascenseur => "Ascenseur".to_string(),
        SurfaceType::Room(id) => format!("Salle {}", id),
    }
}

// Heuristique améliorée pour l'A*
fn heuristic(a: (usize, usize), b: (usize, usize)) -> f32 {
    let dx = (a.0 as isize - b.0 as isize).abs();
    let dy = (a.1 as isize - b.1 as isize).abs();
    (dx + dy) as f32 * 0.75
}

// Détermine les voisins accessibles et leur coût
fn get_neighbors(position: (usize, usize), map: &Map) -> Vec<(usize, usize, f32)> {
    let mut neighbors = vec![];

    let moves = [
        (1, 0, 1.0), (-1, 0, 1.0), (0, 1, 1.0), (0, -1, 1.0),
        (1, 1, 1.4), (-1, 1, 1.4), (1, -1, 1.4), (-1, -1, 1.4)
    ];

    for (dx, dy, cost) in moves.iter() {
        let nx = position.0 as isize + dx;
        let ny = position.1 as isize + dy;

        if nx < 0 || ny < 0 || nx as usize >= map.width || ny as usize >= map.height {
            continue;
        }

        let neighbor = (nx as usize, ny as usize);
        let surface = map.get_surface(neighbor.0, neighbor.1);
        let movement_cost = match surface {
            SurfaceType::Mur => continue,
            SurfaceType::Couloir => *cost,
            SurfaceType::Room(_) => *cost,
            SurfaceType::Escalier | SurfaceType::Ascenseur => *cost * 2.0,
            _ => *cost * 1.5,
        };

        neighbors.push((neighbor.0, neighbor.1, movement_cost));
    }

    neighbors
}

// Algorithme A* optimisé
pub fn astar_pathfinding(map: &Map, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<PathSegment>> {
    info!("🔎 Démarrage de A* entre {:?} et {:?}", start, goal);

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut g_score: HashMap<(usize, usize), f32> = HashMap::new();
    let mut explored: HashSet<(usize, usize)> = HashSet::new();

    open_set.push(Node { cost: 0.0, position: start });
    g_score.insert(start, 0.0);

    while let Some(Node { position, .. }) = open_set.pop() {
        if position == goal {
            info!("✅ Chemin trouvé !");
            return Some(reconstruct_path(map, came_from, start, goal));
        }

        if explored.contains(&position) {
            continue;
        }
        explored.insert(position);

        for (nx, ny, move_cost) in get_neighbors(position, map) {
            let neighbor = (nx, ny);
            let tentative_g_score = g_score.get(&position).unwrap_or(&f32::INFINITY) + move_cost;

            if !g_score.contains_key(&neighbor) || tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f32::INFINITY) {
                came_from.insert(neighbor, position);
                g_score.insert(neighbor, tentative_g_score);
                open_set.push(Node {
                    cost: tentative_g_score + heuristic(neighbor, goal),
                    position: neighbor,
                });
            }
        }
    }

    warn!("❌ Aucun chemin trouvé entre {:?} et {:?}", start, goal);
    None
}

// Reconstruction et amélioration des segments
fn reconstruct_path(
    map: &Map,
    came_from: HashMap<(usize, usize), (usize, usize)>,
    start: (usize, usize),
    goal: (usize, usize)
) -> Vec<PathSegment> {
    let mut path = vec![];
    let mut current = goal;
    let mut last_surface = surface_to_string(map.get_surface(goal.0, goal.1));
    let mut segment_start = goal;
    let mut segment_cost = 0.0;
    let mut steps = 0;

    while let Some(&prev) = came_from.get(&current) {
        let surface = surface_to_string(map.get_surface(prev.0, prev.1));
        segment_cost += 1.0;
        steps += 1;

        // Ajout de points intermédiaires tous les 10 cases pour éviter des segments trop longs
        if steps > 10 {
            path.push(PathSegment {
                start: segment_start,
                end: current,
                surface: last_surface.clone(),
                cost: segment_cost,
                line_number: map.get_line_number(current.0, current.1),
            });

            segment_start = prev;
            segment_cost = 0.0;
            last_surface = surface.clone();
            steps = 0;
        }

        current = prev;
    }

    path.push(PathSegment {
        start,
        end: segment_start,
        surface: last_surface,
        cost: segment_cost,
        line_number: map.get_line_number(start.0, start.1),
    });

    path.reverse();
    path
}
















































////derniere version ok au 17/03
// use std::collections::{BinaryHeap, HashMap, HashSet};
// use std::cmp::Ordering;
// use crate::data::loader::{Map, SurfaceType};
// use log::{info, warn};
// use serde::Serialize;

// // Définition du nœud dans la file de priorité
// #[derive(Debug, PartialEq)]
// struct Node {
//     cost: f32,
//     position: (usize, usize),
// }

// impl Eq for Node {}

// impl Ord for Node {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.cost.total_cmp(&other.cost).reverse()
//     }
// }

// impl PartialOrd for Node {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// // Structure d'un segment compressé du chemin
// #[derive(Debug, Clone, Serialize)]
// pub struct CompressedPathStep {
//     pub start: (usize, usize),
//     pub end: (usize, usize),
//     pub surface: String,
//     pub line_start: usize,
//     pub line_end: usize,
//     pub cost: f32,
// }

// // Convertit un `SurfaceType` en String
// fn surface_to_string(surface: SurfaceType) -> String {
//     match surface {
//         SurfaceType::Mur => "Mur".to_string(),
//         SurfaceType::Couloir => "Couloir".to_string(),
//         SurfaceType::Exterieur => "Exterieur".to_string(),
//         SurfaceType::Escalier => "Escalier".to_string(),
//         SurfaceType::Ascenseur => "Ascenseur".to_string(),
//         SurfaceType::Room(id) => format!("Salle {}", id),
//     }
// }

// // Heuristique de distance Manhattan
// fn heuristic(a: (usize, usize), b: (usize, usize), terrain_cost: f32) -> f32 {
//     let manhattan = ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as f32;
//     manhattan * terrain_cost * 0.8
// }

// // Détection des voisins
// fn get_neighbors(position: (usize, usize), map: &Map) -> Vec<(usize, usize, f32)> {
//     let mut neighbors = vec![];

//     let moves = [
//         (1, 0, 1.0), (-1, 0, 1.0), (0, 1, 1.0), (0, -1, 1.0),
//         (1, 1, 1.4), (-1, 1, 1.4), (1, -1, 1.4), (-1, -1, 1.4)
//     ];

//     for (dx, dy, cost) in moves.iter() {
//         let nx = position.0 as isize + dx;
//         let ny = position.1 as isize + dy;

//         if nx < 0 || ny < 0 || nx as usize >= map.width || ny as usize >= map.height {
//             continue;
//         }

//         let neighbor = (nx as usize, ny as usize);
//         let surface = map.get_surface(neighbor.0, neighbor.1);
//         let movement_cost = match surface {
//             SurfaceType::Mur => continue,
//             SurfaceType::Couloir => *cost,
//             SurfaceType::Room(_) => *cost,
//             SurfaceType::Escalier | SurfaceType::Ascenseur => *cost * 2.0,
//             _ => *cost * 1.5,
//         };

//         neighbors.push((neighbor.0, neighbor.1, movement_cost));
//     }

//     neighbors
// }

// // Algorithme A* amélioré
// pub fn astar_pathfinding(map: &Map, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<CompressedPathStep>> {
//     info!("🔎 Démarrage de A* entre {:?} et {:?}", start, goal);

//     let mut open_set = BinaryHeap::new();
//     let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
//     let mut g_score: HashMap<(usize, usize), f32> = HashMap::new();
//     let mut explored: HashSet<(usize, usize)> = HashSet::new();

//     open_set.push(Node { cost: 0.0, position: start });
//     g_score.insert(start, 0.0);

//     while let Some(Node { position, .. }) = open_set.pop() {
//         if position == goal {
//             info!("✅ Chemin trouvé !");
//             return Some(reconstruct_compressed_path(map, came_from, start, goal));
//         }

//         if explored.contains(&position) {
//             continue;
//         }
//         explored.insert(position);

//         for (nx, ny, move_cost) in get_neighbors(position, map) {
//             let neighbor = (nx, ny);
//             let tentative_g_score = g_score.get(&position).unwrap_or(&f32::INFINITY) + move_cost;

//             if !g_score.contains_key(&neighbor) || tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f32::INFINITY) {
//                 came_from.insert(neighbor, position);
//                 g_score.insert(neighbor, tentative_g_score);
//                 open_set.push(Node {
//                     cost: tentative_g_score + heuristic(neighbor, goal, move_cost),
//                     position: neighbor,
//                 });
//             }
//         }
//     }

//     warn!("❌ Aucun chemin trouvé entre {:?} et {:?}", start, goal);
//     None
// }

// // Reconstruction et compression du chemin
// fn reconstruct_compressed_path(
//     map: &Map,
//     came_from: HashMap<(usize, usize), (usize, usize)>,
//     start: (usize, usize),
//     goal: (usize, usize)
// ) -> Vec<CompressedPathStep> {
//     let mut path = vec![];
//     let mut current = goal;
//     let mut segment_start = goal;
//     let mut last_direction = None;
//     let mut segment_cost = 0.0;

//     while let Some(&prev) = came_from.get(&current) {
//         let direction = (current.0 as isize - prev.0 as isize, current.1 as isize - prev.1 as isize);

//         if Some(direction) != last_direction {
//             path.push(CompressedPathStep {
//                 start: segment_start,
//                 end: current,
//                 surface: surface_to_string(map.get_surface(current.0, current.1)),
//                 line_start: map.get_line_number(segment_start.0, segment_start.1),
//                 line_end: map.get_line_number(current.0, current.1),
//                 cost: segment_cost,
//             });
//             segment_start = current;
//             segment_cost = 0.0;
//         }

//         segment_cost += 1.0;
//         last_direction = Some(direction);
//         current = prev;
//     }

//     path.reverse();
//     path
// }


































































































// use std::collections::{BinaryHeap, HashMap, HashSet};
// use std::cmp::Ordering;
// use crate::data::loader::{Map, SurfaceType};
// use log::{info, warn};
// use serde::Serialize; // Pour sérialisation JSON

// // Définition du nœud dans la file de priorité
// #[derive(Debug, PartialEq)]
// struct Node {
//     cost: f32,
//     position: (usize, usize),
// }

// impl Eq for Node {}

// impl Ord for Node {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.cost.total_cmp(&other.cost).reverse()
//     }
// }

// impl PartialOrd for Node {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// // Structure pour stocker chaque étape du chemin
// #[derive(Debug, Clone, Serialize)]
// pub struct PathStep {
//     pub coordinates: (usize, usize),
//     pub surface: String, // Type de surface en texte
//     pub line_number: usize, // Ligne dans le fichier source
//     pub cost: f32, // Coût de déplacement
// }

// // Convertit un `SurfaceType` en String
// fn surface_to_string(surface: SurfaceType) -> String {
//     match surface {
//         SurfaceType::Mur => "Mur".to_string(),
//         SurfaceType::Couloir => "Couloir".to_string(),
//         SurfaceType::Exterieur => "Exterieur".to_string(),
//         SurfaceType::Escalier => "Escalier".to_string(),
//         SurfaceType::Ascenseur => "Ascenseur".to_string(),
//         SurfaceType::Room(id) => format!("Salle {}", id),
//     }
// }

// // Heuristique améliorée : pondération dynamique pour A*
// fn heuristic(a: (usize, usize), b: (usize, usize), terrain_cost: f32) -> f32 {
//     let manhattan = ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as f32;
//     manhattan * terrain_cost * 0.8  // Pondération dynamique
// }

// // Optimisation du calcul des voisins
// fn get_neighbors(position: (usize, usize), map: &Map) -> Vec<(usize, usize, f32)> {
//     let mut neighbors = vec![];

//     let moves = [
//         (1, 0, 1.0), (-1, 0, 1.0), (0, 1, 1.0), (0, -1, 1.0),
//         (1, 1, 1.4), (-1, 1, 1.4), (1, -1, 1.4), (-1, -1, 1.4)
//     ];

//     for (dx, dy, cost) in moves.iter() {
//         let nx = position.0 as isize + dx;
//         let ny = position.1 as isize + dy;

//         if nx < 0 || ny < 0 || nx as usize >= map.width || ny as usize >= map.height {
//             continue;
//         }

//         let neighbor = (nx as usize, ny as usize);
//         let surface = map.get_surface(neighbor.0, neighbor.1);
//         let movement_cost = match surface {
//             SurfaceType::Mur => continue, // Ignore les murs
//             SurfaceType::Couloir => *cost,
//             SurfaceType::Room(_) => *cost,
//             SurfaceType::Escalier | SurfaceType::Ascenseur => *cost * 2.0, // Coût plus élevé pour changement d'étage
//             _ => *cost * 1.5, // Autres surfaces difficiles
//         };

//         neighbors.push((neighbor.0, neighbor.1, movement_cost));
//     }

//     neighbors
// }

// // Algorithme A* optimisé
// pub fn astar_pathfinding(map: &Map, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<PathStep>> {
//     info!("🔎 Démarrage de A* entre {:?} et {:?}", start, goal);

//     let mut open_set = BinaryHeap::new();
//     let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
//     let mut g_score: HashMap<(usize, usize), f32> = HashMap::new();
//     let mut explored: HashSet<(usize, usize)> = HashSet::new();  

//     open_set.push(Node { cost: 0.0, position: start });
//     g_score.insert(start, 0.0);

//     while let Some(Node { position, .. }) = open_set.pop() {
//         if position == goal {
//             info!("✅ Chemin trouvé !");
//             return Some(reconstruct_path(map, came_from, start, goal));
//         }

//         if explored.contains(&position) {
//             continue;
//         }
//         explored.insert(position);

//         for (nx, ny, move_cost) in get_neighbors(position, map) {
//             let neighbor = (nx, ny);
//             let tentative_g_score = g_score.get(&position).unwrap_or(&f32::INFINITY) + move_cost;

//             if !g_score.contains_key(&neighbor) || tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f32::INFINITY) {
//                 came_from.insert(neighbor, position);
//                 g_score.insert(neighbor, tentative_g_score);
//                 open_set.push(Node {
//                     cost: tentative_g_score + heuristic(neighbor, goal, move_cost),
//                     position: neighbor,
//                 });
//             }
//         }
//     }

//     warn!("❌ Aucun chemin trouvé entre {:?} et {:?}", start, goal);
//     None
// }

// // Reconstruction du chemin avec informations supplémentaires
// fn reconstruct_path(
//     map: &Map,
//     came_from: HashMap<(usize, usize), (usize, usize)>,
//     start: (usize, usize),
//     goal: (usize, usize)
// ) -> Vec<PathStep> {
//     let mut path = vec![];
//     let mut current = goal;

//     while let Some(&prev) = came_from.get(&current) {
//         path.push(PathStep {
//             coordinates: current,
//             surface: surface_to_string(map.get_surface(current.0, current.1)),
//             line_number: map.get_line_number(current.0, current.1),
//             cost: 1.0, // Valeur par défaut, ajuster selon la carte
//         });
//         current = prev;
//     }

//     path.push(PathStep {
//         coordinates: start,
//         surface: surface_to_string(map.get_surface(start.0, start.1)),
//         line_number: map.get_line_number(start.0, start.1),
//         cost: 0.0,
//     });

//     path.reverse();
//     path
// }

// // K-Shortest Paths (Renvoie plusieurs chemins possibles)
// pub fn k_shortest_paths(map: &Map, start: (usize, usize), goal: (usize, usize), k: usize) -> Vec<Vec<PathStep>> {
//     let mut paths = Vec::new();

//     for _ in 0..k {
//         if let Some(path) = astar_pathfinding(map, start, goal) {
//             paths.push(path);
//         }
//     }

//     paths
// }





































































































// // src/navigation/astar.rs

// use std::collections::{BinaryHeap, HashMap};
// use std::cmp::Ordering;
// use crate::data::loader::{Map, SurfaceType};
// use log::{info, warn}; // Logs pour le suivi

// #[derive(Debug, PartialEq)]
// struct Node {
//     cost: f32,
//     position: (usize, usize),
// }

// impl Eq for Node {}

// impl Ord for Node {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.cost.total_cmp(&other.cost).reverse() // Utilise total_cmp pour éviter les problèmes de f32
//     }
// }

// impl PartialOrd for Node {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// // Heuristique Manhattan (distance estimée entre le point courant et la destination)
// fn heuristic(a: (usize, usize), b: (usize, usize)) -> f32 {
//     ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as f32
// }

// pub fn astar_pathfinding(map: &Map, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
//     info!("🔎 Démarrage de A* entre {:?} et {:?}", start, goal);

//     let mut open_set = BinaryHeap::new();
//     let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
//     let mut g_score: HashMap<(usize, usize), f32> = HashMap::new();

//     open_set.push(Node { cost: 0.0, position: start });
//     g_score.insert(start, 0.0);

//     while let Some(Node { position, .. }) = open_set.pop() {
//         if position == goal {
//             info!("✅ Chemin trouvé !");
//             let mut path = vec![];
//             let mut current = position;
//             while let Some(&prev) = came_from.get(&current) {
//                 path.push(current);
//                 current = prev;
//             }
//             path.push(start);
//             path.reverse();
//             return Some(path);
//         }

//         let neighbors = [
//             (1, 0), (-1, 0), (0, 1), (0, -1),
//             (1, 1), (-1, 1), (1, -1), (-1, -1)
//         ];

//         for (dx, dy) in neighbors.iter() {
//             let nx = position.0 as isize + dx;
//             let ny = position.1 as isize + dy;

//             if nx < 0 || ny < 0 || nx as usize >= map.width || ny as usize >= map.height {
//                 continue;
//             }

//             let neighbor = (nx as usize, ny as usize);
//             let surface = map.get_surface(neighbor.0, neighbor.1);

//             let movement_cost = match surface {
//                 SurfaceType::Mur => continue, // On ignore les murs
//                 SurfaceType::Couloir => 1.0,
//                 SurfaceType::Room(_) => 1.0,
//                 _ => 1.5, // Escaliers et autres types
//             };

//             let tentative_g_score = g_score.get(&position).unwrap_or(&f32::INFINITY) + movement_cost;

//             info!("🔍 Analyse voisin: {:?}, Type: {:?}, Coût: {:.2}", neighbor, surface, tentative_g_score);

//             if !g_score.contains_key(&neighbor) || tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f32::INFINITY) {
//                 came_from.insert(neighbor, position);
//                 g_score.insert(neighbor, tentative_g_score);
//                 open_set.push(Node {
//                     cost: tentative_g_score + heuristic(neighbor, goal),
//                     position: neighbor,
//                 });
//             }
//         }
//     }

//     warn!("❌ Aucun chemin trouvé entre {:?} et {:?}", start, goal);
//     None
// }
