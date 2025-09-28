use loco_rs::prelude::*;
use via_generated::controllers::{article, comment};

#[tokio::main]
async fn main() -> Result<()> {
    // Materialize generated routes so compilation ensures we link everything.
    let _ = (article::routes(), comment::routes());

    println!("Blog demo ready. Run loco_rs stack to integrate generated routes.");
    Ok(())
}
