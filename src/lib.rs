pub mod game_systems {
    use bevy::prelude::*;
    use bevy::ecs::system::ResMut;

    pub const ROBOT_SPEED: f32 = 100.0;
    pub const ROBOT_SIZE: f32 = 34.0;
    pub const TREE_SIZE: f32 = 64.0;
    pub const STAR_SIZE: f32 = 20.0;
    pub const ORE_SIZE: f32 = 20.0;

    #[derive(Component)]
    pub struct Robot {
        pub direction: Vec2,
    }

    #[derive(Component)]
    pub struct Star {}

    #[derive(Component)]
    pub struct Ore {}

    #[derive(Resource)]
    pub struct StarScore {
        pub value: u32,
    }

    impl Default for StarScore {
        fn default() -> StarScore {
            StarScore { value: 0 }
        }
    }

    pub fn setup(mut commands: Commands) {
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

    pub fn robot_movement(mut query: Query<(&mut Transform, &Robot)>, time: Res<Time>) {
        for (mut transform, robot) in query.iter_mut() {
            let direction = Vec3::new(robot.direction.x, robot.direction.y, 0.0);
            transform.translation += direction * ROBOT_SPEED * time.delta_seconds();
        }
    }

    pub fn robot_hit_star_ore(
        mut commands: Commands,
        robot_query: Query<(Entity, &Transform), With<Robot>>,
        star_query: Query<(Entity, &Transform), With<Star>>,
        mut star_score: ResMut<StarScore>,
    ) {
        for (robot_entity, robot_transform) in robot_query.iter() {
            for (star_entity, star_transform) in star_query.iter() {
                let distance = robot_transform.translation.distance(star_transform.translation);

                if distance < ROBOT_SIZE / 2.0 + STAR_SIZE / 2.0 {
                    star_score.value += 1;
                    commands.entity(star_entity).despawn();
                }
            }
        }
    }
}
