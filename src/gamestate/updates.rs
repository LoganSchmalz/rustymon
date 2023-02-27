use std::collections::HashMap;

use crate::engine_structures::{
    components::{
        Collision, HumanAnimationType, HumanWalkAnimation, MovingEntity, MovingState, Player,
        Position,
    },
    coordinate::{compute_direction, Coordinate, Direction},
    humanoid_properties::{
        ROTATION_TIME, RUN_SPEED, WALK_SPEED,
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
            //update rotation state if entity is idle
            if let (Idle, Moving(rotation)) = (moving.moving, moving.try_moving) {
                if rotation != moving.rotation && moving.rotation_timer >= ROTATION_TIME {
                    moving.rotation = rotation;
                    moving.rotation_timer = 0.0;
                    animation.play_animation(HumanAnimationType::Rotate, rotation);
                }
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
                (Idle, Moving(new_rotation)) | (CenterTile, Moving(new_rotation)) => {
                    //update sprinting state
                    moving.sprinting = moving.try_sprinting;

                    let &mut Position(Coordinate(x, y)) = pos;

                    let Coordinate(target_x, target_y) = match new_rotation {
                        Direction::Left => Coordinate((x - 1.0).ceil(), y),
                        Direction::Right => Coordinate((x + 1.0).floor(), y),
                        Direction::Up => Coordinate(x, (y - 1.0).ceil()),
                        Direction::Down => Coordinate(x, (y + 1.0).floor()),
                    };

                    //we can only play a new animation if the animation isn't playing already, or if we are changing rotations
                    //note essentially these conditions could also be read
                    //entity is idle OR entity is in center of tile (i.e. is trying to move to a wall so can rotate freely)
                    if !animation.is_playing() || moving.rotation != new_rotation {
                        moving.rotation = new_rotation;
                        moving.moving = Moving(new_rotation);
                        let animation_type = match moving.sprinting {
                            true => HumanAnimationType::Run,
                            false => HumanAnimationType::Walk,
                        };
                        animation.play_animation(animation_type, moving.rotation);
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
                Moving(Direction::Left) => Coordinate((x - 1.0).ceil(), y),
                Moving(Direction::Right) => Coordinate((x + 1.0).floor(), y),
                Moving(Direction::Up) => Coordinate(x, (y - 1.0).ceil()),
                Moving(Direction::Down) => Coordinate(x, (y + 1.0).floor()),
                Idle | CenterTile => panic!("Should not happen"),
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
                moving.moving = match moving.try_moving {
                    Idle => Idle,
                    _ => CenterTile,
                }
            }
        }
    }
}
