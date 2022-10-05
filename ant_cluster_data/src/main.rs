use rand::Rng;
use std::{collections::VecDeque, fmt::Display};

use data_retrieve::{get_data, Data, DATA_1_FP};

const MAPA_HEIGHT: usize = 70;
const MAPA_WIDTH: usize = 45;

const QTD_AGENTS: usize = 20;

type CarryValueType = Data;
type MapaDef = Vec<Vec<CarryValueType>>;

fn init_map() -> MapaDef {
    let mut mapa = vec![];
    for i in 0..MAPA_HEIGHT {
        mapa.push(vec![]);
        for _ in 0..MAPA_WIDTH {
            mapa[i].push(Data::new_empty_data());
        }
    }
    return mapa;
}

fn init_objs() -> MapaDef {
    let mut mapa = init_map();
    let mut rng = rand::thread_rng();
    let mut qtd_done = 0;
    let data = get_data(DATA_1_FP);
    assert_eq!(data.len(), 400);
    while qtd_done < data.len() {
        let i: usize = rng.gen_range(0..MAPA_HEIGHT);
        let j: usize = rng.gen_range(0..MAPA_WIDTH);
        // let value: u32 = rng.gen_range(1u32..=9u32);
        let mapa_pos = mapa[i][j];
        if !mapa_pos.is_empty() {
            continue;
        }
        mapa[i][j].x = data[qtd_done].x;
        mapa[i][j].y = data[qtd_done].y;
        mapa[i][j].group = data[qtd_done].group;
        qtd_done += 1;
    }
    return mapa;
}

fn show_mapa(mapa: &MapaDef) {
    let divisor = "-".repeat(MAPA_WIDTH * 4 + 1);
    println!("{}", divisor);
    for row in mapa {
        for cel in row {
            if cel.is_empty() {
                print!("|   ");
                continue;
            }
            print!("| {} ", cel.group);
        }
        print!("|\n");
    }
    println!("{}", divisor);
    println!();
}

#[derive(PartialEq)]
enum AgentStates {
    CARRYING,
    SEARCHING,
    FINISHING,
    DONE,
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

#[derive(Debug, PartialEq)]
struct Point {
    i: usize,
    j: usize,
}

struct Agent {
    pos: Point,
    state: AgentStates,
    backpack: CarryValueType,
    history: VecDeque<Point>,
    rounds_carrying: usize,
    radius: usize,
}

impl Agent {
    pub fn update_agent(&mut self, mapa: &mut MapaDef) {
        match self.state {
            AgentStates::CARRYING => self.update_carrying(mapa),
            AgentStates::SEARCHING => self.update_searching(mapa),
            AgentStates::FINISHING => self.update_carrying(mapa),
            AgentStates::DONE => (),
        }
        self.move_agent();
        // println!(
        //     "New Agent state {} {} {} {}",
        //     self.pos.i, self.pos.j, self.backpack, self.state
        // );
    }
    pub fn new(initial_i: usize, initial_j: usize, radius: usize) -> Agent {
        let mut history: VecDeque<Point> = VecDeque::new();
        let point = Point {
            i: initial_i,
            j: initial_j,
        };

        history.push_back(Point {
            i: point.i,
            j: point.j,
        });

        Agent {
            pos: point,
            state: AgentStates::SEARCHING,
            backpack: Data::new_empty_data(),
            history,
            rounds_carrying: 0,
            radius,
        }
    }

    fn move_agent(&mut self) {
        const MAX_QUEUE_SIZE: usize = 8;
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
                new_i = MAPA_HEIGHT - 1;
            }
            if new_i >= MAPA_HEIGHT {
                new_i = new_i - MAPA_HEIGHT;
            }

            let mut new_j: usize = old_pos.j + j;
            if new_j > 0 {
                new_j -= 1;
            } else if new_j == 0 {
                new_j = MAPA_WIDTH - 1;
            }
            if new_j >= MAPA_WIDTH {
                new_j = new_j - MAPA_WIDTH;
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

            if history_result.is_none() || tries.len() == MAX_QUEUE_SIZE {
                // println!("History");
                // for point in self.history.iter() {
                //     print!("({} {}) ", point.i, point.j);
                // }
                // print!("\n");
                // println!("Tries");
                // for point in tries.iter() {
                //     print!("({} {}) ", point.i, point.j);
                // }
                // print!("\n");
                break;
            }
        }
        self.history.push_back(Point {
            i: new_pos.i,
            j: new_pos.j,
        });

        if self.history.len() >= MAX_QUEUE_SIZE {
            self.history.pop_front();
        }
        assert_ne!(self.pos, new_pos);
        assert!(new_pos.i < MAPA_HEIGHT);
        assert!(new_pos.j < MAPA_WIDTH);
        // let mut x_diff = new_pos.i as i64 - self.pos.i as i64;
        // if x_diff >= 19 {
        //     x_diff = -1;
        // }else if x_diff <= -19{
        //     x_diff = 1;
        // }
        // let mut y_diff = new_pos.j as i64 - self.pos.j as i64;
        // if y_diff >= 19 {
        //     y_diff = -1;
        // }else if y_diff <= -19{
        //     y_diff = 1;
        // }
        // println!("{} {}", x_diff, y_diff);
        self.pos = new_pos;
    }

    fn get_distance(data_1: &Data, data_2: &Data) -> f64 {
        let diff_x = data_1.x - data_2.x;
        let diff_y = data_1.y - data_2.y;
        let square = (diff_x * diff_x) + (diff_y * diff_y);
        square.sqrt()
    }

