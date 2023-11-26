use std::f32::consts::PI;

use bevy::prelude::*;
use rand::prelude::*;

const WIDTH: i32 = 512;
const HEIGHT: i32 = 512;

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
        app.add_systems(Startup, setup);
        app.add_systems(Update, update);
        app.add_systems(Update, draw);
    }
}

fn setup(mut commands: Commands) {
    let mut sim: Simulation = Simulation::default();

    let mut rng = rand::thread_rng();

    for i in 0..128 {
        let mut pos: Vec2 = Vec2::new(
            rng.gen::<f32>() * WIDTH as f32,
            rng.gen::<f32>() * HEIGHT as f32,
        );

        pos.x -= WIDTH as f32 / 2.0;
        pos.y -= HEIGHT as f32 / 2.0;

        let rot: f32 = ((rng.gen::<f32>() * 2.0) - 1.0) * PI;
        let vel = Vec2::new(rot.cos(), rot.sin()) * 100.0;

        let particle = Particle {
            position: pos,
            velocity: vel,
        };

        sim.particles.insert(i, particle);
    }

    commands.spawn(sim);
}

fn update(mut query: Query<&mut Simulation>, time: Res<Time>) {
    for mut sim in query.iter_mut() {
        for p in sim.particles.iter_mut() {
            let max_x = WIDTH as f32 / 2.0;
            let min_x = -WIDTH as f32 / 2.0;
            let max_y = HEIGHT as f32 / 2.0;
            let min_y = -HEIGHT as f32 / 2.0;

            if p.position.x > max_x || p.position.x < min_x {
                p.velocity.x = -p.velocity.x;
            }

            if p.position.y > max_y || p.position.y < min_y {
                p.velocity.y = -p.velocity.y;
            }

            p.position.x = p.position.x.clamp(min_x, max_x);
            p.position.y = p.position.y.clamp(min_y, max_y);
            p.position += p.velocity * time.delta_seconds();
        }
    }
}

fn draw(mut query: Query<&mut Simulation>, mut gizmos: Gizmos) {
    gizmos.rect_2d(
        Vec2::ZERO,
        0.0,
        Vec2::new(WIDTH as f32, HEIGHT as f32),
        Color::GREEN,
    );

    for sim in query.iter_mut() {
        for particle in &sim.particles {
            gizmos.circle_2d(particle.position, 1.0, Color::WHITE);
        }
    }
}
