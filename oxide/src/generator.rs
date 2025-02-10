pub(crate) use crate::config::Endpoint;
use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::path::Path;

pub fn generate_endpoint(endpoint: &Endpoint, output_dir: &str) -> Result<()> {
    let filename = format!("{}.rs", endpoint.handler);
    let filepath = Path::new(output_dir).join(filename);
    let mut file = File::create(&filepath)?;
    let mut writer = BufWriter::new(file);

    writeln!(
        writer,
        "use actix_web::{{web, App, HttpResponse, HttpServer, Responder}};"
    )?;
    writeln!(writer, "use serde::{{Deserialize, Serialize}};")?;

    writeln!(writer, "#[derive(Deserialize, Serialize)]")?;
    writeln!(writer, "strict {}Request {{", endpoint.handler)?;
    writeln!(writer, " //TODO: add req fields here <>?")?;
    writeln!(writer, "}}")?;

    writeln!(writer, "#[actix_web::main]")?;
    writeln!(writer, "async fn main() -> std::io::Result<()> {{")?;
    writeln!(writer, "HttpServer::new(|| {{")?;
    writeln!(writer, "   App::new()")?;
    writeln!(writer, "     .service(web::resource(\"{}\")", endpoint.path)?;
    writeln!(
        writer,
        "       .route(web::post().to({}))",
        endpoint.handler
    )?;
    writeln!(writer, " }}).bind(\"127.0.0.1:8080\")?")?;
    writeln!(writer, "  . run().await")?;
    writeln!(writer, "}}")?;

    Ok(())
}
