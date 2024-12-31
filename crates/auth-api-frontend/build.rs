use npm_rs::*;

fn main() {
    println!("cargo:rerun-if-changed=../../frontend/package.json");
    println!("cargo:rerun-if-changed=../../frontend/svelte.config.js");
    println!("cargo:rerun-if-changed=../../frontend/tsconfig.json");
    println!("cargo:rerun-if-changed=../../frontend/vite.config.ts");
    println!("cargo:rerun-if-changed=../../frontend/src");
    println!("cargo:rerun-if-changed=../../frontend/static");

    NpmEnv::default()
        .with_node_env(&NodeEnv::from_cargo_profile().unwrap_or_default())
        .set_path("../../frontend")
        .init_env()
        .install(None)
        .run("build")
        .exec()
        .unwrap();
}
