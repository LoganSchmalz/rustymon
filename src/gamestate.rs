/****************************************************/
// Created by: Logan Schmalz
// Description: Logic and data structures for storing and interacting with the state of the game
// including information about the world as well as updating and transitioning between states of the game
/****************************************************/
use std::{clone::Clone, collections::HashMap};

use hecs::{CommandBuffer, Entity, World};
use rand::{distributions::Uniform, rngs::ThreadRng, Rng};
use sdl2::{rect::Rect, video::WindowContext};
use enum_map::EnumMap;
use std::collections::VecDeque;

use crate::{
    components::{animation::HumanWalkAnimation, bag::Bag, sprite::Sprite, stray::*, *},
    constants::{FADE_FRAMES, FADE_TIME, RANDOM_ENCOUNTER_CHANCE},
    font_manager::FontManager,
    gamestate::battle::BattleState,
    menu::{
        main_menu::MainMenu,
        moves_menu::MovesMenu,
        textbox::Textbox,
        Menu, MenuManager,
    },
    render::Renderer,
    resource_manager::TextureManager,
    tilemap::TileMap,
    vec2::{Direction, Vec2},
};

use self::{
    battle::Battle,
    event::Event,
    input::{Control, KeyState},
};

pub mod battle;
pub mod event;
mod input;
mod updates;

pub enum Screen {
    MainMenu,
    Overworld(TileMap),
    Battle(Battle),
}

#[derive(Copy, Clone, Debug)]
pub enum TransitionType {
    Fade,
    Win,
    Loss,
}

#[derive(Debug)]
pub enum Transition {
    None,
    Transitioning {
        transition_type: TransitionType,
        time: f32,
        full: bool,
    },
}

pub struct State {
    pub screen: Screen,
    pub next_screen: Screen,
    pub input: EnumMap<Control, KeyState>,
    pub paused: bool,
    pub allow_input: bool,
    pub world: World,
    pub cmd: CommandBuffer,
    pub events: Vec<Event>,
    pub menus: MenuManager,
    pub player: Entity,
    pub collisions: HashMap<usize, Entity>,
    pub rng: ThreadRng,
    pub transition: Transition,
}

