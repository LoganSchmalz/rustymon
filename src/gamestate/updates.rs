use std::collections::HashMap;

use crate::engine_structures::{
    components::{Collision, HumanWalkAnimation, MovingEntity, MovingState, Player, Position, HumanAnimationType},
    coordinate::{compute_direction, Coordinate, Direction},
    humanoid_properties::{
        ROTATION_TIME, RUNNING_TIME_PER_TILE, RUN_SPEED, WALKING_TIME_PER_TILE, WALK_SPEED,
    },
};

use super::State;

impl State {
    pub fn update_animations(&mut self, delta_time: f32) {
        let mut animation_query = self.world.query::<&mut HumanWalkAnimation>();

        for (_, anim) in animation_query.iter() {
            anim.update(delta_time);
        }
    }

    pub fn update_collisions(&mut self) {
        let mut collision_query = self.world.query::<(&mut Position, &Collision)>();

        self.collisions = HashMap::new();
        //self.collisions.reserve(collision_query.iter().len());

        for (entity, (Position(c), _)) in collision_query.iter() {
            self.collisions.insert(c.to_usize(self.map.size_x), entity);
        }
    }

    pub fn update_player_moving_direction(
        &mut self,
        moving_state: MovingState,
    ) -> Result<(), String> {
        let (_, mut moving) = self
            .world
            .query_one_mut::<(&Player, &mut MovingEntity)>(self.player)
            .or(Err("No player found"))?;

        moving.try_moving = moving_state;

        Ok(())
    }

    pub fn update_player_sprinting(&mut self, sprinting: bool) -> Result<(), String> {
        let (_, mut moving) = self
            .world
            .query_one_mut::<(&Player, &mut MovingEntity)>(self.player)
            .or(Err("No player found"))?;

        moving.try_sprinting = sprinting;

        Ok(())
    }

    pub fn update_moving_objects(&self, delta_time: f32) {
        use MovingState::*;

        let mut moving_query =
            self.world
                .query::<(&mut Position, &mut MovingEntity, &mut HumanWalkAnimation)>();
        for (_, (pos, moving, animation)) in moving_query.iter() {
            //update sprinting state
            match moving.moving {
                Idle | CenterTile => moving.sprinting = moving.try_sprinting,
                _ => (),
            }

            //update rotation state
            match (moving.moving, moving.try_moving) {
                (_, Idle) => (),
                (Idle, Moving(rotation)) => {
                    if rotation != moving.rotation && moving.rotation_timer >= ROTATION_TIME {
                        moving.rotation = rotation;
                        moving.rotation_timer = 0.0;
                        animation.play_animation(HumanAnimationType::Rotate, rotation)
                    }
                }
                (CenterTile, Moving(rotation)) => {
                    if !animation.is_playing() {
                        moving.rotation = rotation;
                    }
                }
                (_, _) => (),
            }

            //increment rotation time and don't allow movement if rotation is not finished
            if moving.rotation_timer < ROTATION_TIME {
                moving.rotation_timer += delta_time;
                continue;
            }

            //if rotation is finished, we can update the moving state of the entity
            match (moving.moving, moving.try_moving) {
                //if the entity is idle and wants to be idle, we don't do any updates
                (Idle, Idle) | (CenterTile, Idle) => {
                    continue;
                }
                //if the entity is idle and wants to be moving, we update the moving state and play the new animation
                (Idle, new_state) | (CenterTile, new_state) => {
                    let &mut Position(Coordinate(x, y)) = pos;

                    let Coordinate(target_x, target_y) = match new_state {
                        MovingState::Moving(Direction::Left) => Coordinate((x - 1.0).ceil(), y),
                        MovingState::Moving(Direction::Right) => Coordinate((x + 1.0).floor(), y),
                        MovingState::Moving(Direction::Up) => Coordinate(x, (y - 1.0).ceil()),
                        MovingState::Moving(Direction::Down) => Coordinate(x, (y + 1.0).floor()),
                        MovingState::Idle | MovingState::CenterTile => {
                            panic!("Should not happen")
                        }
                    };

                    if !animation.is_playing() {
                        let animation_type = match moving.sprinting {
                            true => HumanAnimationType::Run,
                            false => HumanAnimationType::Walk,
                        };
                        animation.play_animation(animation_type, moving.rotation);
                        moving.moving = new_state;
                    }

                    if self.check_collision(&(target_x, target_y).into())
                        || moving.moving == MovingState::CenterTile
                    {
                        moving.moving = MovingState::CenterTile;
                        continue;
                    }
                }
                _ => (),
            }

            let &mut Position(Coordinate(x, y)) = pos;

            let Coordinate(target_x, target_y) = match moving.moving {
                MovingState::Moving(Direction::Left) => Coordinate((x - 1.0).ceil(), y),
                MovingState::Moving(Direction::Right) => Coordinate((x + 1.0).floor(), y),
                MovingState::Moving(Direction::Up) => Coordinate(x, (y - 1.0).ceil()),
                MovingState::Moving(Direction::Down) => Coordinate(x, (y + 1.0).floor()),
                MovingState::Idle | MovingState::CenterTile => panic!("Should not happen"),
            };

            let speed = delta_time
                * match moving.sprinting {
                    true => RUN_SPEED,
                    false => WALK_SPEED,
                };

            let (dx, dy) = match compute_direction(Coordinate(x, y), Coordinate(target_x, target_y))
            {
                Direction::Up => (0.0, -speed),
                Direction::Down => (0.0, speed),
                Direction::Left => (-speed, 0.0),
                Direction::Right => (speed, 0.0),
            };

            //set new position
            *pos = Position(Coordinate(x + dx, y + dy));

            let &mut Position(Coordinate(x, y)) = pos;

            //check if we have passed the tile we were trying to get to
            if (x, y) == (target_x, target_y)
                || dx != 0.0 && (target_x - x).signum() != dx.signum()
                || dy != 0.0 && (target_y - y).signum() != dy.signum()
            {
                *pos = Position(Coordinate(target_x, target_y));
                moving.moving = MovingState::CenterTile;
            }
        }
    }
}
