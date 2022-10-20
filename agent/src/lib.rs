use object::Object;
use rand::Rng;
use std::{collections::VecDeque, fmt::Display};

type Vision<T> = Vec<Vec<T>>;

#[derive(PartialEq, Clone, Copy)]
pub enum AgentStates {
    CARRYING,
    SEARCHING,
    FINISHING,
    DONE,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub i: usize,
    pub j: usize,
}

#[derive(Clone, Copy)]
pub struct AgentConfig {
    vision_radius: usize,
    map_height: usize,
    map_width: usize,
    queue_size: usize,
}

pub struct Agent<T> {
    pos: Point,
    state: AgentStates,
    pub backpack: T,
    history: VecDeque<Point>,
    config: AgentConfig,
}

impl Display for AgentStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentStates::CARRYING => {
                write!(f, "CARRYING")
            }
            AgentStates::SEARCHING => {
                write!(f, "SEARCHING")
            }
            AgentStates::FINISHING => {
                write!(f, "FINISHING")
            }
            AgentStates::DONE => {
                write!(f, "DONE")
            }
        }
    }
}

impl<T: Object + Clone + Copy> Agent<T> {
    pub fn update_agent(&mut self, mapa: &Vision<T>) -> Vision<T> {
        let mut vision = self.see_map(mapa);
        let old_state = self.state;
        match self.state {
            AgentStates::CARRYING => self.update_carrying(&mut vision),
            AgentStates::SEARCHING => self.update_searching(&mut vision),
            AgentStates::FINISHING => self.update_finishing(&mut vision),
            AgentStates::DONE => (),
        }
        let new_state = self.state;
        let pos = self.pos;
        if new_state != old_state {
            match new_state {
                AgentStates::CARRYING => {
                    assert!(!self.backpack.is_empty());
                    assert!(!mapa[pos.i][pos.j].is_empty());
                    assert!(vision[1][1].is_empty());
                }
                AgentStates::SEARCHING => {
                    assert!(self.backpack.is_empty());
                    assert!(mapa[pos.i][pos.j].is_empty());
                    assert!(!vision[1][1].is_empty());
                }
                AgentStates::FINISHING => {
                    // assert!(!self.backpack.is_empty());
                    // assert!(!mapa[pos.i][pos.j].is_empty());
                    // assert!(vision[1][1].is_empty());
                }
                AgentStates::DONE => {
                    assert!(self.backpack.is_empty());
                    assert!(mapa[pos.i][pos.j].is_empty());
                    assert!(!vision[1][1].is_empty());
                }
            }
        }
        // self.update_map(mapa, &vision);
        self.move_agent();
        return vision;
    }

    // fn show_vision(&self, vision: &Vision<T>){
    //     let map_width = self.config.vision_radius * 2 + 1;
    //     let divisor = "-".repeat(map_width * 4 + 1);
    //     println!("{}", divisor);
    //     for row in vision {
    //         for cel in row {
    //             if cel.is_empty() {
    //                 print!("|   ");
    //                 continue;
    //             }
    //             print!("| {} ", 1);
    //         }
    //         print!("|\n");
    //     }
    //     println!("{}", divisor);
    //     println!();
    // }

    pub fn get_pos(&self) -> Point {
        self.pos
    }
    pub fn get_state(&self) -> AgentStates {
        self.state
    }
    pub fn new(pos: Point, config: AgentConfig) -> Agent<T> {
        let mut history: VecDeque<Point> = VecDeque::new();
        history.push_front(pos);
        Agent {
            pos,
            state: AgentStates::SEARCHING,
            backpack: T::clone_empty(),
            history,
            config,
        }
    }

    // fn update_map(&self, mapa: &mut Vision<T>, vision: &Vision<T>) {
    //     let pos = self.get_pos();
    //     let radius = self.config.vision_radius;
    //     let height = self.config.map_height;
    //     let width = self.config.map_width;
    //     for i in 0..vision.len() {
    //         let real_i = if pos.i + i >= radius {
    //             (pos.i + i - radius) % height
    //         } else {
    //             height + pos.i + i - radius
    //         };
    //         for j in 0..vision[i].len() {
    //             let real_j = if pos.j + j >= radius {
    //                 (pos.j + j - radius) % width
    //             } else {
    //                 width + pos.j + j - radius
    //             };

