use clap::Parser;
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    language: Option<String>,
    
    #[arg(short, long)]
    extension: Option<String>,

    #[arg(short, long)]
    action: Option<String>,
}

fn main() {
    let args = Args::parse();

    println!("Hello, {}!", args.name);

    if let Some(language) = args.language {
        println!("You are using {} language.", language);
    }

    if let Some(extension) = args.extension {
        println!("Output file is {}.", extension);
    }

    if let Some(action) = args.action {
        println!("Action is {}.", action);
    }

    let yaml_str = "
        name: Example
        version: 1.0
    ";
    let docs = YamlLoader::load_from_str(yaml_str).unwrap();
    let doc = &docs[0];

    println!("Name: {}", doc["name"].as_str().unwrap());

    println!("-------------------");

    let mut yaml_data = Yaml::Hash(Default::default());

    if let Yaml::Hash(ref mut hash) = yaml_data {
        use yaml_rust::yaml::Hash;
        use yaml_rust::Yaml;

        hash.insert(Yaml::String("name".to_string()), Yaml::String("Example Project".to_string()));
        hash.insert(Yaml::String("version".to_string()), Yaml::String("1.0.0".to_string()));
        
        let mut dependencies = Hash::new();
        dependencies.insert(Yaml::String(("anyhow").to_string()), Yaml::String("1.0".to_string()));
        dependencies.insert(Yaml::String("serde".to_string()), Yaml::String("1.0".to_string()));
        dependencies.insert(Yaml::String("yaml-rust".to_string()), Yaml::String("0.4".to_string()));
        
        hash.insert(Yaml::String("dependencies".to_string()), Yaml::Hash(dependencies));

        let mut dev_dependencies = Hash::new();
        dev_dependencies.insert(Yaml::String("clap".to_string()), Yaml::String("2.33".to_string()));
        dev_dependencies.insert(Yaml::String("serde".to_string()), Yaml::String("1.0".to_string()));

        hash.insert(Yaml::String("dev-dependencies".to_string()), Yaml::Hash(dev_dependencies));
    }

    // Convert the YAML structure to a string
    let mut yaml_string = String::new();
    let mut emitter = YamlEmitter::new(&mut yaml_string);
    emitter.dump(&yaml_data).unwrap(); // Serialize the YAML structure

    // Print the generated YAML
    println!("{}", yaml_string);

}
