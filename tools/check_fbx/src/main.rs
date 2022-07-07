// Adopted from fbxcel-dom example

fn main() {
    env_logger::init();

    let path = match std::env::args_os().nth(1) {
        Some(v) => std::path::PathBuf::from(v),
        None => {
            eprintln!("Usage: load: <FBX_FILE>");

            std::process::exit(1);
        }
    };

    let file = std::fs::File::open(path).expect("Failed to open file");
    let reader = std::io::BufReader::new(file);

    if let fbxcel_dom::any::AnyDocument::V7400(ver, doc) =
        fbxcel_dom::any::AnyDocument::from_seekable_reader(reader).expect("Failed to load document")
    {
        println!("Loaded FBX DOM successfully: FBX version = {:?}", ver);

        doc.scenes().for_each(|scene| {
            println!("Scene object: object_id={:?}", scene.object_id());

            let root_id = scene
                .root_object_id()
                .expect("Failed to get root object ID");

            println!("\tRoot object ID: {:?}", root_id);
        });
    } else {
        panic!("FBX version unsupported by this example")
    }
}