use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::math::Vec3;
use bevy::math::Vec2;
use rand::prelude::*;

pub const ROBOT_SPEED: f32 = 100.0;
pub const ROBOT_SIZE: f32 = 34.0;
pub const TREE_SIZE: f32 = 64.0;
pub const NUMBER_OF_STARS: usize = 3;
pub const STAR_SIZE: f32 = 20.0;
pub const ORE_SIZE: f32 = 20.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;
pub const ORE_SPAWN_TIME: f32 = 1.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<StarScore>()
        .init_resource::<OreScore>()
        .init_resource::<CreateRobot>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<OreSpawnTimer>()
        .add_startup_system(load_background_image)
        .add_startup_system(spawn_robot)
        .add_startup_system(spawn_trees)
        .add_startup_system(spawn_stars)
        .add_system(robot_movement)
        .add_system(update_robot_direction)
        .add_system(confine_robot_movement)
        .add_system(robot_hit_tree)
        .add_system(robot_hit_star_ore)
        .add_system(update_score)
        .add_system(tick_star_ore_spawn_timer)
        .add_system(spawn_stars_ore_over_time)
        .run();
}


// déclarations de composants et ressources
#[derive(Component)]
pub struct Robot {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Tree {}

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

#[derive(Resource)]
pub struct OreScore {
    pub value: u32,
}

impl Default for OreScore {
    fn default() -> OreScore {
        OreScore { value: 0 }
    }
}

#[derive(Resource)]
pub struct CreateRobot {
    pub nb_star: u32,
    pub nb_ore: u32,
}

impl Default for CreateRobot {
    fn default() -> Self {
        Self {
            nb_star: 0 , nb_ore: 0
        }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer, 
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct OreSpawnTimer {
    pub timer: Timer, 
}

impl Default for OreSpawnTimer {
    fn default() -> OreSpawnTimer {
        OreSpawnTimer {
            timer: Timer::from_seconds(ORE_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

// ajout d'une image de fond 
pub fn load_background_image(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let window = window_query.get_single().unwrap();
    let texture_handle: Handle<Image> = asset_server.load("sprites/background-image.png");

    commands.spawn(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(window.width() / 2.0, window.height() / 2.5, -10.0)),
        texture: texture_handle.into(),
        ..Default::default()
    });

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

// affichage du robot vert (principal)
pub fn spawn_robot(
    mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>, 
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0).with_scale(Vec3::splat(0.3)),
            texture: asset_server.load("sprites/robot_greenJump.png"),
            ..default()
        },
        Robot {
            direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
        }, 
    ));
}

// affichage des arbres
pub fn spawn_trees(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>,) {
    let window = window_query.get_single().unwrap();


    // affiche maison
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() - 750.0, window.height() - 250.0, 0.0).with_scale(Vec3::splat(0.5)),
            texture: asset_server.load("sprites/house.png"),
            ..default()
        },
        Tree {},
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() - 100.0, window.height() - 300.0, 0.0).with_scale(Vec3::splat(0.5)),
            texture: asset_server.load("sprites/tree.png"),
            ..default()
        },
        Tree {},
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() - 1000.0, window.height() - 650.0, 0.0).with_scale(Vec3::splat(0.5)),
            texture: asset_server.load("sprites/tree3.png"),
            ..default()
        },
        Tree {},
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() - 400.0, window.height() - 600.0, 0.0).with_scale(Vec3::splat(0.5)),
            texture: asset_server.load("sprites/tree2.png"),
            ..default()
        },
        Tree {},
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() - 750.0, window.height() - 450.0, 0.0).with_scale(Vec3::splat(0.5)),
            texture: asset_server.load("sprites/tree3.png"),
            ..default()
        },
        Tree {},
    ));

}

// affichage des éclaires (étoiles) et minerais de façon aléatoire
pub fn spawn_stars(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz( random::<f32>() * window.width(), random::<f32>() * window.height(), 0.0).with_scale(Vec3::splat(0.5)),
                texture: asset_server.load("sprites/eclaire.png"),
                ..default()
            },
            Star {},
        ));

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz( random::<f32>() * window.width(), random::<f32>() * window.height(), 0.0).with_scale(Vec3::splat(0.2)),
                texture: asset_server.load("sprites/ore_pink.png"),
                ..default()
            },
            Ore {},
        ));
    }
}

// robot qui bouge
pub fn robot_movement(mut robot_query: Query<(&mut Transform, &Robot)>, time: Res<Time>) {
    for (mut transform, robot) in robot_query.iter_mut() {
        let direction = Vec3::new(robot.direction.x, robot.direction.y, 0.0);
        transform.translation += direction * ROBOT_SPEED * time.delta_seconds();
    }
}

pub fn update_robot_direction(mut robot_query: Query<(&Transform, &mut Robot)>, window_query: Query<&Window, With<PrimaryWindow>>, audio: Res<Audio>, asset_server: Res<AssetServer>,) {
    let window = window_query.get_single().unwrap();
    let half_robot_size = ROBOT_SIZE / 2.0;
    let x_min = 0.0 + half_robot_size;
    let x_max = window.width() - half_robot_size;
    let y_min = 0.0 + half_robot_size;
    let y_max = window.height() - half_robot_size;
    
    for (transform, mut robot) in robot_query.iter_mut() {
        let mut direction_changed = false;
        let translation = transform.translation;

        if translation.x < x_min || translation.x > x_max {
            robot.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            robot.direction.y *= -1.0;
            direction_changed = true;
        }
        if direction_changed {
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };
            audio.play(sound_effect);
        }
    }
}

