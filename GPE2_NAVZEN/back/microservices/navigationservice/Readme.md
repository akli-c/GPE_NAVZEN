





navigationservice/
│── Cargo.toml                  # Dépendances et configuration Rust
│── .env                        # Variables d'environnement (PORT, fichiers, etc.)
│── .gitignore                  # Ignore les fichiers non nécessaires
│── README.md                   # Documentation du projet
│
│── src/
│   ├── main.rs                 # Point d'entrée de l'application
│   │
│   ├── config/                 # Configuration et gestion des paramètres
│   │   ├── mod.rs              # Module principal
│   │   ├── settings.rs         # Lecture des variables d'environnement et paramètres globaux
│   │
│   ├── server/                 # Serveur et gestion des routes API
│   │   ├── mod.rs              # Module principal
│   │   ├── app.rs              # Initialisation du serveur et enregistrement des routes
│   │   ├── routes/             # Dossier contenant les handlers des différentes routes
│   │   │   ├── mod.rs          # Module principal des routes
│   │   │   ├── navigation.rs   # Gestion de l'API de navigation
│   │   │   ├── localization.rs # Gestion de l'API de localisation
│   │
│   ├── navigation/             # Algorithmes et logique de navigation
│   │   ├── mod.rs              # Module principal
│   │   ├── astar.rs            # Implémentation de l'algorithme A*
│   │   ├── localization.rs     # Algorithme de localisation
│   │
│   ├── data/                   # Gestion des fichiers et structures de données
│   │   ├── mod.rs              # Module principal
│   │   ├── loader.rs           # Chargement et parsing des fichiers (PNG + SurfaceInfo.txt)
│   │   ├── surface.rs          # Gestion et interprétation des surfaces et zones
│   │
│   ├── services/               # Services métier (logique applicative)
│   │   ├── mod.rs              # Module principal
│   │   ├── pathfinding.rs      # Service qui appelle A* et gère la validation des chemins
│   │   ├── location_service.rs # Service qui analyse la localisation
│   │
│   ├── utils/                  # Fonctions utilitaires et helpers généraux
│   │   ├── mod.rs              # Module principal
│   │   ├── logger.rs           # Gestion des logs
│   │   ├── errors.rs           # Gestion centralisée des erreurs
│   │
│── data/                       # Stockage des fichiers de carte
│   ├── map.png                 # Image de la carte
│   ├── SurfaceInfo.txt         # Fichier des valeurs de surface
│
│── tests/                      # Tests d'intégration et unitaires
│   ├── mod.rs                  # Module principal des tests
│   ├── api_tests.rs            # Tests des endpoints API
│   ├── astar_tests.rs          # Tests unitaires de l'algorithme A*
│   ├── localization_tests.rs   # Tests unitaires de la localisation
│   ├── loader_tests.rs         # Tests de chargement de fichier
