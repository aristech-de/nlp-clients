use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let protos_dir = "./protos";
    let projects_proto = format!("{}/projects.proto", protos_dir);
    let intents_proto = format!("{}/intents.proto", protos_dir);
    let services_proto = format!("{}/nlp_server.proto", protos_dir);

    tonic_build::configure()
        .build_server(false)
        .compile_protos(
            &[&projects_proto, &intents_proto, &services_proto],
            &[protos_dir],
        )?;

    println!("cargo:rerun-if-changed={}", projects_proto);
    println!("cargo:rerun-if-changed={}", intents_proto);
    println!("cargo:rerun-if-changed={}", services_proto);
    Ok(())
}
