mod config;
mod generator;

use config::{load_endpoints, Endpoint};
use generator::generate_endpoint;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let endpoints = load_endpoints("endpoints.json")?;
    let output_dir = "generated";

    std::fs::create_dir_all(output_dir)?;
    for endpoint in endpoints {
        println!(
            "Generating API Endpoints for {} {}",
            endpoint.method, endpoint.path
        );
        generate_endpoint(&endpoint, output_dir)?;
    }

    println!(
        "API endpoint generation complete. Files generated in '{}' directory.",
        output_dir
    );

    Ok(())
}
