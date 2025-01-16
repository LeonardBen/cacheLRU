mod cache;

use cache::Cache;

fn main() {
    let mut cache = Cache::new(3);
    println!("Cache en mémoire créé avec une taille de 3");

    // Exemple de mise en cache
    cache.put("A".to_string(), "value_a".to_string());
    cache.put("B".to_string(), "value_b".to_string());
    cache.put("C".to_string(), "value_c".to_string());

    println!("Récupération de 'A': {:?}", cache.get(&"A".to_string()));
    println!("Récupération de 'B': {:?}", cache.get(&"B".to_string()));
    println!("Récupération de 'C': {:?}", cache.get(&"C".to_string()));

    // Test de sauvegarde et lecture dans un fichier
    let file_path = "mon_cache.txt";
    let mut persistent_cache = Cache::new_persistent(3, file_path).unwrap_or_else(|_| Cache::new(3));

    println!("\nCache persistant créé ou chargé depuis {}", file_path);
    persistent_cache.put("A".to_string(), "value_a".to_string());
    persistent_cache.put("B".to_string(), "value_b".to_string());
    persistent_cache.put("C".to_string(), "value_c".to_string());
    persistent_cache.put("D".to_string(), "value_d".to_string());
    persistent_cache.put("E".to_string(), "value_e".to_string());

    persistent_cache.save_to_file(file_path).unwrap(); // Sauvegarder dans le fichier

    // Charger à nouveau le cache persistant et verif des éléments
    let mut reloaded_cache: Cache<String, String> = Cache::new_persistent(3, file_path).unwrap();
    println!("Récupération de 'D' après redémarrage : {:?}", reloaded_cache.get(&"D".to_string()));
    println!("Récupération de 'E' après redémarrage : {:?}", reloaded_cache.get(&"E".to_string()));
    println!("Récupération de 'A' après redémarrage : {:?}", reloaded_cache.get(&"A".to_string()));
    println!("Récupération de 'B' après redémarrage : {:?}", reloaded_cache.get(&"B".to_string()));
    println!("Récupération de 'C' après redémarrage : {:?}", reloaded_cache.get(&"C".to_string()));
}
