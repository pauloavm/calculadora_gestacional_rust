// build.rs
fn main() {
    // Este script só precisa ser executado se o arquivo de recurso mudar.
    println!("cargo:rerun-if-changed=assets/app.rc");
    // Usa a crate embed-resource para compilar e embutir o recurso.
    embed_resource::compile("assets/app.rc", embed_resource::NONE);
}
