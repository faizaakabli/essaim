# Documentation pour les Tests du Jeu en Rust

Ce document est fait pour l'exécution de tests unitaires pour les différentes fonctionnalités de notre jeu

## Fichier de Tests

vous trouverez un fichier `tests.rs` et dans le dossier tests (integration)

### Description des Tests

Ce fichier `tests.rs` contient des tests unitaires pour les fonctionnalités suivantes :

1. **Mouvement du robot** : Vérifie que le robot se déplace correctement.
2. **Collecte des étoiles** : Vérifie que le robot collecte des étoiles et que le score est mis à jour correctement.

### Exécution des Tests

Pour exécuter les tests, utilisez la commande suivante dans le terminal à la racine de votre projet :

```sh
cargo test
```

### Configuration du Projet pour les Tests

Assurez-vous que votre projet est configuré pour inclure les tests en ajoutant le module de test dans votre fichier principal `main.rs` :

```rust
#[cfg(test)]
mod tests;
```

Cela garantira que les tests sont exécutés correctement lorsque vous utilisez `cargo test`.
