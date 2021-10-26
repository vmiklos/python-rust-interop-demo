use std::collections::HashMap;

fn main() {
    let mut cache: HashMap<String, serde_json::Value> = HashMap::new();
    let argv: Vec<String> = std::env::args().collect();
    let datadir = argv[1].clone();
    let entries = std::fs::read_dir(&datadir).unwrap();
    let mut yaml_paths: Vec<String> = Vec::new();
    for entry in entries {
        let path = entry.unwrap().path();
        let path = path.to_str().unwrap();
        if path.ends_with(".yaml") {
            yaml_paths.push(path.to_string());
        }
    }
    yaml_paths.sort();
    for yaml_path in yaml_paths {
        let cache_key = std::path::Path::new(&yaml_path)
            .strip_prefix(&datadir)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let data = std::fs::read_to_string(&yaml_path).unwrap();
        let cache_value = serde_yaml::from_str::<serde_json::Value>(&data).unwrap();
        cache.insert(cache_key, cache_value);
    }

    let cache_path = format!("{}/yamls.cache", datadir);
    {
        let write_stream = std::fs::File::create(&cache_path).unwrap();
        serde_json::to_writer(&write_stream, &cache).unwrap();
    }

    let workdir = argv[2].clone();
    let yaml_path = format!("{}/relations.yaml", datadir);
    let mut relation_ids: Vec<u64> = Vec::new();
    let stream = std::fs::File::open(yaml_path).unwrap();
    let relations: serde_yaml::Value = serde_yaml::from_reader(stream).unwrap();
    for (_key, value) in relations.as_mapping().unwrap() {
        relation_ids.push(value["osmrelation"].as_u64().unwrap());
    }
    relation_ids.sort();
    relation_ids.dedup();
    let statsdir = format!("{}/stats", workdir);
    std::fs::create_dir_all(&statsdir).unwrap();
    {
        let write_stream = std::fs::File::create(&format!("{}/relations.json", statsdir)).unwrap();
        serde_json::to_writer(&write_stream, &relation_ids).unwrap();
    }
}
