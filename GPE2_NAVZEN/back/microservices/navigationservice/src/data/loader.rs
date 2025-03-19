use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use log::{info, error};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SurfaceType {
    Mur,
    Couloir,
    Exterieur,
    Escalier,
    Ascenseur,
    Room(usize),
}

pub struct Map {
    pub grid: Vec<Vec<SurfaceType>>,
    pub width: usize,
    pub height: usize,
    pub line_map: HashMap<(usize, usize), usize>, // Associe une position à un numéro de ligne
}

impl Map {
    /// Charge la carte depuis un fichier `.txt`
    pub fn load_from_file(filepath: &str) -> Self {
        let expected_width = 175;
        let expected_height = 245;

        info!("📂 Chargement de la carte depuis `{}`", filepath);

        let file = match File::open(filepath) {
            Ok(f) => f,
            Err(e) => {
                error!("❌ Erreur d'ouverture du fichier `{}`: {}", filepath, e);
                panic!("Impossible d'ouvrir le fichier.");
            }
        };

        let reader = BufReader::new(file);

        let mut raw_values: Vec<SurfaceType> = Vec::new();
        let mut line_map: HashMap<(usize, usize), usize> = HashMap::new();

        for (line_index, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(l) => l,
                Err(_) => {
                    error!("⚠️ Erreur de lecture de la ligne {}", line_index);
                    continue;
                }
            };

            let values: Vec<f32> = line
                .split_whitespace()
                .filter_map(|v| v.parse::<f32>().ok()) // Évite les erreurs de parsing
                .collect();

            if values.len() != 3 {
                error!("⚠️ Ligne {} ignorée : elle ne contient pas exactement 3 valeurs (RGB)", line_index);
                continue;
            }

            let surface_type = match (values[0], values[1], values[2]) {
                (0.0, 0.0, 1.0) => SurfaceType::Exterieur,
                (1.0, 1.0, 1.0) => SurfaceType::Couloir,
                (0.0, 0.0, 0.0) => SurfaceType::Mur,
                (0.0, x, 0.0) if x > 0.0 => SurfaceType::Room((x * 10.0) as usize),
                _ => {
                    error!("⚠️ Surface inconnue à la ligne {}", line_index);
                    SurfaceType::Mur // Sécurité : on met un mur si inconnu
                }
            };

            raw_values.push(surface_type);
        }

        if raw_values.len() != expected_width * expected_height {
            error!(
                "❌ Taille de la carte incorrecte ! Attendu: {}x{} ({} cases), trouvé: {} cases",
                expected_width, expected_height, expected_width * expected_height, raw_values.len()
            );
        }

        let mut grid = vec![vec![SurfaceType::Mur; expected_width]; expected_height];

        for y in 0..expected_height {
            for x in 0..expected_width {
                let index = y * expected_width + x;
                if index < raw_values.len() {
                    grid[y][x] = raw_values[index];
                    line_map.insert((x, y), index); // Associe chaque (x, y) à sa ligne d'origine
                }
            }
        }

        info!("✅ Carte chargée avec succès : {}x{}", expected_width, expected_height);

        Self { grid, width: expected_width, height: expected_height, line_map }
    }

    /// Retourne le type de surface à une position donnée
    pub fn get_surface(&self, x: usize, y: usize) -> SurfaceType {
        if x >= self.width || y >= self.height {
            error!("❌ Accès hors limites à la carte: ({}, {}) sur {}x{}", x, y, self.width, self.height);
            return SurfaceType::Mur;
        }
        self.grid[y][x]
    }

    /// Retourne le numéro de ligne du fichier source correspondant à une position (x, y)
    pub fn get_line_number(&self, x: usize, y: usize) -> usize {
        *self.line_map.get(&(x, y)).unwrap_or(&0) // Retourne 0 si absent
    }

    /// Vérifie si une case appartient à un couloir large (3 cases de large minimum)
    pub fn is_wide_corridor(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        let mut count = 0;
        let max_range = 3; // Largeur minimale pour être considéré comme un large couloir

        // Vérifie à gauche et à droite
        for i in 1..=max_range {
            if x >= i && matches!(self.get_surface(x - i, y), SurfaceType::Couloir) {
                count += 1;
            }
            if x + i < self.width && matches!(self.get_surface(x + i, y), SurfaceType::Couloir) {
                count += 1;
            }
        }

        // Vérifie en haut et en bas
        for i in 1..=max_range {
            if y >= i && matches!(self.get_surface(x, y - i), SurfaceType::Couloir) {
                count += 1;
            }
            if y + i < self.height && matches!(self.get_surface(x, y + i), SurfaceType::Couloir) {
                count += 1;
            }
        }

        count >= max_range // Si la case a au moins `max_range` cases couloir autour, c'est un large couloir
    }
}