impl Default for State {
    //default constructor for State object
    //initializes world, command buffer, menus, and all entities within the map including the player
    fn default() -> Self {
        let mut world = World::new();
        let mut cmd = CommandBuffer::new();

        //here we create a bunch of example entities for the default world
        //spawn player
        let player = world.spawn((
            //entity type
            Player,
            //position
            Position(Vec2(14f32, 15f32)),
            //movement of entity
            MovingEntity::new(),
            //texture for entity
            Sprite::character(String::from("assets/char-sprites/augosprite.png")),
            //collision for entity
            Collision,
            //initial animation state
            HumanWalkAnimation {
                rotation: Direction::Down,
                time: (1.0, 0.0),
                left_leg: true,
                sprinting: false,
            },
            //inventory
            Bag::new(),
        ));

        //spawn door object
        let _door = world.spawn((
            //position
            Position(Vec2(2f32, 0f32)),
            //texture
            Sprite {
                texture: String::from("assets/tiles/tilesprites.png"),
                src: Rect::new(96, 0, 16, 16),
                ..Default::default()
            },
            //collision
            Collision,
        ));

        //spawn dad NPC
        let _dad = world.spawn((
            //position
            Position(Vec2(16f32, 16f32)),
            //movement
            MovingEntity {
                moving: MovingState::Moving(Direction::Left),
                try_moving: MovingState::Moving(Direction::Left),
                rotation: Direction::Left,
                ..Default::default()
            },
            //texture
            Sprite::character(String::from("assets/char-sprites/dadsprite.png")),
            //collision
            Collision,
            Npc {
                //interaction dialogue
                says: "Hi hungry, I'm dad! Nice try, little child --> you are bad!".to_string(),
                //walking path
                path: Some(WalkingPath {
                    path: vec![
                        Direction::Left,
                        Direction::Up,
                        Direction::Right,
                        Direction::Down,
                    ],
                    index: 0,
                }),
            },
            //initial animation state
            HumanWalkAnimation {
                rotation: Direction::Left,
                time: (1.0, 0.0),
                left_leg: true,
                sprinting: false,
            },
        ));

        //spawn shaman NPC
        let _shaman = world.spawn((
            //position
            Position(Vec2(30f32, 15f32)),
            //movement
            MovingEntity {
                moving: MovingState::Moving(Direction::Left),
                try_moving: MovingState::Moving(Direction::Left),
                rotation: Direction::Left,
                ..Default::default()
            },
            //texture
            Sprite::character(String::from("assets/char-sprites/shamansprite.png")),
            //collision
            Collision,
            Npc {
                //interaction dialogue
                says: "You have much to learn.".to_string(),
                //walking path
                path: Some(WalkingPath {
                    path: vec![
                        Direction::Left,
                        Direction::Left,
                        Direction::Right,
                        Direction::Right,
                    ],
                    index: 0,
                }),
            },
            //initial animation state
            HumanWalkAnimation {
                rotation: Direction::Left,
                time: (1.0, 0.0),
                left_leg: true,
                sprinting: false,
            },
        ));

        //spawn sika NPC
        let _sika = world.spawn((
            //position
            Position(Vec2(30f32, 24f32)),
            //movement
            MovingEntity {
                moving: MovingState::Moving(Direction::Left),
                try_moving: MovingState::Moving(Direction::Left),
                rotation: Direction::Left,
                ..Default::default()
            },
            //texture
            Sprite::character(String::from("assets/char-sprites/sikasprite.png")),
            //collision
            Collision,
            Npc {
                //interaction dialogue
                says: "You are small".to_string(),
                //walking path
                path: Some(WalkingPath {
                    path: vec![
                        Direction::Up,
                        Direction::Up,
                        Direction::Down,
                        Direction::Down,
                    ],
                    index: 0,
                }),
            },
            //initial animation state
            HumanWalkAnimation {
                rotation: Direction::Left,
                time: (1.0, 0.0),
                left_leg: true,
                sprinting: false,
            },
        ));


        //spawn mom NPC
        let _mom = world.spawn((
            //position
            Position(Vec2(16f32, 24f32)),
            //movement
            MovingEntity {
                moving: MovingState::Moving(Direction::Left),
                try_moving: MovingState::Moving(Direction::Left),
                rotation: Direction::Left,
                ..Default::default()
            },
            //texture
            Sprite::character(String::from("assets/char-sprites/momsprite.png")),
            //collision
            Collision,
            Npc {
                //interaction dialogue
                says: "Son, I'm not real.".to_string(),
                //walking path
                path: Some(WalkingPath {
                    path: vec![
                        Direction::Up,
                        Direction::Up,
                        Direction::Down,
                        Direction::Down,
                    ],
                    index: 0,
                }),
            },
            //initial animation state
            HumanWalkAnimation {
                rotation: Direction::Left,
                time: (1.0, 0.0),
                left_leg: true,
                sprinting: false,
            },
        ));

        //spawn Ilasiak NPC
        let _ilasiak = world.spawn((
            //position
            Position(Vec2(22f32, 18f32)),
            //movement
            MovingEntity {
                moving: MovingState::Moving(Direction::Left),
                try_moving: MovingState::Moving(Direction::Left),
                rotation: Direction::Left,
                ..Default::default()
            },
            //texture
            Sprite::character(String::from("assets/char-sprites/ilasiaksprite.png")),
            //collision
            Collision,
            Npc {
                //interaction dialogue
                says: "Hey bub, move outta the way!".to_string(),
                //walking path
                path: Some(WalkingPath {
                    path: vec![
                        Direction::Up,
                        Direction::Right,
                        Direction::Down,
                        Direction::Left,
                    ],
                    index: 0,
                }),
            },
            //initial animation state
            HumanWalkAnimation {
                rotation: Direction::Left,
                time: (1.0, 0.0),
                left_leg: true,
                sprinting: false,
            },
        ));

        //spawn berry object
        let _berry1 = world.spawn((
            //position
            Position(Vec2(16f32, 8f32)),
            //texture
            Sprite::berry(),
            //collision
            Collision,
            //type of entity
            GroundItem {
                item: bag::Item::Berry,
                amount: 1,
            },
        ));

        //spawn berry object
        let _berry2 = world.spawn((
            //position
            Position(Vec2(16f32, 16f32)),
            //texture
            Sprite::berry(),
            //collision
            Collision,
            //type of entity
            GroundItem {
                item: bag::Item::Berry,
                amount: 1,
            },
        ));

        //open menu manager
        let mut menus = MenuManager::new();
        //open starting menu
        menus.open_menu(MainMenu::new().into());

        //initialize all variables in State
        Self {
            screen: Screen::Overworld(TileMap::load(0)),
            next_screen: Screen::Overworld(TileMap::load(0)),
            //screen: Screen::Battle(TEST_BATTLE),
            input: EnumMap::default(),
            paused: true,
            allow_input: true,
            world,
            cmd,
            events: vec![],
            menus,
            player,
            collisions: HashMap::new(),
            rng: rand::thread_rng(),
            transition: Transition::None,
        }
    }
}

