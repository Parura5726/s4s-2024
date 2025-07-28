use std::process::Command;

// TODO: Link to ghcr
const UNIVERSAL_IMAGE: &str = "localhost/clic-s4s-2025:latest";

//pub const JAVA_IMAGE: &str = "cimg/openjdk:21.0";
//pub const PYTHON_IMAGE: &str = "python:3-bullseye";
//pub const CPP_IMAGE: &str = "ghcr.io/clicepfl/s4s-2024-cpp:main";

pub const JAVA_IMAGE: &str = UNIVERSAL_IMAGE;
pub const PYTHON_IMAGE: &str = UNIVERSAL_IMAGE;
pub const CPP_IMAGE: &str = UNIVERSAL_IMAGE;

pub const IMAGES: [&str; 3] = [JAVA_IMAGE, PYTHON_IMAGE, CPP_IMAGE];

pub fn pull_required_images() {
    for image in IMAGES {
        if let Err(err) = Command::new("docker").args(["pull", image]).status() {
            println!("Error while pulling {image}: {err:#?}");
        }
    }
}