// // src/data/loader.rs
// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use log::{info, error};
// use std::collections::HashMap;

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum SurfaceType {
//     Mur,
//     Couloir,
//     Exterieur,
//     Escalier,
//     Ascenseur,
//     Room(usize),
// }

// pub struct Map {
//     pub grid: Vec<Vec<SurfaceType>>,
//     pub width: usize,
//     pub height: usize,
//     pub line_map: HashMap<(usize, usize), usize>, // Associe une position à un numéro de ligne
// }

// impl Map {
//     pub fn load_from_file(filepath: &str) -> Self {
//         let expected_width = 175;
//         let expected_height = 245;

//         info!("📂 Chargement de la carte depuis `{}`", filepath);

//         let file = File::open(filepath).expect("Erreur de lecture du fichier SurfaceInfo.txt");
//         let reader = BufReader::new(file);

//         let mut raw_values: Vec<SurfaceType> = Vec::new();
//         let mut line_map: HashMap<(usize, usize), usize> = HashMap::new();

//         for (line_index, line) in reader.lines().enumerate() {
//             let line = line.unwrap();
//             let values: Vec<f32> = line
//                 .split_whitespace()
//                 .map(|v| v.parse::<f32>().unwrap_or_else(|_| {
//                     error!("❌ Erreur de parsing ligne {}: `{}`", line_index, v);
//                     0.0
//                 }))
//                 .collect();

//             if values.len() != 3 {
//                 error!("⚠️ Ligne {} ignorée : elle ne contient pas exactement 3 valeurs (RGB)", line_index);
//                 continue;
//             }

//             let surface_type = match (values[0], values[1], values[2]) {
//                 (0.0, 0.0, 1.0) => SurfaceType::Exterieur,
//                 (1.0, 1.0, 1.0) => SurfaceType::Couloir,
//                 (0.0, 0.0, 0.0) => SurfaceType::Mur,
//                 (0.0, x, 0.0) if x > 0.0 => SurfaceType::Room((x * 10.0) as usize),
//                 _ => SurfaceType::Mur,
//             };

//             raw_values.push(surface_type);
//         }

//         if raw_values.len() != expected_width * expected_height {
//             error!("❌ Taille de la carte incorrecte ! Attendu: {}x{} ({} cases), trouvé: {} cases",
//                 expected_width, expected_height, expected_width * expected_height, raw_values.len());
//         }

//         let mut grid = vec![vec![SurfaceType::Mur; expected_width]; expected_height];

//         for y in 0..expected_height {
//             for x in 0..expected_width {
//                 let index = y * expected_width + x;
//                 grid[y][x] = raw_values[index];

//                 // Associer chaque (x, y) à la ligne du fichier `.txt`
//                 line_map.insert((x, y), index);
//             }
//         }

//         info!("✅ Carte chargée avec succès : {}x{}", expected_width, expected_height);

//         Self { grid, width: expected_width, height: expected_height, line_map }
//     }

//     pub fn get_surface(&self, x: usize, y: usize) -> SurfaceType {
//         if x >= self.width || y >= self.height {
//             error!("❌ Accès hors limites à la carte: ({}, {}) sur {}x{}", x, y, self.width, self.height);
//             return SurfaceType::Mur;
//         }
//         self.grid[y][x]
//     }

//     pub fn get_line_number(&self, x: usize, y: usize) -> usize {
//         *self.line_map.get(&(x, y)).unwrap_or(&0) // Retourne 0 si absent
//     }
// }
