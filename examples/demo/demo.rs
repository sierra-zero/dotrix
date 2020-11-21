use dotrix::{
    Dotrix,
    assets::{ Mesh, Texture },
    components::{ Light, StaticModel },
    ecs::{ Mut, RunLevel, System },
    services::{ Assets, Camera, World },
    systems::{ static_renderer, camera_control },
};

fn main() {

    Dotrix::application("Input Example")
        .with_system(System::from(static_renderer).with(RunLevel::Render))
        .with_system(System::from(startup).with(RunLevel::Startup))
        .with_system(System::from(camera_control))
        .with_service(Assets::new())
        .with_service(Camera::new(10.0, 3.14 / 2.0, 4.0))
        .with_service(World::new())
        .run();

}

fn startup(mut world: Mut<World>, mut assets: Mut<Assets>) {
    assets.import("assets/crate.png", "crate");

    let texture = assets.find::<Texture>("crate");
    let cube1 = assets.register::<Mesh>(Mesh::cube(), String::from("cube1"));
    let cube2 = assets.register::<Mesh>(Mesh::cube2(), String::from("cube2"));

    world.spawn(vec![
        (StaticModel::new(cube2, texture),),
        (StaticModel::new(cube1, texture),),
    ]);

    world.spawn(Some((Light::white([10.0, 2.0, 4.0]),)));
}