    //             mapa[real_i][real_j] = vision[i][j].clone();
    //         }
    //     }
    // }

    fn create_vision(&self) -> Vision<T> {
        let mut vision = vec![];
        let radius = self.config.vision_radius;
        let vision_side = radius * 2 + 1;
        for i in 0..vision_side {
            vision.push(vec![]);
            for _ in 0..vision_side {
                vision[i].push(T::clone_empty());
            }
        }
        vision
    }
    fn see_map(&self, mapa: &Vision<T>) -> Vision<T> {
        let mut vision: Vision<T> = self.create_vision();

        let height = self.config.map_height;
        let width = self.config.map_width;
        let radius = self.config.vision_radius;

        let side = radius * 2 + 1;

        let pos = &self.pos;

        for index_i in 0..side {
            let i = if pos.i + index_i >= radius {
                (pos.i + index_i - radius) % height
            } else {
                height + pos.i - radius + index_i
            };
            for index_j in 0..side {
                let j = if pos.j + index_j >= radius {
                    (pos.j + index_j - radius) % width
                } else {
                    width + pos.j + index_j - radius
                };
                vision[index_i][index_j] = mapa[i][j].clone();
            }
        }

        return vision;
    }
    fn move_agent(&mut self) {
        let height = self.config.map_height;
        let width = self.config.map_width;
        let queue_size = self.config.queue_size;

        let mut rng = rand::thread_rng();

        let old_pos = &self.pos;

        let mut new_pos: Point;
        let mut tries: Vec<Point> = vec![];

        loop {
            let i: usize = rng.gen_range(0..=2);
            let j: usize = rng.gen_range(0..=2);

            let mut new_i: usize = old_pos.i + i;
            if new_i > 0 {
                new_i -= 1;
            } else if new_i == 0 {
                new_i = height - 1;
            }
            if new_i >= height {
                new_i = new_i - height;
            }

            let mut new_j: usize = old_pos.j + j;
            if new_j > 0 {
                new_j -= 1;
            } else if new_j == 0 {
                new_j = width - 1;
            }
            if new_j >= width {
                new_j = new_j - width;
            }
            new_pos = Point { i: new_i, j: new_j };

            let tries_history = tries
                .iter()
                .find(|&point| point.i == new_pos.i && point.j == new_pos.j);
            if !tries_history.is_none() {
                continue;
            }
            tries.push(Point {
                i: new_pos.i,
                j: new_pos.j,
            });

            let history_result = self
                .history
                .iter()
                .find(|&pos| pos.i == new_pos.i && pos.j == new_pos.j);

            if history_result.is_none() || tries.len() == queue_size {
                break;
            }
        }
        self.history.push_back(Point {
            i: new_pos.i,
            j: new_pos.j,
        });

        if self.history.len() >= queue_size {
            self.history.pop_front();
        }
        assert_ne!(self.pos, new_pos);
        assert!(new_pos.i < height);
        assert!(new_pos.j < width);
        self.pos = new_pos;
    }

    fn update_carrying(&mut self, vision: &mut Vision<T>) {
        let pos = self.get_self_pos();
        if !self.should_drop(vision) {
            return;
        }
        vision[pos.i][pos.j] = self.backpack.clone();
        self.backpack = T::clone_empty();
        self.state = AgentStates::SEARCHING;
        // println!("DROPPED");
    }
    fn update_searching(&mut self, vision: &mut Vision<T>) {
        let pos = self.get_self_pos();
        if !self.should_take(vision) {
            return;
        }
        self.backpack = vision[pos.i][pos.j].clone();
        vision[pos.i][pos.j] = T::clone_empty();
        self.state = AgentStates::CARRYING;
        // println!("CARRING");
    }
    fn update_finishing(&mut self, vision: &mut Vision<T>) {
        let pos = self.get_self_pos();
        if !self.should_take(vision) {
            return;
        }
        self.backpack = vision[pos.i][pos.j].clone();
        vision[pos.i][pos.j] = T::clone_empty();
        self.state = AgentStates::DONE;
    }
    // fn count_objs_around(&self, vision: &Vision<T>) -> usize {
    //     let mut count = 0;

