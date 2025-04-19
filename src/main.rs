use std::f32::consts::PI;

use bevy::{
    math::{ivec3, prelude::Circle},
    pbr::CascadeShadowConfigBuilder,
    prelude::*
    
};

use bevy_screen_diagnostics::ScreenDiagnosticsPlugin;

use new_voxel_testing::{
    rendering::{
        ChunkMaterial, ChunkMaterialWireframe, GlobalChunkMaterial, GlobalChunkWireframeMaterial,
        RenderingPlugin,
    },
    scanner::{Scanner, ScannerPlugin},
    sun::{Sun, SunPlugin},
    utils::world_to_chunk,
    voxel::*,
    voxel_engine::{ChunkModification, VoxelEngine, VoxelEnginePlugin},
};

use bevy_flycam::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VoxelEnginePlugin)
        // .add_plugins(SunPlugin)
        .add_plugins(ScannerPlugin)
        .add_systems(Startup, setup)
        // camera plugin
        .add_plugins(NoCameraPlayerPlugin)
        .add_plugins(RenderingPlugin)
        // .add_plugins(ScreenDiagnosticsPlugin::default())
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 64.0 * 2.0,    // default: 12.0
                                  // speed: 32.0 * 12.0,   // default: 12.0
        })
        .add_systems(Update, modify_current_terrain)
        .run();
}

pub fn setup(
    mut commands: Commands,
    mut chunk_materials: ResMut<Assets<ChunkMaterial>>,
    mut chunk_materials_wireframe: ResMut<Assets<ChunkMaterialWireframe>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        Name::new("directional light light"),
        Sun,
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 10000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_euler(
                EulerRot::ZYX,
                0.0,
                PI / 2.,
                -PI / 4.,
            )),
            cascade_shadow_config: CascadeShadowConfigBuilder {
                num_cascades: 1,
                maximum_distance: 32.0 * 20.0,
                ..default()
            }
            .into(),
            ..default()
        },
    ));

    commands
        .spawn((
            Scanner::new(12),
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 2.0, 0.5),
                ..default()
            },
        ))
        .insert(FlyCam);

    commands.insert_resource(GlobalChunkMaterial(chunk_materials.add(ChunkMaterial {
        reflectance: 0.5,
        perceptual_roughness: 1.0,
        metallic: 0.01,
    })));
    commands.insert_resource(GlobalChunkWireframeMaterial(chunk_materials_wireframe.add(
        ChunkMaterialWireframe {
            reflectance: 0.5,
            perceptual_roughness: 1.0,
            metallic: 0.01,
        },
    )));

    // circular base in origin
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(22.0)),
        material: materials.add(Color::GREEN),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
}

pub fn modify_current_terrain(
    query: Query<&Transform, With<Camera>>,
    key: Res<ButtonInput<KeyCode>>,
    mut voxel_engine: ResMut<VoxelEngine>,
) {
    if !key.pressed(KeyCode::KeyN) {
        return;
    }
    let cam_transform = query.single();
    let cam_chunk = world_to_chunk(cam_transform.translation + (cam_transform.forward() * 64.0));

    let mut rng = rand::thread_rng();
    let mut mods = vec![];
    for _i in 0..32 * 32 {
        let pos = ivec3(
            rng.gen_range(0..32),
            rng.gen_range(0..32),
            rng.gen_range(0..32),
        );
        mods.push(ChunkModification(pos, BlockType::Air));
    }
    voxel_engine.chunk_modifications.insert(cam_chunk, mods);
}