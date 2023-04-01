use std::{clone::Clone, collections::HashMap};

use hecs::{CommandBuffer, Entity, World};
use rand::{distributions::Uniform, rngs::ThreadRng, Rng};
use sdl2::{rect::Rect, video::WindowContext};

use enum_map::EnumMap;
use std::collections::VecDeque;


use crate::{
    components::{animation::HumanWalkAnimation, bag::Bag, sprite::Sprite, stray::*, *},
    constants::RANDOM_ENCOUNTER_CHANCE,
    font_manager::FontManager,
    menu::{
        Menu,
        battle_select_stray::{self, BattleSelectStray},
        main_menu::MainMenu,
        moves_menu::MovesMenu,
        textbox::Textbox,
        MenuManager,
    },
    render::{Renderer, Transition},
    resource_manager::TextureManager,
    tilemap::TileMap,
    vec2::{Direction, Vec2},
};

use self::{
    event::Event,
    input::{Control, KeyState},
};

pub mod event;
mod input;
mod updates;

#[derive(Clone)]
pub enum Screen {
    MainMenu,
    Overworld,
    Battle(Battle),
}

#[derive(Default, Clone)]
pub struct Battle {
    pub player_strays: [Option<Stray>; 4],
    pub opponent_strays: [Option<Stray>; 4],
    pub selected_move: Option<Move>,
    pub battle_state: BattleState,
    pub turn_order: VecDeque<Stray>,
}

impl Battle {
    pub fn new(player_strays: [Option<Stray>; 4], opponent_strays: [Option<Stray>; 4]) -> Battle {
            let mut turn_order = VecDeque::new();
            for stray in player_strays.iter().chain(opponent_strays.iter()) {
                if let Some(stray) = stray {
                    turn_order.push_back(stray.clone());
                }
            }

            turn_order.make_contiguous().sort_by(|a, b| b.spd.cmp(&a.spd));
            
            Battle {
                player_strays,
                opponent_strays,
                selected_move: None,
                battle_state: BattleState::SelectingMove,
                turn_order,
            }
        }
}


#[derive(Clone, Default)]
pub enum BattleState {
    #[default]
    SelectingMove,
    SelectingOpponentStray,
    SelectingFriendlyStray,
}

pub struct State {
    pub screen: Screen,
    pub next_screen: Screen,
    pub input: EnumMap<Control, KeyState>,
    pub paused: bool,
    pub world: World,
    pub cmd: CommandBuffer,
    pub events: Vec<Event>,
    pub map: TileMap,
    pub menus: MenuManager,
    pub player: Entity,
    pub collisions: HashMap<usize, Entity>,
    pub rng: ThreadRng,
    pub transitioning: bool,
    pub trans: Transition,
}

