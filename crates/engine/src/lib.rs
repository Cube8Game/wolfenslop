mod tile_grid;

use bevy::{asset::embedded_asset, core_pipeline::{Core2d, Core2dSystems, fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin}, tonemapping}, ecs::schedule::ScheduleLabel, prelude::*, render::{RenderApp, extract_component::ExtractComponent, render_resource::ShaderType}};
use raycaster::basic_tiles::FullTile;

use crate::tile_grid::SimpleTileGrid;

const HORIZONTAL_RESOLUTION: usize = 1024;

#[derive(Component, ExtractComponent, Clone, Copy, ShaderType)]
#[extract_app(RenderApp)]
pub struct RayCastShader {
    rays: [Vec4; HORIZONTAL_RESOLUTION],
}

impl Default for RayCastShader {
    fn default() -> Self {
        Self {
            rays: [Vec4::new(1.0, 0.5, 0.0, 0.0); HORIZONTAL_RESOLUTION], // TODO: Get texture UV from world
        }
    }
}

impl FullscreenMaterial for RayCastShader {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "embedded://engine/assets/shaders/raycaster.wesl".into()
    }

    fn schedule() -> impl ScheduleLabel + Clone {
        Core2d
    }
    
    fn schedule_configs(system: bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::BoxedSystem>) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::BoxedSystem> {
        system
            .in_set(Core2dSystems::PostProcess)
            .before(tonemapping::tonemapping)
    }
}
pub struct RayCasterPlugin;

impl Plugin for RayCasterPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "assets/shaders/raycaster.wesl");

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
        Camera2d::default(),
        Transform::default(),
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
        effect.rays = distances.map(|hit| {
            Vec4::new(hit.distance, hit.texture, 0.0, 0.0)
        });
    }
}