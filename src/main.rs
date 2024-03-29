use bevy::gltf::{Gltf, GltfMesh};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, load_gltf)
        .add_systems(Update, setup)
        .run();
}

#[derive(Resource)]
struct MyAssetPack(Handle<Gltf>);

fn load_gltf(mut commands: Commands, ass: Res<AssetServer>) {
    let gltf = ass.load("Fox.glb");
    commands.insert_resource(MyAssetPack(gltf));
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut ev_asset: EventReader<AssetEvent<Gltf>>,
    my: Res<MyAssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    if !ev_asset.read().any(|it| matches!(it, AssetEvent::LoadedWithDependencies { .. })) {
        return;
    }

    let gltf = assets_gltf.get(&my.0).unwrap();
    let fox_gltf_mesh = assets_gltfmesh.get(&gltf.meshes[0]).unwrap();

    commands.spawn(PbrBundle {
        mesh: fox_gltf_mesh.primitives[0].mesh.clone(),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, -10.0, 20.0),
        ..Default::default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 10.0, 15.0).looking_at(Vec3::from((0.0, 0.0, 0.0)), Vec3::Y),
        ..Default::default()
    });
}