    //     let radius = self.config.vision_radius;

    //     let pos = self.get_self_pos();

    //     let side = radius * 2 + 1;

    //     for index_i in 0..side {
    //         for index_j in 0..side {
    //             if !vision[index_i][index_j].is_empty() && (pos.i != index_i || pos.j != index_j) {
    //                 count += 1;
    //             }
    //         }
    //     }
    //     return count;
    // }

    // fn probability(&self, qtd_objs: usize) -> f64 {
    //     let radius = self.config.vision_radius;
    //     let side = radius * 2 + 1;
    //     let qtd_cels = side * side - 1;
    //     qtd_objs as f64 / qtd_cels as f64
    // }
    fn get_self_pos(&self) -> Point {
        let radius = self.config.vision_radius;
        Point {
            i: radius,
            j: radius,
        }
    }
    fn get_density(&self, vision: &Vision<T>) -> f64 {
        let mut density = 0.0;
        let pos: &Point = &self.pos;
        let radius = self.config.vision_radius;
        let side = radius * 2 + 1;
        let mut area = 0.0;
        let alpha = 6.0;
        for i in 0..side {
            for j in 0..side {
                if !vision[i][j].is_empty() && (i != pos.i || j != pos.j) {
                    // let dist = Agent::get_distance(&self.backpack, &vision[i][j]);
                    let dist = self.backpack.get_distance(&vision[i][j]);
                    let dissim = 1.0 - (dist / alpha);
                    if dissim >= 0.0 {
                        density += dissim;
                    }
                    area += 1.0;
                }
            }
        }
        if density <= 0.0 {
            return 0.0;
        }
        let f = density / (area * area);
        assert!(f <= 1.0);
        f
    }

    fn should_take(&self, vision: &mut Vision<T>) -> bool {
        let pos = self.get_self_pos();
        if vision[pos.i][pos.j].is_empty() {
            return false;
        }
        // assert_ne!(vision[pos.i][pos.j].x, 0.0);
        // assert_ne!(vision[pos.i][pos.j].y, 0.0);
        // assert_ne!(vision[pos.i][pos.j].group, 0);
        let k1 = 0.25;
        let density = self.get_density(vision);
        let coeff = k1 / (k1 + density);
        let prob = coeff * coeff;

        let mut rng = rand::thread_rng();

        let value = rng.gen_range(0f64..=1f64);

        let decision = value <= prob;
        decision
    }

    fn should_drop(&self, vision: &mut Vision<T>) -> bool {
        let pos = self.get_self_pos();
        if !vision[pos.i][pos.j].is_empty() {
            return false;
        }

        let k2 = 0.2;
        let density = self.get_density(vision);
        // if density != 0.0 {
        //     println!("{}", density);
        // }
        let coeff = density / (k2 + density);
        let prob = coeff * coeff;

        let mut rng = rand::thread_rng();
        let value = rng.gen_range(0f64..=1f64);

        let decision = value <= prob;
        decision
    }

    pub fn finish(&mut self) {
        self.state = match self.state {
            AgentStates::CARRYING => AgentStates::FINISHING,
            AgentStates::SEARCHING => AgentStates::DONE,
            AgentStates::FINISHING => AgentStates::FINISHING,
            AgentStates::DONE => AgentStates::DONE,
        };
        // println!("{}", self.state);
    }

    pub fn is_finishing(&self) -> bool {
        self.state == AgentStates::FINISHING
    }

    pub fn create_agents(
        radius: usize,
        qtd: usize,
        map_height: usize,
        map_width: usize,
    ) -> Vec<Agent<T>> {
        let mut agents: Vec<Agent<T>> = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..qtd {
            let pos = Point {
                i: rng.gen_range(0..map_height),
                j: rng.gen_range(0..map_width),
            };
            let config = AgentConfig {
                vision_radius: radius,
                map_height,
                map_width,
                queue_size: 8,
            };
            agents.push(Agent::new(pos, config));
        }
        return agents;
    }
}
