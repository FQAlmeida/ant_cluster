use rand::Rng;
use std::{collections::VecDeque, fmt::Display};

type Vision<T> = Vec<Vec<T>>;

#[derive(PartialEq)]
enum AgentStates {
    CARRYING,
    SEARCHING,
    FINISHING,
    DONE,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    i: usize,
    j: usize,
}

#[derive(Clone, Copy)]
struct AgentConfig {
    vision_radius: usize,
    map_height: usize,
    map_width: usize,
    queue_size: usize,
}

struct Agent<T> {
    pos: Point,
    state: AgentStates,
    backpack: T,
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

trait Object {
    fn is_empty(&self) -> bool;
    fn clone_empty() -> Self;
}

impl<T: Object + Clone> Agent<T> {
    pub fn update_agent(&mut self, mapa: &mut Vision<T>) {
        let mut vision = self.see_map(mapa);
        match self.state {
            AgentStates::CARRYING => self.update_carrying(&mut vision),
            AgentStates::SEARCHING => self.update_searching(&mut vision),
            AgentStates::FINISHING => self.update_finishing(&mut vision),
            AgentStates::DONE => (),
        }
        self.move_agent();
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
    fn see_map(&self, mapa: &Vision<T>) -> Vision<T> {
        let mut vision: Vision<T> = vec![];

        let height = self.config.map_height;
        let width = self.config.map_width;
        let radius = self.config.vision_radius;

        let side = radius * 2 + 1;

        let pos = &self.pos;

        for index_i in 0..side {
            let i = if pos.i + index_i > radius {
                pos.i + index_i - radius - 1
            } else {
                height + pos.i + index_i - radius - 1
            };
            for index_j in 0..side {
                let j = if pos.j + index_j > radius {
                    pos.j + index_j - radius - 1
                } else {
                    width + pos.j + index_j - radius - 1
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
        let pos = &self.pos;
        if !self.should_drop(vision) {
            return;
        }
        vision[pos.i][pos.j] = self.backpack.clone();
        self.backpack = T::clone_empty();
        self.state = AgentStates::SEARCHING;
        // println!("DROPPED");
    }
    fn update_searching(&mut self, vision: &mut Vision<T>) {
        let pos = &self.pos;
        if !self.should_take(vision) {
            return;
        }
        self.backpack = vision[pos.i][pos.j].clone();
        vision[pos.i][pos.j] = T::clone_empty();
        self.state = AgentStates::CARRYING;
    }
    fn update_finishing(&mut self, vision: &mut Vision<T>) {
        let pos = &self.pos;
        if !self.should_take(vision) {
            return;
        }
        self.backpack = vision[pos.i][pos.j].clone();
        vision[pos.i][pos.j] = T::clone_empty();
        self.state = AgentStates::DONE;
    }
    fn count_objs_around(&self, vision: &Vision<T>) -> usize {
        let mut count = 0;

        let radius = self.config.vision_radius;
        let height = self.config.map_height;
        let width = self.config.map_width;

        let pos: &Point = &self.pos;

        let side = radius * 2 + 1;

        for index_i in 0..side {
            let i = if pos.i + index_i > radius {
                pos.i + index_i - radius - 1
            } else {
                height + pos.i + index_i - radius - 1
            };
            for index_j in 0..side {
                let j = if pos.j + index_j > radius {
                    pos.j + index_j - radius - 1
                } else {
                    width + pos.j + index_j - radius - 1
                };
                if vision[i][j].is_empty() && (pos.i != i || pos.j != j) {
                    count += 1;
                }
            }
        }
        return count;
    }
    fn probability(&self, qtd_objs: usize) -> f64 {
        let radius = self.config.vision_radius;
        let side = radius * 2 + 1;
        let qtd_cels = side * side - 1;
        qtd_objs as f64 / qtd_cels as f64
    }
    fn should_take(&self, vision: &mut Vision<T>) -> bool {
        let pos = &self.pos;
        if vision[pos.i][pos.j].is_empty() {
            return false;
        }
        let qtd_objs = self.count_objs_around(vision);
        let prob = self.probability(qtd_objs);

        let mut rng = rand::thread_rng();

        let value = rng.gen_range(0f64..=1f64);

        let decision = value >= prob;
        decision
    }
    
    fn should_drop(&self, vision: &mut Vision<T>) -> bool {
        let pos = &self.pos;
        if vision[pos.i][pos.j].is_empty() {
            return false;
        }

        let qtd_objs = self.count_objs_around(vision);
        let prob = self.probability(qtd_objs);

        let mut rng = rand::thread_rng();

        let value = rng.gen_range(0f64..=1f64);

        let decision = value <= prob;
        decision // || self.rounds_carrying > 10
    }

    pub fn should_finish(&mut self){
        self.state = match self.state {
            AgentStates::CARRYING => AgentStates::FINISHING,
            AgentStates::SEARCHING => AgentStates::DONE,
            AgentStates::FINISHING => AgentStates::FINISHING,
            AgentStates::DONE => AgentStates::DONE,
        };
    }
}
