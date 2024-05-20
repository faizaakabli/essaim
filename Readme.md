# Jeu de Robot avec Bevy

Ce projet est un jeu simple développé en Rust en utilisant la bibliothèque Bevy. Le but du jeu est d'avoir un robot qui se déplace à l'écran, collecte des étoiles et des minerais, et interagit avec des arbres (obstacles)

<img width="1273" alt="rustme" src="https://github.com/faizaakabli/essaim/assets/64701810/20bd5859-ae8f-49f4-b03f-22de731effb5">



## Table des matières
- [Installation](#installation)
- [Utilisation](#utilisation)
- [Fonctionnalités](#fonctionnalités)
- [Systèmes](#systèmes)
- [Ressources](#ressources)
- [Composants](#composants)
- [Equipe](#equipe)
- [Important : Précisions suite projet](#précisions)

## Installation

Pour installer et exécuter ce projet, vous aurez besoin de Rust et de Cargo. Si vous ne les avez pas encore installés, suivez les instructions [ici](https://www.rust-lang.org/learn/get-started).

1. Clonez ce dépôt :
    ```sh
    git clone <url-du-depot>
    cd <nom-du-dossier>
    ```

2. Installez les dépendances :
    ```sh
    cargo build
    ```

## Utilisation

Pour exécuter le jeu, utilisez la commande suivante :
```sh
cargo run
```
## Fonctionnalités

- **Mouvement du robot :** Le robot se déplace de manière autonome à l'écran.
- **Collision avec des arbres :** Le robot change de direction lorsqu'il entre en collision avec un arbre.
- **Collecte d'étoiles et de minerais :** Le robot collecte des étoiles et des minerais, ce qui augmente le score.
- **Affichage du score :** Les scores des étoiles et des minerais sont affichés dans la console.
- **Apparition aléatoire des étoiles et des minerais :** Les étoiles et les minerais apparaissent de manière aléatoire à des intervalles définis.

## Systèmes

### Chargement de l'image de fond
Charge et affiche une image de fond au lancement du jeu.

```rust
load_background_image
```

### Apparition du robot
Positionne le robot au centre de l'écran avec une texture et une direction aléatoire.

```rust
spawn_robot
```

### Apparition des arbres
Positionne plusieurs arbres à des endroits spécifiques de l'écran.

```rust
spawn_trees
```

### Apparition des étoiles et des minerais
Génère un nombre défini d'étoiles et de minerais à des positions aléatoires sur l'écran.

```rust
spawn_stars
```

### Mouvement du robot
Fait bouger le robot dans une direction spécifique.

```rust
robot_movement
```

### Mise à jour de la direction du robot
Change la direction du robot lorsqu'il atteint les bords de l'écran et joue un son.

```rust
update_robot_direction
```

### Confinement du mouvement du robot
Empêche le robot de sortir des limites de l'écran.

```rust
confine_robot_movement
```

### Collision avec les arbres
Change la direction du robot lorsqu'il entre en collision avec un arbre et joue un son.

```rust
robot_hit_tree
```

### Collision avec les étoiles et les minerais
Incrémente les scores et supprime les étoiles et les minerais lorsqu'ils sont collectés par le robot.

```rust
robot_hit_star_ore
```

### Affichage des scores
Affiche les scores des étoiles et des minerais dans la console et ajoute un nouveau robot lorsque certaines conditions sont remplies.

```rust
update_score
```

### Timers
Gère les timers pour les apparitions des étoiles et des minerais.

```rust
tick_star_ore_spawn_timer
```

### Apparition des étoiles et des minerais dans le temps
Génère de nouvelles étoiles et minerais à des intervalles définis.

```rust
spawn_stars_ore_over_time
```

## Ressources

### StarScore
Gère le score des étoiles collectées.

```rust
#[derive(Resource)]
pub struct StarScore {
    pub value: u32,
}
```

### OreScore
Gère le score des minerais collectés.

```rust
#[derive(Resource)]
pub struct OreScore {
    pub value: u32,
}
```

### CreateRobot
Gère le nombre d'étoiles et de minerais collectés pour créer de nouveaux robots.

```rust
#[derive(Resource)]
pub struct CreateRobot {
    pub nb_star: u32,
    pub nb_ore: u32,
}
```

### StarSpawnTimer
Timer pour l'apparition des étoiles.

```rust
#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer, 
}
```

### OreSpawnTimer
Timer pour l'apparition des minerais.

```rust
#[derive(Resource)]
pub struct OreSpawnTimer {
    pub timer: Timer, 
}
```

## Composants

### Robot
Composant pour le robot avec une direction.

```rust
#[derive(Component)]
pub struct Robot {
    pub direction: Vec2,
}
```

### Tree
Composant pour les arbres.

```rust
#[derive(Component)]
pub struct Tree {}
```

### Star
Composant pour les étoiles.

```rust
#[derive(Component)]
pub struct Star {}
```

### Ore
Composant pour les minerais.

```rust
#[derive(Component)]
pub struct Ore {}
```

## Equipe

Notre équipe est composée de 5 personnes (noms sur le discord). 

Nous avons travaillé sur ce projet principalement lors de notre presence à l'école, ainsi que durant nos temps libres, souvent ensemble et en appels Discord. Nous avons souvent travaillé en pair programming pour résoudre des problèmes complexes et partager nos connaissances.


## Précisions 


Pour notre projet, nous avons pris le pari  d'utiliser le moteur de jeu Bevy plutôt que d'autres bibliothèques plus simples comme Ggez (choix recommandé par autres collegues). 
On était motivé de partir sur Bevy meme si cela pourrait entraîner des difficultés. 


### Développement et Difficultés

Durant le dév de notre projet, nous avons rencontré plusieurs problemes liés à l'utilisation de Bevy :
1. Gérer les ressources et comprendre l'architecture de Bevy a nécessité du temps significatif.
2. Nous avons rencontré des bugs lors du lancement du jeu, en particulier lors de l'ajout de nouvelles fonctionnalités. Par exemple, lors de la tentative de dev pour un écran noir au début du jeu pour simuler un effet de brouillard, nous avons fait face à des erreurs difficiles à debug.

### Tentatives de Nouvelles Fonctionnalités

Nous avons également tenté d'ajouter des fonctionnalités comme l'effet de brouillard où la carte est initialement cachée par un écran noir qui se dévoile au fur et à mesure que le robot se déplace. Cependant, on a eu plus de bugs persistants et des conflits dans la gestion des composants.

## Conclusion

En conclusion, bien que notre choix d'utiliser Bevy ait rendu le projet plus complexe et ait engendré des défis, nous avons développé une compréhension plus profonde des moteurs de jeu et de la programmation en Rust, même si cela signifie rencontrer et résoudre des problèmes complexes. Nous espérons que notre travail vous a plu.
