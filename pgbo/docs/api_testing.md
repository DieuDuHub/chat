# API Testing Guide - PGBO

Ce document contient toutes les requêtes curl pour tester l'API PGBO.

## Prérequis

1. Démarrer le serveur PGBO :
```bash
cd /Users/matthieudebray/dev/rust/chat/pgbo
cargo run
```

2. Le serveur écoute sur `http://localhost:8000`

## Endpoints disponibles

### 1. Test de connectivité simple
**GET** `/ping`

```bash
curl http://localhost:8000/ping
```

**Réponse attendue :**
```
alive
```

### 2. Test de connectivité avec création de tables
**GET** `/test`

```bash
curl http://localhost:8000/test
```

**Réponse attendue :**
```
Database connectivity test passed
```

### 3. Lire toutes les personnes
**GET** `/person_data`

```bash
curl http://localhost:8000/person_data
```

**Réponse attendue :**
```json
{
  "Ok": [
    {
      "id": 1,
      "first_name": "Jean",
      "last_name": "Dupont",
      "email": "jean.dupont@test.com",
      "phone": "+33123456789",
      "birth_date": "1990-05-15",
      "gender": "M",
      "street_address": "123 Rue de la Paix",
      "city": "Paris",
      "state_province": "Île-de-France",
      "postal_code": "75001",
      "country": "France",
      "nationality": "French",
      "occupation": "Engineer",
      "company": "Tech Corp",
      "salary": 65000.0,
      "marital_status": "Single",
      "created_at": "2025-08-19T09:25:30.135028Z",
      "updated_at": "2025-08-19T09:25:30.135028Z",
      "is_active": true
    }
  ]
}
```

### 4. Lire une personne par ID
**GET** `/person_data/<id>`

```bash
# Lire la personne avec ID 1
curl http://localhost:8000/person_data/1

# Lire une personne inexistante
curl http://localhost:8000/person_data/999
```

**Réponse attendue (personne trouvée) :**
```json
{
  "Ok": {
    "id": 1,
    "first_name": "Jean",
    "last_name": "Dupont",
    "email": "jean.dupont@test.com",
    "phone": "+33123456789",
    "birth_date": "1990-05-15",
    "gender": "M",
    "street_address": "123 Rue de la Paix",
    "city": "Paris",
    "state_province": "Île-de-France",
    "postal_code": "75001",
    "country": "France",
    "nationality": "French",
    "occupation": "Engineer",
    "company": "Tech Corp",
    "salary": 65000.0,
    "marital_status": "Single",
    "created_at": "2025-08-19T09:25:30.135028Z",
    "updated_at": "2025-08-19T09:25:30.135028Z",
    "is_active": true
  }
}
```

**Réponse attendue (personne non trouvée) :**
```json
{
  "Ok": null
}
```

### 5. Créer une nouvelle personne
**POST** `/person_data`

#### 5.1 Création avec données complètes

```bash
curl -X POST http://localhost:8000/person_data \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Jean",
    "last_name": "Dupont",
    "email": "jean.dupont@test.com",
    "phone": "+33123456789",
    "birth_date": "1990-05-15",
    "gender": "M",
    "street_address": "123 Rue de la Paix",
    "city": "Paris",
    "state_province": "Île-de-France",
    "postal_code": "75001",
    "country": "France",
    "nationality": "French",
    "occupation": "Engineer",
    "company": "Tech Corp",
    "salary": 65000.0,
    "marital_status": "Single"
  }'
```

#### 5.2 Création avec données minimales

```bash
curl -X POST http://localhost:8000/person_data \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Marie",
    "last_name": "Martin",
    "email": "marie.martin@test.com"
  }'
```

#### 5.3 Création d'une deuxième personne

```bash
curl -X POST http://localhost:8000/person_data \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Marie",
    "last_name": "Martin",
    "email": "marie.martin@test.com",
    "phone": "+33987654321",
    "birth_date": "1985-12-10",
    "gender": "F",
    "street_address": "456 Avenue des Champs",
    "city": "Lyon",
    "state_province": "Auvergne-Rhône-Alpes",
    "postal_code": "69000",
    "country": "France",
    "nationality": "French",
    "occupation": "Designer",
    "company": "Design Studio",
    "salary": 55000.0,
    "marital_status": "Married"
  }'
```

