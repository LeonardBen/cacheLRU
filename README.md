# CacheLRU

le repo CacheLRU est une implémentation simple du cache LRU (Least Recently Used) dans Rust. Ce cache est conçu pour stocker un nombre limité d'éléments (3) et pour supprimer automatiquement les éléments les moins récemment utilisés lorsque le cache atteint sa capacité.

## Installation

Clonez ce dépôt :

```bash
git clone <https://github.com/LeonardBen/cacheLRU>
cd <cache_lru>
```

## Structure du projet

```
.
├── src
│   ├── cache.rs        # Contient l’implémentation de la structure Cache
│   ├── main.rs         # Exemple d’utilisation du cache
│   └── lib.rs          # Point d’entrée pour la bibliothèque
├── tests
│   └── cache_tests.rs  # Tests unitaires et d’intégration
├── Cargo.toml          # Dépendances et configuration du projet
└── README.md           # Documentation du projet
```

## Utilisation

Voici un exemple d'utilisation du CacheLRU :

```rust
mod cache;

use cache::Cache;

fn main() {
    let mut cache = Cache::new(3);
    println!("Cache en mémoire créé avec une taille de 3");

    // Ajouter des éléments au cache
    cache.put("A".to_string(), "value_a".to_string());
    cache.put("B".to_string(), "value_b".to_string());
    cache.put("C".to_string(), "value_c".to_string());

    // Récupération d'une valeur
    println!("Récupération de 'A': {:?}", cache.get(&"A".to_string()));
}
```

Exécutez le programme avec la commande suivante :

```bash
cargo run
```

### Tests

Pour exécuter les tests unitaires et d’intégration, utilisez :

```bash
cargo test
```

### Cache persistant

Le cache peut être sauvegardé dans un fichier et rechargé :

```rust
let file_path = "cache_data.txt";
let mut cache = Cache::new_persistent(3, file_path).unwrap_or_else(|_| Cache::new(3));

cache.put("key1".to_string(), "value1".to_string());
cache.save_to_file(file_path).unwrap();

let reloaded_cache = Cache::new_persistent(3, file_path).unwrap();
println!("Récupération : {:?}", reloaded_cache.get(&"key1".to_string()));
```

## Auteur

- [@LeonardBEN](https://github.com/LeonardBen)