# PlantUML Documentation - PGBO Project

Ce dossier contient les diagrammes PlantUML qui documentent l'architecture et les flux de l'application Rust/Rocket avec PostgreSQL.

## Diagrammes Disponibles

### 🏗️ Architecture & Components

- **`architecture.puml`** - Diagramme de séquence principal montrant les interactions entre le client, Rocket, et PostgreSQL
- **`components.puml`** - Vue composants de l'application avec les relations entre modules
- **`api_flows.puml`** - Flux détaillés des API REST pour la gestion des données person_data
- **`database_schema.puml`** - Schéma de base de données montrant les tables person et person_data

### 🧪 Testing & Data

- **`testing_overview.puml`** - Vue d'ensemble de la stratégie de tests (unitaires, intégration)
- **`data_structures.puml`** - Structures de données Rust (PersonData, SqlConfig)

### 📊 Legacy

- **`api_sequence.puml`** - Diagramme de séquence pour les API (version antérieure)

## Structure de l'Application

L'application suit cette architecture :

```
Client (HTTP) → Rocket Server → Database Module → PostgreSQL
```

### Endpoints API

- `GET /ping` - Health check (retourne "alive")
- `GET /test` - Test de connectivité base de données
- `GET /person_data` - Récupère toutes les données person_data
- `GET /person_data/<id>` - Récupère une personne spécifique par ID

### Tables de Base de Données

1. **`person`** - Table simple pour les tests de connectivité
   - `id` (SERIAL PRIMARY KEY)
   - `name` (TEXT NOT NULL)
   - `data` (BYTEA)

2. **`person_data`** - Table complète pour les données personnelles
   - `id` (SERIAL PRIMARY KEY)
   - `first_name`, `last_name` (VARCHAR, NOT NULL)
   - `email` (VARCHAR, UNIQUE, avec validation)
   - `phone`, `birth_date` (optionnels)
   - Champs d'adresse complets
   - Métadonnées (timestamps, statut actif)

## Tests

L'application inclut une suite de tests complète :

- **Tests de connectivité** - Validation de la connexion PostgreSQL
- **Tests d'API** - Validation des endpoints REST
- **Tests d'intégration** - Tests end-to-end

## Génération des Diagrammes

Pour générer les images à partir des fichiers PlantUML :

```bash
# Si vous avez PlantUML installé
plantuml *.puml

# Ou avec Docker
docker run --rm -v $(pwd):/work plantuml/plantuml *.puml
```

## Configuration

L'application utilise :
- **`pgbo_sql.toml`** - Configuration des requêtes SQL et données de test
- **`Cargo.toml`** - Dépendances Rust (Rocket, tokio-postgres, serde, chrono)
- Variables d'environnement pour la connexion DB (`PGBO_DB`)
