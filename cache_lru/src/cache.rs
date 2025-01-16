use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, Read, Write};
use std::hash::Hash;
use std::path::Path;

pub struct Cache<K, V>// Def de la struct Cache avec deux paramètres génériques (K , V )
where
    K: Eq + Hash, 
    V: ToString,
{
    capacity: usize,
    cache_map: HashMap<K, usize>,
    cache_list: VecDeque<(K, V)>,
    file_path: Option<String>,
}

impl Cache<String, String> {  //Implementation de la struct Cache avec deux paramètres génériques (String , String )
    //cache mémoire avec une capacité maximale
    pub fn new(capacity: usize) -> Self {
        Cache {
            capacity,
            cache_map: HashMap::new(),
            cache_list: VecDeque::new(),
            file_path: None,
        }
    }

    //cache persistant avec une capacité maximale et un fichier pour enregistrer les données
    pub fn new_persistent(capacity: usize, file_path: &str) -> io::Result<Self> {
        let mut cache = Cache {
            capacity,
            cache_map: HashMap::new(),
            cache_list: VecDeque::new(),
            file_path: Some(file_path.to_string()),
        };

        // Charger les données existantes depuis le fichier
        if Path::new(file_path).exists() {
            cache.load_from_file(file_path)?;
        }

        Ok(cache)
    }

    // Ajout / maj d'un élément dans le cache
    pub fn put(&mut self, key: String, value: String) {
        if self.cache_map.contains_key(&key) {
            // Si l'élément existe déjà --> réorganiser l'ordre
            let pos = self.cache_map[&key];
            self.cache_list.remove(pos);
            self.cache_list.push_front((key.clone(), value.clone()));
        } else {
            if self.cache_list.len() == self.capacity {
                // Expulsion de l'élément LRU
                let (evicted_key, _) = self.cache_list.pop_back().unwrap();
                self.cache_map.remove(&evicted_key);
            }
            // Ajouter le nouvel élément
            self.cache_list.push_front((key.clone(), value.clone()));
        }

        // vider la cahe map et mettre à jour les positions
        self.cache_map.clear();
        for (index, (k, _)) in self.cache_list.iter().enumerate() {
            self.cache_map.insert(k.clone(), index);
        }

        // Si un fichier est défini, sauvegarder l'état du cache
        if let Some(file_path) = &self.file_path {
            self.save_to_file(file_path).unwrap(); 
        }
    }

    // Récupération d'un élément du cache
   pub fn get(&mut self, key: &String) -> Option<&String> {
        if let Some(position) = self.cache_map.get(key) {
            // Marquer l'élément comme récemment utilisé (réorganiser l'ordre)
            let value = self.cache_list.remove(*position);
            self.cache_list.push_front(value.unwrap());  // Déplacer l'élément au début

            // Mettre à jour le cache_map pour
            self.cache_map.clear();
            for (index, (k, _)) in self.cache_list.iter().enumerate() {
                self.cache_map.insert(k.clone(), index);  // Remettre la position dans la map
            }

            Some(&self.cache_list[0].1)  // Retourner la valeur du premier élément
        } else {
            None
        }
    }

    // Sauvegarde du cache dans un fichier
    pub fn save_to_file(&self, file_path: &str) -> io::Result<()> {
        let mut file = File::create(file_path)?;
        for (key, value) in &self.cache_list {
            writeln!(file, "{}={}", key, value)?;
        }
        Ok(())
    }

    // Chargement du cache depuis un fichier
    fn load_from_file(&mut self, file_path: &str) -> io::Result<()> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        for line in contents.lines() {
            let mut split = line.splitn(2, '=');
            if let (Some(key), Some(value)) = (split.next(), split.next()) {
                self.put(key.to_string(), value.to_string());
            }
        }
        Ok(())
    }
}
