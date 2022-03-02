use crate::controller::GridManager;
use crate::event_handler::*;
use flowy_derive::{Flowy_Event, ProtoBuf_Enum};
use lib_dispatch::prelude::*;
use std::sync::Arc;
use strum_macros::Display;

pub fn create(grid_manager: Arc<GridManager>) -> Module {
    let mut module = Module::new().name(env!("CARGO_PKG_NAME")).data(grid_manager);

    module = module
        .event(GridEvent::CreateGrid, create_grid_handler)
        .event(GridEvent::OpenGrid, open_grid_handler);

    module
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Display, Hash, ProtoBuf_Enum, Flowy_Event)]
#[event_err = "FlowyError"]
pub enum GridEvent {
    #[event(input = "CreateGridPayload", output = "Grid")]
    CreateGrid = 0,

    #[event(input = "GridId", output = "Grid")]
    OpenGrid = 1,
}
