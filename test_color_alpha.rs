use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());
    
    // Test different color formats and alpha values
    let test_colors = [
        ("srgb(0.56, 0.93, 0.56)", Color::srgb(0.56, 0.93, 0.56)),
        ("srgba with alpha=1", Color::srgba(0.56, 0.93, 0.56, 1.0)),
        ("srgba with alpha=0.5", Color::srgba(0.56, 0.93, 0.56, 0.5)),
        ("linear_rgb", Color::linear_rgb(0.56, 0.93, 0.56)),
        ("from tuple", Color::from((0.56, 0.93, 0.56))),
    ];
    
    for (i, (name, color)) in test_colors.iter().enumerate() {
        let x = (i as f32 - 2.0) * 120.0;
        
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(*color))),
            Transform::from_xyz(x, 0.0, 0.0),
        ));
        
        println!("{}: alpha = {}", name, color.alpha());
    }
    
    println!("\nIf some squares are missing, alpha might be the issue");
}