impl State {
    //function for rendering the entire game
    //takes in the game state (self), renderer, texture manager, font manager, time, and the tilemap
    //returns success or failure
    pub fn render(
        &mut self,
        renderer: &mut Renderer,
        //_canvas: &Canvas<Window>,
        texture_manager: &mut TextureManager<WindowContext>,
        font_manager: &FontManager,
        delta_time: f32,
        map: &mut TileMap,
    ) -> Result<(), String> {
        match &mut self.screen {
            Screen::MainMenu => {}
            Screen::Overworld(map) => {
                //render overworld if we are in the overworld
                renderer.render_overworld(
                    texture_manager,
                    font_manager,
                    &self.world,
                    map,
                    &mut self.menus,
                )?;
            }
            Screen::Battle(battle) => {
                //render battle if we are in battle
                renderer.render_battle(
                    //render battle screen dynamically
                    texture_manager,
                    font_manager,
                    battle,
                    &mut self.menus,
                    &self.world,
                )?;
            }
        }

        if let Transition::Transitioning { time, full, .. } = &mut self.transition {
            //render transition for constant amount of time
            if *time <= FADE_TIME {
                *time += delta_time;
            } else {
                self.transition = Transition::None;
                self.allow_input = true;
                return Ok(());
            }

            if !*full && FADE_FRAMES / 2 >= FADE_FRAMES * (1.0 - *time / FADE_TIME).round() as i32 {
                self.events.push(Event::TransitionFull);
                *full = true;
            }

            renderer.render_transition(texture_manager, &self.transition)?;
        }

        //perform rendering on canvas
        renderer.present();

        Ok(())
    }

    //function for updating the state of the game
    //takes in the current game state, time, tilemap, and font manager
    //returns success or failure
    pub fn update(
        &mut self,
        delta_time: f32,
        _map: &mut TileMap,
        font_manager: &FontManager,
    ) -> Result<(), String> {
        //determine correct input handler
        match &self.screen {
            Screen::Overworld(_) => {
                //handle overworld input if in overworld, including handling input for menus opened while in the overworld
                if self.allow_input {
                    if self.menus.is_open() {
                        self.update_player_moving(MovingState::Idle);
                        self.paused = self.handle_input_menus(font_manager);
                    } else {
                        self.handle_input_gameplay(font_manager);
                    }
                } else {
                    self.update_player_moving(MovingState::Idle);
                }

                if !self.paused {
                    self.update_moving_objects(delta_time);
                    self.update_collisions();
                    self.update_animations(delta_time);
                }
            }
            Screen::MainMenu => {}
            Screen::Battle(_) => {
                //handle battle input while in battle
                if self.allow_input {
                    self.handle_input_battle(font_manager);
                }
            }
        }

        if !self.paused {
            //process game events as long as the game is not paused
            self.process_events(font_manager);
        }
        Ok(())
    }

