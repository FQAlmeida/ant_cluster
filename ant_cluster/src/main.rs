use rand::Rng;
use std::{collections::VecDeque, fmt::Display};

const MAPA_HEIGHT: usize = 40;
const MAPA_WIDTH: usize = 40;

const QTD_OBJS: usize = 150;
const QTD_AGENTS: usize = 20;

type CarryValueType = u32;
type MapaDef = Vec<Vec<CarryValueType>>;

fn init_objs() -> MapaDef {
    let mut mapa = vec![vec![0; MAPA_WIDTH]; MAPA_HEIGHT];
    let mut rng = rand::thread_rng();
    let mut qtd_done = 0;
    while qtd_done < QTD_OBJS {
        let i: usize = rng.gen_range(0..MAPA_HEIGHT);
        let j: usize = rng.gen_range(0..MAPA_WIDTH);
        // let value: u32 = rng.gen_range(1u32..=9u32);
        let mapa_pos = mapa[i][j];
        if mapa_pos != 0 {
            continue;
        }
        mapa[i][j] = 1;
        qtd_done += 1;
    }
    return mapa;
}

fn show_mapa(mapa: &MapaDef) {
    let divisor = "-".repeat(MAPA_WIDTH * 4 + 1);
    println!("{}", divisor);
    for row in mapa {
        for cel in row {
            if *cel == 0 {
                print!("|   ");
                continue;
            }
            print!("| {} ", cel);
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
            backpack: 0,
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

    fn count_objs_around(&self, mapa: &MapaDef) -> usize {
        let mut count = 0;
        let pos: &Point = &self.pos;
        let radius = self.radius;
        let side = radius * 2 + 1;
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
                if mapa[i][j] != 0 {
                    count += 1;
                }
            }
        }
        return count;
    }

    fn probability(qtd_objs: usize, radius: usize) -> f64 {
        let len_radius = radius * 2 + 1;
        let qtd_cels = len_radius * len_radius - 1;
        qtd_objs as f64 / qtd_cels as f64
    }

    fn update_searching(&mut self, mapa: &mut MapaDef) {
        let pos = &self.pos;
        if self.should_take(mapa) {
            let old = mapa[pos.i][pos.j];
            self.backpack = mapa[pos.i][pos.j];
            mapa[pos.i][pos.j] = 0;
            assert_eq!(self.backpack, old);
            assert_eq!(mapa[pos.i][pos.j], 0);
            self.state = AgentStates::CARRYING;
            self.rounds_carrying = 1;
            // println!("TOOK");
        }
    }

    fn should_take(&self, mapa: &mut MapaDef) -> bool {
        let pos = &self.pos;
        if mapa[pos.i][pos.j] == 0 {
            return false;
        }
        let qtd_objs = self.count_objs_around(mapa);
        let prob = Agent::probability(qtd_objs, self.radius);

        let mut rng = rand::thread_rng();

        let value = rng.gen_range(0f64..=1f64);

        let decision = value >= prob;
        decision
    }

    fn update_carrying(&mut self, mapa: &mut MapaDef) {
        let pos = &self.pos;
        if !self.should_drop(mapa) {
            self.rounds_carrying += 1;
            return;
        }
        let old = self.backpack;
        mapa[pos.i][pos.j] = self.backpack;
        self.backpack = 0;
        assert_eq!(self.backpack, 0);
        assert_eq!(mapa[pos.i][pos.j], old);
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
        if mapa[pos.i][pos.j] != 0 {
            return false;
        }

        let qtd_objs = self.count_objs_around(mapa);
        let prob = Agent::probability(qtd_objs, self.radius);

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
    for _ in 0..max_iters {
        // if iter % 10000 == 0 {
        //     println!("Iteração: {}", iter);
        // }
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
    let mut iter = 0;
    loop {
        let remaining = agents
            .iter_mut()
            .filter(|agent| agent.state == AgentStates::FINISHING)
            .collect::<Vec<&mut Agent>>();
        if remaining.is_empty() {
            break;
        }
        for agent in remaining {
            agent.update_agent(&mut mapa);
        }
        // if iter % 1000 == 0 {
        //     println!("Iteração Extra: {}", iter);
        // }
        iter += 1;
    }
    println!("Extra Iters {}", iter);
    show_mapa(&mapa);
}
