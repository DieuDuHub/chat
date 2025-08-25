#!/bin/bash

# Script pour tester l'API PGBO avec des données de test
# Usage: ./test_api.sh

API_BASE="http://localhost:8002"

echo "🧪 Test de l'API PGBO"
echo "===================="

# Test de ping
echo "📡 Test de connectivité..."
curl -s "$API_BASE/ping" && echo

echo ""

# Test de récupération des données (peut être vide au début)
echo "📋 Récupération des données existantes..."
curl -s "$API_BASE/person_data" | python3 -m json.tool

echo ""

# Création de données de test
echo "➕ Création de données de test..."

# Personne 1
echo "Création de Matthieu..."
curl -X POST "$API_BASE/person_data" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Matthieu",
    "last_name": "Debray",
    "email": "matthieu.debray@example.com",
    "phone": "+33 6 12 34 56 78",
    "birth_date": "1985-05-15",
    "gender": "M",
    "street_address": "123 Rue de la Paix",
    "city": "Paris",
    "postal_code": "75001",
    "country": "France",
    "nationality": "Française",
    "occupation": "Développeur",
    "company": "TechCorp",
    "salary": 65000,
    "marital_status": "Célibataire",
    "is_active": true
  }' | python3 -m json.tool

echo ""

# Personne 2
echo "Création de Sophie..."
curl -X POST "$API_BASE/person_data" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Sophie",
    "last_name": "Martin",
    "email": "sophie.martin@example.com",
    "phone": "+33 6 98 76 54 32",
    "birth_date": "1990-08-22",
    "gender": "F",
    "street_address": "456 Avenue des Champs",
    "city": "Lyon",
    "postal_code": "69000",
    "country": "France",
    "nationality": "Française",
    "occupation": "Designer UX",
    "company": "DesignStudio",
    "salary": 58000,
    "marital_status": "Mariée",
    "is_active": true
  }' | python3 -m json.tool

echo ""

# Personne 3
echo "Création de Jean..."
curl -X POST "$API_BASE/person_data" \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "Jean",
    "last_name": "Dupont",
    "email": "jean.dupont@example.com",
    "phone": "+33 6 11 22 33 44",
    "birth_date": "1978-12-03",
    "gender": "M",
    "street_address": "789 Boulevard Saint-Germain",
    "city": "Marseille",
    "postal_code": "13000",
    "country": "France",
    "nationality": "Française",
    "occupation": "Chef de projet",
    "company": "ProjectCorp",
    "salary": 72000,
    "marital_status": "Divorcé",
    "is_active": false
  }' | python3 -m json.tool

echo ""

# Récupération finale des données
echo "📋 Récupération des données après création..."
curl -s "$API_BASE/person_data" | python3 -m json.tool

echo ""
echo "✅ Tests terminés !"
