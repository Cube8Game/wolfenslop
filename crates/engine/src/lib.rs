mod tile_grid;

use bevy::{asset::embedded_asset, core_pipeline::fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin}, prelude::*, render::{RenderApp, extract_component::ExtractComponent, render_resource::ShaderType}};
use raycaster::basic_tiles::FullTile;

use crate::tile_grid::SimpleTileGrid;

const HORIZONTAL_RESOLUTION: usize = 256;

#[derive(Component)]
pub struct Player;

#[derive(Component, ExtractComponent, Clone, Copy, ShaderType)]
#[extract_app(RenderApp)]
pub struct RayCastShader {
    distances: [Vec4; HORIZONTAL_RESOLUTION],
}

impl Default for RayCastShader {
    fn default() -> Self {
        Self {
            distances: [Vec4::ONE; HORIZONTAL_RESOLUTION]
        }
    }
}

impl FullscreenMaterial for RayCastShader {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "embedded://engine/assets/shaders/placeholder.wesl".into()
    }
}
pub struct RayCasterPlugin;

impl Plugin for RayCasterPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "assets/shaders/placeholder.wesl");

        app
            .add_plugins(FullscreenMaterialPlugin::<RayCastShader>::default())
            .add_systems(Startup, (ray_cast_setup, world_setup))
            .add_systems(Update, update_graphics);
    }
}

#[derive(Component, Default)]
pub struct RayCastCamera {
    pub position: Vec2,
    pub direction: f32,
    pub fov: f32,
}

fn ray_cast_setup(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)).looking_at(Vec3::default(), Vec3::Y),
        RayCastShader::default(),
    ));
}

fn world_setup(
    mut commands: Commands
) {
    let mut grid = SimpleTileGrid::bordered((15, 15));
    grid.set_tile::<FullTile>((7, 7));
    commands.spawn(grid);
}

fn update_graphics(
    mut effects: Query<&mut RayCastShader>,
    camera: Single<&RayCastCamera>,
    world: Single<&SimpleTileGrid>
) {
    let distances = world.raycast(camera.position.into(), camera.direction, camera.fov);
    for mut effect in effects.iter_mut() {
        effect.distances = distances.map(|n| {
            Vec4::new(n, 0.0, 0.0, 0.0)
        });
    }
}