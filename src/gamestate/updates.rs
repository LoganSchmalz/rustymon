/****************************************************/
// Created by: Logan Schmalz
// Description: Logic for updating the game state at every tick of the game
// based on player input and other game state considerations
/****************************************************/
use std::collections::HashMap;

use crate::{
    components::{
        animation::{HumanAnimationType, HumanWalkAnimation},
        Collision, MovingEntity, MovingState, Position, Npc,
    },
    constants::{ROTATION_TIME, RUN_SPEED, WALK_SPEED},
    gamestate::{event::Event, Screen},
    vec2::{compute_direction, Direction, Vec2},
};

use super::State;

impl State {
    //this just loops through animations and increases their time
    pub fn update_animations(&mut self, delta_time: f32) {
        let mut animation_query = self.world.query::<&mut HumanWalkAnimation>();

        for (_, anim) in animation_query.iter() {
            anim.update(delta_time);
        }
    }

    pub fn update_screen(&mut self, delta_time: f32) {}

    //this updates the collision hashmap to remember where entitites are for collision and interaction
    pub fn update_collisions(&mut self) {
        let Screen::Overworld(map) = &self.screen else { panic!(); };

        let mut collision_query = self
            .world
            .query::<(&mut Position, Option<&MovingEntity>)>()
            .with::<&Collision>();

        self.collisions = HashMap::new();
        //self.collisions.reserve(collision_query.iter().len());

        for (entity, (Position(c), moving)) in collision_query.iter() {
            self.collisions.insert(c.to_usize(map.size_x), entity);
            if let Some(moving) = moving {
                let next_pos = match moving.moving {
                    MovingState::Moving(Direction::Left) => Vec2((c.0 - 1.0).ceil(), c.1),
                    MovingState::Moving(Direction::Right) => Vec2((c.0 + 1.0).floor(), c.1),
                    MovingState::Moving(Direction::Up) => Vec2(c.0, (c.1 - 1.0).ceil()),
                    MovingState::Moving(Direction::Down) => Vec2(c.0, (c.1 + 1.0).floor()),
                    _ => continue,
                };

                self.collisions
                    .insert(next_pos.to_usize(map.size_x), entity);
            }
        }
    }

    //updates player moving state
    pub fn update_player_moving(&mut self, moving_state: MovingState) {
        let mut moving = self
            .world
            .query_one_mut::<&mut MovingEntity>(self.player)
            .unwrap();

        moving.try_moving = moving_state;
    }

    //updates player sprinting state
    pub fn update_player_sprinting(&mut self, sprinting: bool) {
        let mut moving = self
            .world
            .query_one_mut::<&mut MovingEntity>(self.player)
            .unwrap();

        moving.try_sprinting = sprinting;
    }

    //runs physics for every object that is capable of moving
    pub fn update_moving_objects(&mut self, delta_time: f32) {
        let Screen::Overworld(map) = &self.screen else { panic!(); };

        use MovingState::*;

        let mut moving_query = self.world.query::<(
            &mut Position,
            &mut MovingEntity,
            &mut HumanWalkAnimation,
            Option<&Npc>,
        )>();
        for (id, (pos, moving, animation, npc)) in moving_query.iter() {
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

                    let &mut Position(Vec2(x, y)) = pos;

                    let Vec2(target_x, target_y) = match new_rotation {
                        Direction::Left => Vec2((x - 1.0).ceil(), y),
                        Direction::Right => Vec2((x + 1.0).floor(), y),
                        Direction::Up => Vec2(x, (y - 1.0).ceil()),
                        Direction::Down => Vec2(x, (y + 1.0).floor()),
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

            let &mut Position(Vec2(x, y)) = pos;

            let Vec2(target_x, target_y) = match moving.moving {
                Moving(Direction::Left) => Vec2((x - 1.0).ceil(), y),
                Moving(Direction::Right) => Vec2((x + 1.0).floor(), y),
                Moving(Direction::Up) => Vec2(x, (y - 1.0).ceil()),
                Moving(Direction::Down) => Vec2(x, (y + 1.0).floor()),
                Idle | CenterTile => panic!("Should not happen"),
            };

            let speed = delta_time
                * match moving.sprinting {
                    true => RUN_SPEED,
                    false => WALK_SPEED,
                };

            let (dx, dy) = match compute_direction(Vec2(x, y), Vec2(target_x, target_y)) {
                Direction::Up => (0.0, -speed),
                Direction::Down => (0.0, speed),
                Direction::Left => (-speed, 0.0),
                Direction::Right => (speed, 0.0),
            };

            //set new position
            *pos = Position(Vec2(x + dx, y + dy));

            let &mut Position(Vec2(x, y)) = pos;

            //check if we have passed the tile we were trying to get to
            if (x, y) == (target_x, target_y)
                || dx != 0.0 && (target_x - x).signum() != dx.signum()
                || dy != 0.0 && (target_y - y).signum() != dy.signum()
            {
                *pos = Position(Vec2(target_x, target_y));
                moving.moving = match moving.try_moving {
                    Idle => Idle,
                    _ => CenterTile,
                };

                if id == self.player {
                    self.events
                        .push(Event::PlayerMoved(Vec2(target_x, target_y)))
                } else if let Some(npc) = npc {
                    self.events.push(Event::NpcMoved(id));
                }
            }
        }
    }
}
