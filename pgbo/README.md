# PGBO - PostgreSQL Backend with Rocket

Application Rust utilisant le framework web Rocket avec PostgreSQL pour la gestion des données personnelles.

## Structure du Projet

```
pgbo/
├── src/
│   ├── main.rs          # Point d'entrée Rocket avec les endpoints API
│   ├── database.rs      # Module de gestion de la base de données
│   └── tests.rs         # Tests unitaires et d'intégration
├── docs/                # Documentation PlantUML
│   ├── *.puml          # Diagrammes d'architecture
│   └── README_PLANTUML.md
├── conf/                # Configuration
│   ├── pgbo_sql.toml   # Requêtes SQL et configuration
│   └── pgbo_sql_invalid.toml
└── Cargo.toml          # Configuration et dépendances Rust
```

## API Endpoints

- `GET /ping` - Health check (retourne "alive")
- `GET /test` - Test de connectivité base de données
- `GET /person_data` - Récupère toutes les données person_data (JSON)
- `GET /person_data/<id>` - Récupère une personne spécifique par ID (JSON)

## Démarrage Rapide

```bash
# Installation des dépendances
cargo build

# Lancement des tests
cargo test

# Démarrage du serveur
cargo run
```

## Configuration

L'application utilise la variable d'environnement `PGBO_DB` pour la connexion PostgreSQL :
```bash
export PGBO_DB="host=localhost dbname=md user=postgres password=password"
```

Par défaut : `host=localhost dbname=md`

## Documentation

La documentation complète de l'architecture est disponible dans le dossier [`docs/`](docs/) avec les diagrammes PlantUML.

## Technologies

- **Rust** - Langage de programmation
- **Rocket 0.5.1** - Framework web avec support JSON
- **tokio-postgres** - Client PostgreSQL asynchrone
- **serde** - Sérialisation/désérialisation JSON
- **chrono** - Gestion des dates et heures
