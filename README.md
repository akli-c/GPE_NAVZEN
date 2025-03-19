# 🌍 GPE2_NAVZEN - Architecture et Bonnes Pratiques  

## 🏗️ Introduction  

GPE2_NAVZEN est un projet de navigation intérieure conçu avec une architecture **microservices**.  
Le projet regroupe **le front-end, le back-end, la base de données, la documentation et l’intégration continue** dans un **monorepo** hébergé sur GitLab.  

Ce document présente **l’architecture globale**, **les bonnes pratiques**, et **l’utilité des différents dossiers** du projet.  

---

## 📁 **Structure du Projet**  



---

## 🎯 **Philosophie du Projet**  

1. **📦 Monorepo** : Tout est centralisé dans un seul dépôt GitLab pour faciliter la gestion et les mises à jour.  
2. **⚡ Architecture Microservices** : Chaque service est indépendant et peut être déployé séparément.  
3. **🔄 CI/CD GitLab** : Automatisation du build, des tests et du déploiement.  
4. **🛠️ Code Qualitatif** : Utilisation de bonnes pratiques (Clean Code, linting, tests).  
5. **📜 Documentation Intégrée** : Stockée en **Markdown** pour être facilement consultable.  
6. **🌍 Scalabilité** : Possibilité d'ajouter des services et d’intégrer Docker/Kubernetes à l’avenir.  

---

## 📌 **Description des Dossiers**  

### 📂 **/autres/**  
Contient des fichiers annexes ou scripts qui ne rentrent pas dans les autres catégories.  

### 📂 **/back/**  
Le backend est divisé en **API Gateway** (Symfony) et plusieurs **microservices** indépendants (Symfony & Rust).  

- **gateway/** : Point d’entrée principal qui route les requêtes vers les microservices.  
- **microservices/** : Chaque microservice a son propre dossier.  

📌 **Bonnes pratiques Backend** :  
✔ Chaque microservice doit être **indépendant**.  
✔ Utiliser **les conventions REST** et **GraphQL** si nécessaire.  
✔ Ajouter des **tests unitaires et d’intégration** avant chaque merge.  

---

### 📂 **/ci-cd/**  
Contient la configuration de **GitLab CI/CD** pour :  
- **Linting** (vérification du code).  
- **Tests unitaires & intégration**.  
- **Build et déploiement automatique**.  

📌 **Bonnes pratiques CI/CD** :  
✔ Automatiser les tests pour éviter les régressions.  
✔ Exécuter les builds en parallèle pour accélérer le déploiement.  

---

### 📂 **/database/**  
- **schemas/** : Contient les modèles de la base de données.  
- **migrations/** : Scripts pour gérer les évolutions de la base.  

📌 **Bonnes pratiques Database** :  
✔ Versionner la base avec des migrations.  
✔ Séparer **les données sensibles** (fichiers `.env`).  

---

### 📂 **/docs/**  
Documentation du projet.  

- **API/** : Explication des endpoints REST/GraphQL.  
- **architecture/** : Schémas UML et diagrammes.  

📌 **Bonnes pratiques Documentation** :  
✔ Toujours mettre à jour la doc après un changement majeur.  
✔ Utiliser des **diagrammes UML** pour l’architecture.  

---

### 📂 **/front/**  
Application React.js (TypeScript, Tailwind).  

📌 **Bonnes pratiques Frontend** :  
✔ Utiliser **un state manager** (Redux, Zustand…).  
✔ Organiser le code en **composants réutilisables**.  
✔ Respecter **les principes d’accessibilité** (a11y).  

---

### 📂 **/scripts/**  
Scripts d'automatisation :  
- **build.sh** : Compilation et mise en production.  
- **deploy.sh** : Déploiement automatique.  

📌 **Bonnes pratiques Scripts** :  
✔ Éviter les scripts trop longs.  
✔ Documenter chaque script avec des commentaires.  

---

### 📂 **/tests/**  
- **unit/** : Tests unitaires (chaque composant individuel).  
- **integration/** : Tests d’interaction entre services.  
- **e2e/** : Tests End-to-End simulant un utilisateur réel.  

📌 **Bonnes pratiques Tests** :  
✔ Exécuter **les tests à chaque commit** avec CI/CD.  
✔ Écrire **des tests clairs et courts** (AAA : Arrange, Act, Assert).  
✔ Couvrir **80% du code** avec des tests unitaires.  

---

## 🏆 **Bonnes Pratiques Générales**  

✅ **Respecter les conventions de code** (PSR pour PHP, Rustfmt pour Rust, Prettier pour React).  
✅ **Utiliser Git Flow** pour organiser le développement :  
   - `main` → Code stable en production.  
   - `dev` → Dernière version en développement.  
   - `feature/*` → Développement d’une nouvelle fonctionnalité.  
✅ **Faire des revues de code avant tout merge**.  
✅ **Écrire une documentation claire pour chaque module**.  
✅ **Optimiser les performances** (ex. caching, DB indexing).  

---

## 🚀 **Développement et Déploiement**  

### 📌 **Installation du projet**  
```bash
git clone https://gitlab.com/ton_projet/GPE2_NAVZEN.git
cd GPE2_NAVZEN
