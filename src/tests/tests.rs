use bevy::prelude::*;
use bevy::app::App;
use bevy::ecs::system::ResMut;
use bevy::ecs::schedule::Schedule;
use bevy::ecs::system::SystemStage;
use robotsessaim_projectrust::game_systems::*;

#[test]
fn test_robot_movement() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .insert_resource(Time::default())
        .add_startup_system(setup.system())
        .add_system(robot_movement.system());

    app.update();

    let query = app.world.query::<(&Transform, &Robot)>();
    for (transform, _robot) in query.iter(&app.world) {
        assert_eq!(transform.translation.x, ROBOT_SPEED * 0.016); // Assuming a frame time of 16ms
        assert_eq!(transform.translation.y, 0.0);
    }
}

#[test]
fn test_robot_hit_star() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .insert_resource(StarScore::default())
        .add_startup_system(setup.system())
        .add_system(robot_hit_star_ore.system());

    app.update();

    let star_score = app.world.get_resource::<StarScore>().unwrap();
    assert_eq!(star_score.value, 1);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Transform::default(),
        Robot {
            direction: Vec2::new(1.0, 0.0),
        }
    ));

    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Star {}
    ));
}