pub fn confine_robot_movement(mut robot_query: Query<&mut Transform, With<Robot>>, window_query: Query<&Window, With<PrimaryWindow>>,) {
    let window = window_query.get_single().unwrap();
    let half_robot_size = ROBOT_SIZE / 2.0;
    let x_min = 0.0 + half_robot_size;
    let x_max = window.width() - half_robot_size;
    let y_min = 0.0 + half_robot_size;
    let y_max = window.height() - half_robot_size; 

    for mut transform in robot_query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }
        
        transform.translation = translation;
    }
}

// robot qui bouge sans passer à travers des arbres
pub fn robot_hit_tree(
    mut commands: Commands,
    robot_query: Query<(Entity, &Transform), With<Robot>>,
    mut robot_query2: Query<(&Transform, &mut Robot)>,
    tree_query: Query<&Transform, With<Tree>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for (robot_entity, robot_transform) in robot_query.iter() {
        for tree_transform in tree_query.iter() {
            let distance = robot_transform.translation.distance(tree_transform.translation);
            let robot_radius = ROBOT_SIZE / 2.0;
            let tree_radius = TREE_SIZE / 2.0;

            if distance < robot_radius + tree_radius {
                // println!("** BONK **");
                let sound_effect = asset_server.load("audio/pluck_002.ogg");
                audio.play(sound_effect);

                let random_angle: f32 = random::<f32>() * std::f32::consts::FRAC_PI_4 * 0.5;
                let sign = if random::<bool>() { 1.0 } else { -1.0 }; 
                let rotated_direction = Vec2::new(sign * random_angle.cos(), random_angle.sin());
                for (transform, mut robot) in robot_query2.iter_mut() {
                    if transform.translation == robot_transform.translation {
                        robot.direction = rotated_direction;
                    }
                }
            }
        }
    }
}


// robot qui collecte des éclaires (stars) et minerais
pub fn robot_hit_star_ore(
    mut commands: Commands,
    robot_query: Query<(Entity, &Transform), With<Robot>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    ore_query: Query<(Entity, &Transform), With<Ore>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut star_score: ResMut<StarScore>,
    mut ore_score: ResMut<OreScore>,
) {
    // Itérer sur tous les robots
    for (robot_entity, robot_transform) in robot_query.iter() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = robot_transform.translation.distance(star_transform.translation);

            if distance < ROBOT_SIZE / 2.0 + STAR_SIZE / 2.0 {
                // println!("***** Robot hit star !!! *****");
                star_score.value += 1;
                let sound_effect = asset_server.load("audio/explosionCrunch_002.ogg");
                audio.play(sound_effect);
                commands.entity(star_entity).despawn();
            }
        }
        
        for (ore_entity, ore_transform) in ore_query.iter() {
            let distance = robot_transform.translation.distance(ore_transform.translation);

            if distance < ROBOT_SIZE / 2.0 + ORE_SIZE / 2.0 {
                // println!("***** Robot hit ore !!! *****");
                ore_score.value += 1;
                let sound_effect = asset_server.load("audio/explosionCrunch_002.ogg");
                audio.play(sound_effect);
                commands.entity(ore_entity).despawn();
            }
        }
    }
}


// affichage des scores des éclaires (stars) et minerais sur la console
pub fn update_score(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>, star_score: Res<StarScore>, ore_score: Res<OreScore>, mut create_robot: Local<CreateRobot>) {
    if star_score.is_changed() {
        println!("Eclaire Score: {}", star_score.value.to_string());
        create_robot.nb_star = star_score.value;
    }

    if ore_score.is_changed() {
        println!("Ore Score: {}", ore_score.value.to_string());
        create_robot.nb_ore = ore_score.value;
    }

    if (create_robot.nb_star >= 3 && create_robot.nb_ore >= 1) {
        let window = window_query.get_single().unwrap();

        let robot_textures = vec![
            "sprites/robot_yellowJump.png",
            "sprites/robot_redJump.png",
            "sprites/robot_blueJump.png",
            "sprites/robot_greenJump.png",
        ];
    
        let random_robot_index = thread_rng().gen_range(0..robot_textures.len());
        let chosen_robot_texture = robot_textures[random_robot_index];

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random::<f32>() * window.width(), random::<f32>() * window.height(), 0.0).with_scale(Vec3::splat(0.3)),
                texture: asset_server.load(chosen_robot_texture),
                ..default()
            },
            Robot {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            }, 
        ));

        create_robot.nb_star = 0;
        create_robot.nb_ore = 0;
    }
}

// incrémenter un robot après 
pub fn tick_star_ore_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, mut ore_spawn_timer: ResMut<OreSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
    ore_spawn_timer.timer.tick(time.delta());
}

// incrémebtation des éclaires (stars) et minerais une fois collecté par le robot
pub fn spawn_stars_ore_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: ResMut<StarSpawnTimer>,
    ore_spawn_timer: ResMut<OreSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random::<f32>() * window.width(), random::<f32>() * window.height(), 0.0).with_scale(Vec3::splat(0.5)),
                texture: asset_server.load("sprites/eclaire.png"),
                ..default()
            },
            Star {},
        ));
    }

    if ore_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random::<f32>() * window.width(), random::<f32>() * window.height(), 0.0).with_scale(Vec3::splat(0.2)),
                texture: asset_server.load("sprites/ore_pink.png"),
                ..default()
            },
            Ore {},
        ));
    }
}


