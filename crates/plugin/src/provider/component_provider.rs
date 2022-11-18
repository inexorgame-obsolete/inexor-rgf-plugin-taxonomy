use log::debug;
use log::error;
use paste::paste;
use rust_embed::RustEmbed;

use crate::di::*;
use crate::model::component::Component;
use crate::plugins::component_provider_impl;
use crate::plugins::embedded_asset_provider_impl;
use crate::plugins::ComponentProvider;

component_provider_impl!(Taxonomy, "../../assets/types/components");
