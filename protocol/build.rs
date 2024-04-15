use std::{io::Result, path::Path};

fn main() -> Result<()> {
    let protos = glob::glob("protos/**/*.proto")
        .unwrap()
        .into_iter()
        .map(|r| {
            let path = r.unwrap();

            #[cfg(windows)]
            let path = PathBuf::from(path.to_slash_lossy().to_string());

            println!(
                "cargo:rerun-if-changed={}",
                path.as_os_str().to_string_lossy()
            );
            path
        })
        .collect::<Vec<_>>();

    let includes = vec![Path::new("protos")];

    let mut config = prost_build::Config::new();
    config.bytes(&["."]);
    config.type_attribute(".", "#[derive(PartialOrd)]");

    let _ = std::fs::remove_dir_all("src/pb");
    std::fs::create_dir_all("src/pb").unwrap();

    config
        .out_dir("src/pb")
        .compile_protos(&protos, &includes)?;

    Ok(())
}
