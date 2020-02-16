use std::{
    convert::TryFrom,
    ops::{BitAnd, Mul, Sub},
};

use crow_ecs::{Entity, SparseStorage, Storage};

use crow_anim::{AnimationHandle, AnimationState, Sprite};

#[derive(Default)]
pub struct Components {
    pub count: usize,
    pub deleted: Vec<Entity>,
    pub positions: Storage<Position>,
    pub sprites: Storage<Sprite>,
    pub animations: Storage<AnimationState>,
    pub previous_positions: Storage<Position>,
    pub velocities: Storage<Velocity>,
    pub colliders: Storage<Collider>,
    pub grounded: Storage<Grounded>,
    pub wall_collisions: Storage<WallCollision>,
    pub gravity: Storage<Gravity>,
    pub ignore_bridges: SparseStorage<IgnoreBridges>,
    pub player_state: SparseStorage<PlayerState>,
    pub player_animations: SparseStorage<PlayerAnimations>,
    pub depths: Storage<Depth>,
    pub mirrored: SparseStorage<Mirrored>,
}

impl Components {
    pub fn new() -> Self {
        Components::default()
    }

    pub fn new_entity(&mut self) -> Entity {
        if let Some(e) = self.deleted.pop() {
            e
        } else {
            let e = Entity(self.count);
            self.count += 1;
            e
        }
    }

    pub fn delete_entity(&mut self, e: Entity) {
        self.deleted.push(e);
        self.positions.remove(e);
        self.sprites.remove(e);
        self.animations.remove(e);
        self.previous_positions.remove(e);
        self.velocities.remove(e);
        self.colliders.remove(e);
        self.grounded.remove(e);
        self.wall_collisions.remove(e);
        self.gravity.remove(e);
        self.ignore_bridges.remove(e);
        self.player_state.remove(e);
        self.player_animations.remove(e);
        self.depths.remove(e);
        self.mirrored.remove(e);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Mul<f32> for Velocity {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Velocity {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Sub for Velocity {
    type Output = Velocity;

    fn sub(self, other: Self) -> Velocity {
        Velocity {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Depth {
    Background,
    Bridges,
    Grass,
    Player,
    Tiles,
    EditorSelection,
    Particles,
}

impl From<Depth> for f32 {
    fn from(depth: Depth) -> f32 {
        match depth {
            Depth::Background => 0.3,
            Depth::Grass => 0.5,
            Depth::Tiles => 0.5,
            Depth::Player => 0.6,
            Depth::Bridges => 0.7,
            Depth::EditorSelection => 0.8,
            Depth::Particles => 0.9,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    Grounded,
    Airborne,
    Dying,
    Dead,
}

pub struct PlayerAnimations {
    pub idle: AnimationHandle,
    pub running: AnimationHandle,
    /// run once after jumping -> falling,
    pub jumping: AnimationHandle,
    /// run once during a jump/fall -> falling,
    pub start_falling: AnimationHandle,
    pub falling: AnimationHandle,
}

#[derive(Debug, Clone, Copy)]
pub struct Collider {
    pub w: f32,
    pub h: f32,
    pub ty: ColliderType,
}

impl Collider {
    #[inline]
    pub fn left_border(&self, pos: Position) -> f32 {
        pos.x
    }

    #[inline]
    pub fn right_border(&self, pos: Position) -> f32 {
        pos.x + self.w
    }

    #[inline]
    pub fn lower_border(&self, pos: Position) -> f32 {
        pos.y
    }

    #[inline]
    pub fn upper_border(&self, pos: Position) -> f32 {
        pos.y + self.h
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ColliderType {
    Environment,
    Player,
    Bridge,
    PlayerDamage,
}

#[derive(Debug, Clone, Copy)]
pub struct Collision(pub Entity, pub Entity);

#[derive(Debug, Default)]
pub struct Collisions {
    pub fixed: Vec<Collision>,
    pub bridge: Vec<Collision>,
    pub player_damage: Vec<Collision>,
}

impl Collisions {
    pub fn clear(&mut self) {
        self.fixed.clear();
        self.bridge.clear();
        self.player_damage.clear();
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CollisionDirection {
    None = 0b0000,
    LeftAbove = 0b1001,
    Above = 0b0001,
    RightAbove = 0b0011,
    Right = 0b0010,
    RightBelow = 0b0110,
    Below = 0b0100,
    LeftBelow = 0b1100,
    Left = 0b1000,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct InvalidBitPattern;

impl TryFrom<u8> for CollisionDirection {
    type Error = InvalidBitPattern;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0000 => Ok(CollisionDirection::None),
            0b1001 => Ok(CollisionDirection::LeftAbove),
            0b0001 => Ok(CollisionDirection::Above),
            0b0011 => Ok(CollisionDirection::RightAbove),
            0b0010 => Ok(CollisionDirection::Right),
            0b0110 => Ok(CollisionDirection::RightBelow),
            0b0100 => Ok(CollisionDirection::Below),
            0b1100 => Ok(CollisionDirection::LeftBelow),
            0b1000 => Ok(CollisionDirection::Left),
            _ => Err(InvalidBitPattern),
        }
    }
}

impl BitAnd for CollisionDirection {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::try_from(self as u8 & rhs as u8).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grounded;

#[derive(Debug, Clone, Copy)]
pub struct Gravity;

#[derive(Debug, Clone, Copy)]
pub struct IgnoreBridges;

/// Used during `draw::scene` to horizontally flip sprites based on the collider of the given entity.
#[derive(Debug, Clone, Copy)]
pub struct Mirrored;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WallCollision {
    Right,
    Left,
}
