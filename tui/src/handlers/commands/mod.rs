pub(super) mod back_to_main_menu;
pub(super) mod command;
pub(super) mod game;
pub(super) mod main_menu;
pub(super) mod promotion_menu;
pub(super) mod quit;
pub mod registry;

use back_to_main_menu::*;
use command::*;
use game::*;
use main_menu::*;
use promotion_menu::*;
use quit::*;
pub use registry::*;