    fn get_density(&self, mapa: &MapaDef) -> f64 {
        let mut density = 0.0;
        let pos: &Point = &self.pos;
        let radius = self.radius;
        let side = radius * 2 + 1;
        let mut area = 0.0;
        let alpha = 2.0;
        for index_i in 0..side {
            let i = if pos.i + index_i >= radius {
                (pos.i + index_i - radius) % MAPA_HEIGHT
            } else {
                MAPA_HEIGHT + pos.i - radius + index_i
            };
            for index_j in 0..side {
                let j = if pos.j + index_j >= radius {
                    (pos.j + index_j - radius) % MAPA_WIDTH
                } else {
                    MAPA_WIDTH + pos.j + index_j - radius
                };
                if !mapa[i][j].is_empty() && (i != pos.i || j != pos.j) {
                    let dist = Agent::get_distance(&self.backpack, &mapa[i][j]);
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

    fn update_searching(&mut self, mapa: &mut MapaDef) {
        let pos = &self.pos;
        if !self.should_take(mapa) {
            return;
        }
        self.backpack.x = mapa[pos.i][pos.j].x;
        self.backpack.y = mapa[pos.i][pos.j].y;
        self.backpack.group = mapa[pos.i][pos.j].group;
        let empty = Data::new_empty_data();
        mapa[pos.i][pos.j].x = empty.x;
        mapa[pos.i][pos.j].y = empty.y;
        mapa[pos.i][pos.j].group = empty.group;
        self.state = AgentStates::CARRYING;
        self.rounds_carrying = 0;
        // println!("TOOK");
    }

    fn should_take(&self, mapa: &mut MapaDef) -> bool {
        let pos = &self.pos;
        if mapa[pos.i][pos.j].is_empty() {
            return false;
        }
        assert_ne!(mapa[pos.i][pos.j].x, 0.0);
        assert_ne!(mapa[pos.i][pos.j].y, 0.0);
        assert_ne!(mapa[pos.i][pos.j].group, 0);
        let k1 = 0.01;
        let density = self.get_density(mapa);
        let coeff = k1 / (k1 + density);
        let prob = coeff * coeff;

        let mut rng = rand::thread_rng();

        let value = rng.gen_range(0f64..=1f64);

        let decision = value <= prob;
        decision
    }

    fn update_carrying(&mut self, mapa: &mut MapaDef) {
        let pos = &self.pos;
        if !self.should_drop(mapa) {
            self.rounds_carrying += 1;
            return;
        }
        mapa[pos.i][pos.j].x = self.backpack.x;
        mapa[pos.i][pos.j].y = self.backpack.y;
        mapa[pos.i][pos.j].group = self.backpack.group;
        let empty = Data::new_empty_data();
        self.backpack.x = empty.x;
        self.backpack.y = empty.y;
        self.backpack.group = empty.group;
        self.state = if self.state == AgentStates::CARRYING {
            AgentStates::SEARCHING
        } else {
            AgentStates::DONE
        };
        self.rounds_carrying = 0;
        // println!("DROPPED");
    }

    fn should_drop(&self, mapa: &mut MapaDef) -> bool {
        let pos = &self.pos;
        if !mapa[pos.i][pos.j].is_empty() {
            return false;
        }

        let k2 = 0.6;
        let density = self.get_density(mapa);
        let coeff = density / (k2 + density);
        let prob = coeff * coeff;

        let mut rng = rand::thread_rng();
        let value = rng.gen_range(0f64..=1f64);

        let decision = value <= prob;
        decision // || self.rounds_carrying > 10
    }
}

// fn agent_worker(initial_x: usize, initial_y: usize) {}

fn create_agents(radius: usize) -> Vec<Agent> {
    let mut agents: Vec<Agent> = vec![];
    let mut rng = rand::thread_rng();
    for _ in 0..QTD_AGENTS {
        let pos = Point {
            i: rng.gen_range(0..MAPA_HEIGHT),
            j: rng.gen_range(0..MAPA_WIDTH),
        };
        agents.push(Agent::new(pos.i, pos.j, radius));
    }
    return agents;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let radius: usize = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(1)
    } else {
        1
    };
    let max_iters: usize = if args.len() > 2 {
        args[2].parse::<usize>().unwrap_or(100_000)
    } else {
        100_000
    };
    println!("Config Radius {} Iters {}", radius, max_iters);
    let mut mapa = init_objs();
    let mut agents = create_agents(radius);
    show_mapa(&mapa);
    for iter in 0..max_iters {
        if iter % 100000 == 0 {
            println!("Iteração: {}", iter);
            show_mapa(&mapa);
        }
        for agent in agents.iter_mut() {
            agent.update_agent(&mut mapa);
        }
    }
    for agent in agents.iter_mut() {
        agent.state = if agent.state == AgentStates::SEARCHING {
            AgentStates::DONE
        } else {
            AgentStates::FINISHING
        };
    }
    show_mapa(&mapa);
    let mut iter = 0;
    loop {
        let mut remaining = agents
            .iter_mut()
            .filter(|agent| agent.state == AgentStates::FINISHING)
            .collect::<Vec<&mut Agent>>();
        if remaining.is_empty() {
            break;
        }
        for agent in remaining.iter_mut() {
            agent.update_agent(&mut mapa);
        }
        if iter % 10000 == 0 {
            println!("Iteração Extra: {}", iter);
            println!("Remaining {}", remaining.len());
        }
        iter += 1;
    }
    println!("Extra Iters {}", iter);
    show_mapa(&mapa);
}
