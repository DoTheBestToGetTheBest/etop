// #![warn(missing_docs, unreachable_pub, unused_crate_dependencies)]
// #![deny(unused_must_use, rust_2018_idioms)]
// #![doc(test(
//     no_crate_inject,
//     attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
// ))]

pub mod action;
pub mod app;
pub mod components;
pub mod tui;
pub mod utils;

use crate::{
    app::App,
    utils::{initialize_logging, initialize_panic_handler},
};
use color_eyre::eyre::Result;
use etop_core::EtopState;

pub async fn tokio_main(data: Option<EtopState>, poll_rate: f64) -> Result<()> {
    initialize_logging()?;
    initialize_panic_handler()?;

    let tick_rate = 1.0;
    let frame_rate = 10.0;
    let mut app = App::new(tick_rate, frame_rate, data, poll_rate)?;
    app.run().await?;
    Ok(())
}
