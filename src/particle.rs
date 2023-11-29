use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use std::f32::consts::TAU;

use rand::prelude::*;

use crate::grid::Grid;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

#[derive(Clone, Copy, Default, Debug)]
struct Particle {
    position: Vec2,
    velocity: Vec2,
}

#[derive(Component, Default)]
struct Simulation {
    particles: Vec<Particle>,
}

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, input)
            .add_systems(FixedUpdate, update)
            .insert_resource(Time::<Fixed>::from_seconds(1.0 / 15.0))
            .add_systems(Update, draw);
    }
}

fn setup(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let mut particles: Vec<Particle> = Vec::new();
    particles.resize(
        10,
        Particle {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
        },
    );

    for particle in particles.iter_mut() {
        let mut pos: Vec2 = Vec2::new(
            rng.gen::<f32>() * WIDTH as f32,
            rng.gen::<f32>() * HEIGHT as f32,
        );

        pos.x -= WIDTH as f32 / 2.0;
        pos.y -= HEIGHT as f32 / 2.0;

        let rot: f32 = rng.gen::<f32>() * TAU;
        let vel = Vec2::new(rot.cos(), rot.sin()) * 100.0;

        particle.position = pos;
        particle.velocity = vel;
    }

    commands.spawn(Simulation { particles });
}

fn input(
    mut query: Query<&mut Simulation>,
    buttons: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    for mut sim in query.iter_mut() {
        if let Some(mut cursor_position) = window.single().cursor_position() {
            for button in buttons.get_just_pressed() {
                if button.clone() == MouseButton::Left {
                    let window = window.single();

                    let window_size: Vec2 = Vec2::new(window.width(), window.height());

                    cursor_position = cursor_position - window_size / 2.0;
                    cursor_position.y = -cursor_position.y;

                    let particle = Particle {
                        position: cursor_position,
                        velocity: Vec2::ZERO,
                    };

                    sim.particles.push(particle);
                }
            }
        } else {
            return;
        }
    }
}

fn update(mut query: Query<&mut Simulation>, time: Res<Time>) {
    for mut sim in query.iter_mut() {
        let h_w = WIDTH as f32 / 2.0;
        let h_h = HEIGHT as f32 / 2.0;

        let mut grid = Grid::new(
            Rect {
                min: Vec2::new(-(WIDTH as f32 - h_w), -(HEIGHT as f32 - h_h)),
                max: Vec2::new(h_w, h_h),
            },
            1,
        );

        for i in 0..sim.particles.len() {
            let h_w = WIDTH as f32 / 2.0;
            let h_h = HEIGHT as f32 / 2.0;

            let mut position: Vec2 = sim.particles[i].position;
            let mut velocity: Vec2 = sim.particles[i].velocity;

            if position.x > h_w || position.x < -h_w {
                velocity.x = -velocity.x;
            }

            if position.y > h_h || position.y < -h_h {
                velocity.y = -velocity.y;
            }

            position.x = position.x.clamp(-h_w, h_w);
            position.y = position.y.clamp(-h_h, h_h);
            position += velocity * time.delta_seconds();

            let cell_idx = grid.get_cell(position);
            grid.insert(cell_idx, i);

            sim.particles[i].position = position;
            sim.particles[i].velocity = velocity;

            let cell_idx = grid.get_cell(position);
            grid.insert(cell_idx, i);
        }

        println!("{:?}", grid.get_cell_elements(0));
    }
}

fn draw(mut query: Query<&mut Simulation>, mut gizmos: Gizmos) {
    for sim in query.iter_mut() {
        for particle in &sim.particles {
            gizmos.circle_2d(particle.position, 1.0, Color::WHITE);
        }

        let splits: usize = 1;
        let cell_size = Vec2::new((WIDTH / splits) as f32, (HEIGHT / splits) as f32);

        for y in 0..splits {
            for x in 0..splits {
                let mut position = Vec2::new(x as f32, y as f32) * cell_size;
                position += cell_size / 2.0;
                position -= Vec2::new((WIDTH / 2) as f32, (HEIGHT / 2) as f32);

                gizmos.rect_2d(position, 0.0, cell_size, Color::WHITE);
            }
        }
    }

    gizmos.rect_2d(
        Vec2::ZERO,
        0.0,
        Vec2::new(WIDTH as f32, HEIGHT as f32),
        Color::GREEN,
    );
}