    //function for processing game events
    //takes in the currrent state and the font manager
    pub fn process_events(&mut self, font_man: &FontManager) {
        //while there are events in the queue, perform the event
        while let Some(event) = self.events.pop() {
            match event {
                Event::PlayerMoved(pos) => {
                    //if the player moved, process all possible consequences
                    if let Screen::Overworld(map) = &self.screen {
                        if map.check_encounter(pos)
                            //if encounter occurred
                            && self.rng.gen::<f32>() <= RANDOM_ENCOUNTER_CHANCE
                        {
                            //set next screen to battle
                            self.next_screen = Screen::Battle(Battle::new(
                                [
                                    Some(Stray::cespae(true)),
                                    Some(Stray::palliub(true)),
                                    None,
                                    Some(Stray::catis(true)),
                                ],
                                [
                                    Some(Stray::carerus(false)),
                                    None,
                                    Some(Stray::rubridum(false)),
                                    Some(Stray::omikae(false)),
                                ],
                            ));
                            //set and run transition
                            self.transition = Transition::Transitioning {
                                transition_type: TransitionType::Fade,
                                time: 0.0,
                                full: false,
                            };
                            //freeze character
                            self.allow_input = false;
                        }
                    }
                }
                Event::TransitionFull => {
                    //after transition is done, swap screens and open necessary menus
                    std::mem::swap(&mut self.screen, &mut self.next_screen);
                    if matches!(self.screen, Screen::Battle(_)) {
                        if let Screen::Battle(battle) = &mut self.screen {
                            battle.menus.open_menu(
                                MovesMenu::new(
                                    battle.player_strays[battle.turn_order[0]]
                                        .as_ref()
                                        .unwrap()
                                        .moves
                                        .clone(),
                                )
                                .into(),
                            );
                        }
                    }
                }

                Event::NpcMoved(entity) => {
                    //process consequences of NPC movement
                    let (moving, npc) = self
                        .world
                        .query_one_mut::<(&mut MovingEntity, &mut Npc)>(entity)
                        .unwrap();
                    if let Some(path) = &mut npc.path {
                        //move NPC in direction defined by their path
                        path.advance();
                        moving.rotation = path.direction();
                        moving.try_moving = MovingState::Moving(path.direction());
                    }
                }
                Event::BattleAttack(selection) => {
                    //process move selection in battles
                    println!("{:?}", &selection); //print the selected move
                    let Screen::Battle(battle) = &mut self.screen else { panic!() };
                    match selection.kind {
                        MoveKind::Healing => {
                            battle.battle_state = BattleState::SelectingFriendlyStray;
                            battle.selected_stray = battle.get_left_player_stray(Some(0));
                        }
                        MoveKind::Damage => {
                            battle.battle_state = BattleState::SelectingOpponentStray;
                            battle.selected_stray = battle.get_left_opponent_stray(Some(0));
                        }
                    }
                    battle.selected_move = Some(selection); //set the selected move of the battle to be the move we selected to trigger the event
                    battle.menus.close_menu(); //close move menu
                }
                Event::AttackStray(idx) => {
                    //process attacks in battles 
                    let Screen::Battle(battle) = &mut self.screen else { panic!() };

                    battle.selected_stray = None;

                    //display selection menu based on type of move selected
                    match battle.battle_state {
                        //target friendly strays
                        BattleState::SelectingFriendlyStray => {
                            if let Some(stray) = &mut battle.player_strays[idx] {
                                if let Some(mv) = &mut battle.selected_move {
                                    stray.cur_hp = stray.cur_hp + mv.power;
                                    if stray.cur_hp > stray.hp {
                                        stray.cur_hp = stray.hp;
                                    }
                                }
                            }
                        }
                        //target enemy strays
                        BattleState::SelectingOpponentStray => {
                            if let Some(stray) = &mut battle.opponent_strays[idx] {
                                let rand_int: f32 = self.rng.gen();
                                let mut damage = 0;
                                if let Some(mv) = &mut battle.selected_move {
                                    if rand_int < (mv.accuracy as f32 / 100 as f32) {
                                        damage = mv.power;
                                    }
                                }
                                stray.cur_hp = stray.cur_hp - damage; //subtract hp from selected stray by the amount of damage the move does

                                if stray.cur_hp <= 0 {
                                    battle.opponent_strays[idx] = None;
                                }
                            }
                        }
                        _ => panic!("something bad happened"),
                    }

                    //run AI for enemy strays
                    'opponent_ai: loop {
                        println!("broken");
                        //TODO: REMOVE THIS  LOOP, INSTEAD OF JUST ITERATING OVER TURN ORDER UNTIL YOU GET TO A PLAYER-OWNED STRAY, THERE SHOULD BE ENEMY AI
                        if let Some(i) = &battle.turn_order.pop_front() {
                            //remove stray that just went from the front of the queue
                            if *i > 3 {
                                if battle.opponent_strays[*i - 4].is_some() {
                                    battle.turn_order.push_back(*i); //if stray that just moved is still alive, add it back to the back of the queue
                                }
                            } else {
                                if battle.player_strays[*i].is_some() {
                                    battle.turn_order.push_back(*i); //if stray that just moved is still alive, add it back to the back of the queue
                                }
                            }
                        }
                        //println!("{}", battle.turn_order[0].clone().species);
                        if battle.turn_order[0] < 4 {
                            //if current turn is a player's stray
                            if battle.player_strays[battle.turn_order[0]].is_none() {
                                battle.turn_order.pop_front();
                            } else {
                                battle.menus.open_menu(
                                    MovesMenu::new(
                                        battle.player_strays[battle.turn_order[0]]
                                            .as_ref()
                                            .unwrap()
                                            .moves
                                            .clone(),
                                    )
                                    .into(),
                                ); //open moves menu
                                break; //continue adjusting turn order until it's one of the player's stray's turn
                            }
                        } else if battle.opponent_strays[battle.turn_order[0] - 4]
                            .as_ref()
                            .is_some()
                        {
                            //if current turn is an enemy stray

                            if battle.player_strays.iter().all(|x| x.is_none()) {
                                break 'opponent_ai;
                            }

                            let mut rand_p_stray: usize = 0;
                            let mut rand_move: usize = 0;
                            loop {
                                println!("broken1");
                                //TODO: probly wanna change this cause these nested loops are cringe
                                rand_p_stray = self.rng.gen_range(0..4);
                                dbg!(rand_p_stray);
                                dbg!(&battle.player_strays);
                                //println!("rand stray: {}", rand_p_stray);

                                if battle.player_strays[rand_p_stray].is_some() {
                                    //loop until enemy selects a valid target
                                    break;
                                }
                            }
                            loop {
                                println!("broken2");
                                //TODO: probly wanna change this cause these nested loops are cringe
                                rand_move = self.rng.gen::<usize>() % 4;
                                //println!("rand move: {}", rand_move);
                                if battle.opponent_strays[battle.turn_order[0] - 4]
                                    .as_ref()
                                    .unwrap()
                                    .moves[rand_move]
                                    .is_some()
                                {
                                    //loop until enemy selects a valid move
                                    break;
                                }
                            }

                            //do random move on random target
                            let rand_int: f32 = self.rng.gen();
                            let mut damage = 0;
                            let mv = battle.opponent_strays[battle.turn_order[0] - 4]
                                .as_ref()
                                .unwrap()
                                .moves[rand_move]
                                .as_ref()
                                .unwrap();
                            if rand_int < (mv.accuracy as f32 / 100 as f32) {
                                //calculate damage
                                damage = mv.power;
                            }
                            if let Some(p_stray) = &mut battle.player_strays[rand_p_stray] {
                                p_stray.cur_hp -= damage; //subtract hp from selected stray by the amount of damage the move does
                                //display text stating what the stray just did
                                battle.menus.open_menu(
                                    Textbox::new(
                                        &("".to_owned()
                                            + &String::from(
                                                &battle.opponent_strays[battle.turn_order[0] - 4]
                                                    .as_ref()
                                                    .unwrap()
                                                    .species,
                                            )
                                            + " used "
                                            + &mv.name
                                            + " on "
                                            + &p_stray.species
                                            + "!"),
                                        font_man,
                                    )
                                    .into(),
                                );
                                //TODO fix bug where only one enemy turn displays at a time
                                if p_stray.cur_hp <= 0 {
                                    //kill stray if it has no hp
                                    battle.player_strays[rand_p_stray] = None;
                                }
                            }
                        }
                    }

                    //go to next battle state
                    battle.battle_state = BattleState::SelectingMove;

                    //check for win condition (all enemy strays dead), end battle and show win screen
                    if battle.opponent_strays.iter().all(|x| x.is_none()) {
                        battle.menus.close_menu();
                        self.transition = Transition::Transitioning {
                            transition_type: TransitionType::Win,
                            time: 0.0,
                            full: false,
                        };
                        self.allow_input = false;
                    }
                    //check for loss condition (all player strays dead), end battle and show loss screen
                    if battle.player_strays.iter().all(|x| x.is_none()) {
                        battle.menus.close_menu();
                        self.transition = Transition::Transitioning {
                            transition_type: TransitionType::Loss,
                            time: 0.0,
                            full: false,
                        };
                        self.allow_input = false;
                    }
                }
            }
        }
    }

    //function for checking for collisions between entities
    //takes in a position and returns a bool
    pub fn check_collision(&self, position: &Vec2) -> bool {
        let Screen::Overworld(map) = &self.screen else { panic!(); };
        map.check_collision(*position)
            || self.collisions.contains_key(&position.to_usize(map.size_x))
    }

    //function for player interaction with entities
    //takes in the font manager
    pub fn try_player_interaction(&mut self, font_man: &FontManager) {
        //player should only be able to interact with entities in the overworld
        let Screen::Overworld(map) = &self.screen else { panic!(); };

        //check for position of player
        let (&Position(Vec2(x, y)), moving) = self
            .world
            .query_one_mut::<(&Position, &MovingEntity)>(self.player)
            .unwrap();

        if moving.moving != MovingState::CenterTile && moving.moving != MovingState::Idle {
            return;
        }

        let temp_pos = match moving.rotation {
            Direction::Left => Vec2(x - 1.0, y),
            Direction::Right => Vec2(x + 1.0, y),
            Direction::Up => Vec2(x, y - 1.0),
            Direction::Down => Vec2(x, y + 1.0),
        };

        //check for an entity based on player position in the collisions map
        let interact_entity = self
            .collisions
            .get_key_value(&temp_pos.to_usize(map.size_x));

        //if it matches, run the list of interactions it comes with
        if let Some((_, &entity)) = interact_entity {
            let npc = self.world.query_one_mut::<&Npc>(entity);
            if let Ok(Npc { says, .. }) = npc {
                self.menus.open_menu(Textbox::new(says, font_man).into());
                return;
            }

            if let Ok(&GroundItem { item, amount }) =
                self.world.query_one_mut::<&GroundItem>(entity)
            {
                if let Ok(bag) = self.world.query_one_mut::<&mut Bag>(self.player) {
                    if bag.add_item(item, amount) {
                        self.menus.open_menu(
                            Textbox::new(
                                &format!("You picked up {} (x{}).", item, amount),
                                font_man,
                            )
                            .into(),
                        );
                        self.world.despawn(entity).unwrap();
                    }
                };
            }
        }
    }
}