**Réponse attendue (succès) :**
```json
{
  "Ok": {
    "id": 1,
    "first_name": "Jean",
    "last_name": "Dupont",
    "email": "jean.dupont@test.com",
    "phone": "+33123456789",
    "birth_date": "1990-05-15",
    "gender": "M",
    "street_address": "123 Rue de la Paix",
    "city": "Paris",
    "state_province": "Île-de-France",
    "postal_code": "75001",
    "country": "France",
    "nationality": "French",
    "occupation": "Engineer",
    "company": "Tech Corp",
    "salary": 65000.0,
    "marital_status": "Single",
    "created_at": "2025-08-19T09:25:30.135028Z",
    "updated_at": "2025-08-19T09:25:30.135028Z",
    "is_active": true
  }
}
```

## Tests d'erreur

### 1. JSON invalide

```bash
curl -X POST http://localhost:8000/person_data \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Jean",
    "last_name": "Dupont"
    // JSON invalide
  }'
```

**Réponse attendue :** HTTP 400 Bad Request

### 2. Champs manquants

```bash
curl -X POST http://localhost:8000/person_data \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Jean"
  }'
```

**Réponse attendue :** HTTP 422 Unprocessable Entity

### 3. Email déjà existant

```bash
# Créer la première personne
curl -X POST http://localhost:8000/person_data \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Jean",
    "last_name": "Dupont",
    "email": "jean.dupont@test.com"
  }'

# Tenter de créer une personne avec le même email
curl -X POST http://localhost:8000/person_data \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Pierre",
    "last_name": "Durand",
    "email": "jean.dupont@test.com"
  }'
```

**Réponse attendue (erreur) :**
```json
{
  "Err": "Failed to create person data: db error: ERROR: duplicate key value violates unique constraint \"person_data_email_key\""
}
```

## Formatage avec jq

Pour une meilleure lisibilité des réponses JSON, utilisez `jq` :

```bash
# Lire toutes les personnes avec formatage
curl http://localhost:8000/person_data | jq

# Créer une personne avec formatage
curl -X POST http://localhost:8000/person_data \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Alice",
    "last_name": "Wonderland",
    "email": "alice@wonderland.com"
  }' | jq
```

## Workflow de test complet

```bash
# 1. Démarrer le serveur
cargo run &

# 2. Attendre le démarrage
sleep 3

# 3. Test de connectivité
curl http://localhost:8000/ping

# 4. Initialiser les tables
curl http://localhost:8000/test

# 5. Créer des personnes
curl -X POST http://localhost:8000/person_data \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Jean",
    "last_name": "Dupont",
    "email": "jean.dupont@test.com"
  }'

curl -X POST http://localhost:8000/person_data \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Marie",
    "last_name": "Martin",
    "email": "marie.martin@test.com"
  }'

# 6. Lire toutes les personnes
curl http://localhost:8000/person_data | jq

# 7. Lire une personne spécifique
curl http://localhost:8000/person_data/1 | jq

# 8. Arrêter le serveur
kill %1
```

## Schéma des données PersonData

### Champs requis
- `first_name` (string) : Prénom
- `last_name` (string) : Nom de famille
- `email` (string) : Email (unique)

### Champs optionnels
- `phone` (string) : Numéro de téléphone
- `birth_date` (string, format: YYYY-MM-DD) : Date de naissance
- `gender` (string) : Genre ("M", "F", "Male", "Female", "Other", "Prefer not to say")
- `street_address` (string) : Adresse postale
- `city` (string) : Ville
- `state_province` (string) : État/Province
- `postal_code` (string) : Code postal
- `country` (string) : Pays
- `nationality` (string) : Nationalité
- `occupation` (string) : Profession
- `company` (string) : Entreprise
- `salary` (number) : Salaire
- `marital_status` (string) : Statut marital ("Single", "Married", "Divorced", "Widowed", "Other")
- `is_active` (boolean) : Statut actif (défaut: true)

### Champs automatiques
- `id` (number) : Identifiant unique (auto-généré)
- `created_at` (string, ISO 8601) : Date de création
- `updated_at` (string, ISO 8601) : Date de dernière modification
