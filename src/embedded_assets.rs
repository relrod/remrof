use bevy::{asset::embedded_asset, prelude::*};

pub struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../assets/bg/green.png");
        embedded_asset!(app, "../assets/textures/idle.png");
        embedded_asset!(app, "../assets/textures/jump.png");
        embedded_asset!(app, "../assets/textures/run.png");
    }
}