impl Default for State {
    fn default() -> Self {
        let mut world = World::new();
        let mut cmd = CommandBuffer::new();

        //here we create a bunch of example entities for the default world
        let player = world.spawn((
            Player,
            Position(Vec2(14f32, 15f32)),
            MovingEntity::new(),
            Sprite::character(String::from("assets/char-sprites/augosprite.png")),
            Collision,
            HumanWalkAnimation {
                rotation: Direction::Down,
                time: (1.0, 0.0),
                left_leg: true,
                sprinting: false,
            },
            Bag::new(),
        ));

        let _door = world.spawn((
            Position(Vec2(2f32, 0f32)),
            Sprite {
                texture: String::from("assets/tiles/tilesprites.png"),
                src: Rect::new(96, 0, 16, 16),
                ..Default::default()
            },
            Collision,
        ));

        let _dad = world.spawn((
            Position(Vec2(16f32, 16f32)),
            MovingEntity {
                moving: MovingState::Moving(Direction::Left),
                try_moving: MovingState::Moving(Direction::Left),
                rotation: Direction::Left,
                ..Default::default()
            },
            Sprite::character(String::from("assets/char-sprites/dadsprite.png")),
            Collision,
            Npc {
                says: "Hi hungry, I'm dad! Nice try, little child --> you are bad!".to_string(),
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
            HumanWalkAnimation {
                rotation: Direction::Left,
                time: (1.0, 0.0),
                left_leg: true,
                sprinting: false,
            },
        ));

        let _shaman = world.spawn((
            Position(Vec2(30f32, 15f32)),
            MovingEntity {
                moving: MovingState::Moving(Direction::Left),
                try_moving: MovingState::Moving(Direction::Left),
                rotation: Direction::Left,
                ..Default::default()
            },
            Sprite::character(String::from("assets/char-sprites/shamansprite.png")),
            Collision,
            Npc {
                says: "You have much to learn.".to_string(),
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
            HumanWalkAnimation {
                rotation: Direction::Left,
                time: (1.0, 0.0),
                left_leg: true,
                sprinting: false,
            },
        ));

        let _sika = world.spawn((
            Position(Vec2(30f32, 24f32)),
            MovingEntity {
                moving: MovingState::Moving(Direction::Left),
                try_moving: MovingState::Moving(Direction::Left),
                rotation: Direction::Left,
                ..Default::default()
            },
            Sprite::character(String::from("assets/char-sprites/sikasprite.png")),
            Collision,
            Npc {
                says: "You are small".to_string(),
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
            HumanWalkAnimation {
                rotation: Direction::Left,
                time: (1.0, 0.0),
                left_leg: true,
                sprinting: false,
            },
        ));

        let _mom = world.spawn((
            Position(Vec2(16f32, 24f32)),
            MovingEntity {
                moving: MovingState::Moving(Direction::Left),
                try_moving: MovingState::Moving(Direction::Left),
                rotation: Direction::Left,
                ..Default::default()
            },
            Sprite::character(String::from("assets/char-sprites/momsprite.png")),
            Collision,
            Npc {
                says: "Son, I'm not real.".to_string(),
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
            HumanWalkAnimation {
                rotation: Direction::Left,
                time: (1.0, 0.0),
                left_leg: true,
                sprinting: false,
            },
        ));

        let _ilasiak = world.spawn((
            Position(Vec2(22f32, 18f32)),
            MovingEntity {
                moving: MovingState::Moving(Direction::Left),
                try_moving: MovingState::Moving(Direction::Left),
                rotation: Direction::Left,
                ..Default::default()
            },
            Sprite::character(String::from("assets/char-sprites/ilasiaksprite.png")),
            Collision,
            Npc {
                says: "Hey bub, move outta the way!".to_string(),
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
            HumanWalkAnimation {
                rotation: Direction::Left,
                time: (1.0, 0.0),
                left_leg: true,
                sprinting: false,
            },
        ));



        let _berry1 = world.spawn((
            Position(Vec2(16f32, 8f32)),
            Sprite::berry(),
            Collision,
            GroundItem {
                item: bag::Item::Berry,
                amount: 1,
            },
        ));

        let _berry2 = world.spawn((
            Position(Vec2(16f32, 16f32)),
            Sprite::berry(),
            Collision,
            GroundItem {
                item: bag::Item::Berry,
                amount: 1,
            },
        ));

        let mut menus = MenuManager::new();
        menus.open_menu(MainMenu::new().into());

        Self {
            screen: Screen::Overworld,
            next_screen: Screen::Overworld,
            //screen: Screen::Battle(TEST_BATTLE),
            input: EnumMap::default(),
            paused: true,
            world,
            cmd,
            events: vec![],
            map: TileMap::load(0),
            menus,
            player,
            collisions: HashMap::new(),
            rng: rand::thread_rng(),
            transitioning: false,
            trans: Transition::Fade,
        }
    }
}

impl State {
    pub fn render(
        &mut self,
        renderer: &mut Renderer,
        //_canvas: &Canvas<Window>,
        texture_manager: &mut TextureManager<WindowContext>,
        font_manager: &FontManager,
        delta_time: f32,
        map: &mut TileMap,
    ) -> Result<(), String> {
        renderer.trans = self.trans;
        renderer.transitioning = self.transitioning;

        match &self.screen {
            Screen::MainMenu => {}
            Screen::Overworld => {
                let transition_done = renderer.render_overworld(
                    texture_manager,
                    font_manager,
                    delta_time,
                    &self.world,
                    map,
                    &mut self.menus,
                )?;
                if self.transitioning && transition_done {
                    self.events.push(Event::TransitionFull);
                    self.transitioning = false;
                }
            }
            Screen::Battle(battle) => {
                let transition_done = renderer.render_battle(
                    //render battle screen dynamically
                    texture_manager,
                    font_manager,
                    delta_time,
                    battle,
                    &mut self.menus,
                    &self.world,
                )?;
                if self.transitioning && transition_done {
                    self.events.push(Event::TransitionFull);
                    self.transitioning = false;
                }
            }
        }

        self.transitioning = renderer.transitioning;

        Ok(())
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        _map: &mut TileMap,
        font_manager: &FontManager,
    ) -> Result<(), String> {
        //determine correct input handler
        if self.menus.is_open() {
            self.update_player_moving(MovingState::Idle);
            self.paused = self.handle_input_menus(font_manager);
        } else {
            self.handle_input_gameplay(font_manager);
        }

        //do any physics/animation updates
        if !self.paused {
            self.update_moving_objects(delta_time);
            self.update_collisions();
            self.update_animations(delta_time);
            self.process_events(font_manager);
        }
        Ok(())
    }

    pub fn process_events(&mut self, font_man: &FontManager) {
        while let Some(event) = self.events.pop() {
            match event {
                Event::PlayerMoved(pos) => {
                    if self.map.check_encounter(pos)
                        && self.rng.gen::<f32>() <= RANDOM_ENCOUNTER_CHANCE
                    {
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
                            ]
                        ));
                        self.trans = Transition::Fade;
                        self.transitioning = true;
                    }
                }
                Event::TransitionFull => {
                        std::mem::swap(&mut self.screen, &mut self.next_screen);
                        if matches!(self.screen, Screen::Battle(_)) {
                            if let Screen::Battle(battle) = &mut self.screen {
                                self.menus.open_menu(MovesMenu::new(battle.turn_order[0].moves.clone()).into());
                            }
                        }
                        self.transitioning = false;
                }
                Event::NpcMoved(entity) => {
                    let (moving, npc) = self
                        .world
                        .query_one_mut::<(&mut MovingEntity, &mut Npc)>(entity)
                        .unwrap();
                    if let Some(path) = &mut npc.path {
                        path.advance();
                        moving.rotation = path.direction();
                        moving.try_moving = MovingState::Moving(path.direction());
                    }
                }
                Event::BattleAttack(selection) => {
                    println!("{:?}", selection); //print the selected move
                    if let Screen::Battle(b) = &mut self.screen {
                        //if the current screen is a battle
                        b.selected_move = Some(selection); //set the selected move of the battle to be the move we selected to trigger the event
                        b.battle_state = BattleState::SelectingOpponentStray; //change battle state to next menu
                        self.menus.close_menu(); //close move menu
                        self.menus.open_menu(BattleSelectStray::new().into()); //open opponent selection menu
                    }
                }
                Event::AttackStray(idx) => {
                    if let Screen::Battle(battle) = &mut self.screen {
                        //if the current screen is a battle
                        if let Some(stray) = &mut battle.opponent_strays[idx] {
                            self.menus.close_menu(); //close opponent selection menu
                            self.menus.close_menu(); //close move selection menu?
                            //stray = opponent at the index chosen
                            let mut rand_int: f32 = self.rng.gen();
                            let mut damage = 0;
                            if let Some(mv) = &mut battle.selected_move {
                                if rand_int < (mv.accuracy as f32/100 as f32) {
                                    damage = mv.power;
                                }
                            }
                            stray.cur_hp = stray.cur_hp - damage; //subtract hp from selected stray by the amount of damage the move does
                            
                            if stray.cur_hp <= 0 {
                                battle.opponent_strays[idx] = None;
                            }
                            loop { //TODO: REMOVE THIS  LOOP, INSTEAD OF JUST ITERATING OVER TURN ORDER UNTIL YOU GET TO A PLAYER-OWNED STRAY, THERE SHOULD BE ENEMY AI
                                if let Some(s) = &battle.turn_order.pop_front() { //remove stray that just went from the front of the queue
                                    if s.cur_hp > 0 {
                                        battle.turn_order.push_back(s.clone()); //if stray that just moved is still alive, add it back to the back of the queue
                                    }
                                }
                                //println!("{}", battle.turn_order[0].clone().species);
                                if battle.turn_order[0].owner { //if current turn is a player's stray
                                    break; //continue adjusting turn order until it's one of the player's stray's turn
                                } else { //if current turn is an enemy stray
                                    let mut rand_p_stray: usize = 0;
                                    let mut rand_move: usize = 0;
                                    loop { //TODO: probly wanna change this cause these nested loops are cringe
                                        rand_p_stray= self.rng.gen::<usize>() % 4;
                                        //println!("rand stray: {}", rand_p_stray);
                                        if battle.player_strays[rand_p_stray].is_some() { //loop until enemy selects a valid target
                                            break;
                                        }
                                    }
                                    loop { //TODO: probly wanna change this cause these nested loops are cringe
                                        rand_move = self.rng.gen::<usize>() % 4;
                                        //println!("rand move: {}", rand_move);
                                        if battle.turn_order[0].moves[rand_move].is_some() { //loop until enemy selects a valid move
                                            break;
                                        }
                                    }

                                    //do random move on random target
                                    rand_int = self.rng.gen();
                                    damage = 0;
                                    if let Some(mv) = &battle.turn_order[0].moves[rand_p_stray] {
                                        if rand_int < (mv.accuracy as f32/100 as f32) {
                                            damage = mv.power;
                                        }
                                        if let Some(p_stray) = &mut battle.player_strays[rand_p_stray] {
                                            p_stray.cur_hp -= damage; //subtract hp from selected stray by the amount of damage the move does
                                            self.menus.open_menu(Textbox::new(&("".to_owned() + &String::from(&battle.turn_order[0].species) + " used " + &mv.name + " on " + &p_stray.species + "!"), font_man).into());
                                            //TODO fix bug where only one enemy turn displays at a time
                                            if p_stray.cur_hp <= 0 {
                                                battle.player_strays[rand_p_stray] = None;
                                            }
                                        }
                                    }
                                    
                                    
                                    if battle.opponent_strays.iter().all(|x| x.is_none()) {
                                        self.menus.close_menu();
                                        self.trans = Transition::Win;
                                        self.transitioning = true;
                                    }
                                    if battle.player_strays.iter().all(|x| x.is_none()) {
                                        self.menus.close_menu();
                                        self.trans = Transition::Loss;
                                        self.transitioning = true;
                                    }    

                                }
                            }
                             
                            self.menus.open_menu(MovesMenu::new(battle.turn_order[0].moves.clone()).into()); //open moves menu
                            
                            //println!("{}", battle.turn_order[0].clone().species);
                        }
                        if battle.opponent_strays.iter().all(|x| x.is_none()) {
                            self.menus.close_menu();
                            self.trans = Transition::Win;
                            self.transitioning = true;
                        }
                        if battle.player_strays.iter().all(|x| x.is_none()) {
                            self.menus.close_menu();
                            self.trans = Transition::Loss;
                            self.transitioning = true;
                        }
                    }
                }
            }
        }
    }

    pub fn check_collision(&self, position: &Vec2) -> bool {
        self.map.check_collision(*position)
            || self
                .collisions
                .contains_key(&position.to_usize(self.map.size_x))
    }

    pub fn try_player_interaction(&mut self, font_man: &FontManager) {
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
            .get_key_value(&temp_pos.to_usize(self.map.size_x));

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
