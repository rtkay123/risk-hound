pub mod api;

use anyhow::Result;

pub async fn serve() -> Result<()> {
    api::router(3000).await
}
