use rand::Rng;
use std::{
    collections::VecDeque,
    fmt::Display,
    sync::{Arc, RwLock},
    thread, vec,
};

const MAPA_HEIGHT: usize = 25;
const MAPA_WIDTH: usize = 25;

const QTD_OBJS: usize = 200;
const QTD_AGENTS: usize = 10;

type CarryValueType = RwLock<u32>;
type MapaDef = Vec<Vec<CarryValueType>>;

fn init_objs() -> MapaDef {
    let mut mapa: MapaDef = vec![];
    for i in 0..MAPA_HEIGHT {
        mapa.push(vec![]);
        for _ in 0..MAPA_WIDTH {
            mapa[i].push(RwLock::new(0));
        }
    }
    let mut rng = rand::thread_rng();
    let mut qtd_done = 0;
    while qtd_done < QTD_OBJS {
        let i: usize = rng.gen_range(0usize..MAPA_HEIGHT);
        let j: usize = rng.gen_range(0usize..MAPA_WIDTH);
        let value: u32 = rng.gen_range(1u32..=9u32);
        {
            let mapa_pos = mapa[i][j].read().unwrap();
            if *mapa_pos != 0 {
                continue;
            }
        }
        {
            let mut mapa_pos = mapa[i][j].write().unwrap();
            *mapa_pos = value;
        }
        qtd_done += 1;
    }
    return mapa;
}

fn show_mapa(mapa: &MapaDef) {
    let divisor = "-".repeat(MAPA_WIDTH * 4 + 1);
    println!("{}", divisor);
    for row in mapa.iter() {
        for cel in row {
            let value = *cel.read().unwrap();
            print!("| {} ", value);
        }
        print!("|\n");
    }
    println!("{}", divisor);
    println!();
}

enum AgentStates {
    CARRYING,
    SEARCHING,
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
        }
    }
}

struct Point {
    x: usize,
    y: usize,
}

struct Agent {
    pos: Point,
    state: AgentStates,
    backpack: u32,
    history: VecDeque<Point>,
    rounds_carrying: usize,
}

impl Agent {
    pub fn update_agent(&mut self, mapa: &MapaDef) {
        match self.state {
            AgentStates::CARRYING => self.update_carrying(mapa),
            AgentStates::SEARCHING => self.update_searching(mapa),
        }
        self.move_agent();
        // println!(
        //     "New Agent state {} {} {} {}",
        //     self.pos.x, self.pos.y, self.backpack, self.state
        // );
    }
    pub fn new(initial_x: usize, initial_y: usize) -> Agent {
        Agent {
            pos: Point {
                x: initial_x,
                y: initial_y,
            },
            state: AgentStates::SEARCHING,
            backpack: 0,
            history: VecDeque::new(),
            rounds_carrying: 0,
        }
    }

    fn move_agent(&mut self) {
        const MAX_QUEUE_SIZE: usize = 8;
        let mut rng = rand::thread_rng();
        let old_pos = &self.pos;
        let mut new_pos: Point = Point {
            x: old_pos.x,
            y: old_pos.y,
        };
        for _ in 0..MAX_QUEUE_SIZE {
            let x: usize = rng.gen_range(0..=2);
            let y: usize = rng.gen_range(0..=2);
            let mut new_x: usize = old_pos.x + x;
            if new_x > 0 {
                new_x -= 1;
            }
            if new_x >= MAPA_WIDTH {
                new_x = MAPA_WIDTH - 1;
            }
            let mut new_y: usize = old_pos.y + y;
            if new_y > 0 {
                new_y -= 1;
            }
            if new_y >= MAPA_HEIGHT {
                new_y = MAPA_HEIGHT - 1;
            }
            new_pos = Point { x: new_x, y: new_y };
            let history_result = self
                .history
                .iter()
                .find(|&pos| pos.x == new_pos.x && pos.y == new_pos.y);
            if history_result.is_none() {
                break;
            }
        }
        self.history.push_back(Point {
            x: new_pos.x,
            y: new_pos.y,
        });

        if self.history.len() >= MAX_QUEUE_SIZE {
            self.history.pop_front();
        }
        self.pos.y = new_pos.y;
        self.pos.x = new_pos.x;
    }

    fn count_objs_around(&self, mapa: &MapaDef, radius: usize) -> usize {
        let mut count = 0;
        let pos: &Point = &self.pos;
        for i in 0..(radius * 2 + 1) {
            if i + pos.x < radius || i + pos.x - radius >= MAPA_HEIGHT || i == 0 {
                continue;
            }
            for j in 0..(radius * 2 + 1) {
                if j + pos.y < radius || j + pos.y - radius >= MAPA_WIDTH || j == 0 {
                    continue;
                }
                {
                    let mapa_pos = mapa[pos.x + i - radius][pos.y + j - radius].read().unwrap();
                    if *mapa_pos != 0 {
                        count += 1;
                    }
                }
            }
        }
        return count;
    }

    fn probability(&self, qtd_objs: usize, radius: usize) -> f64 {
        let len_radius = radius * 2 + 1;
        let qtd_cels = len_radius * len_radius - 1;
        qtd_objs as f64 / qtd_cels as f64
    }

    fn update_searching(&mut self, mapa: &MapaDef) {
        let pos = &self.pos;
        if self.should_take(mapa) {
            {
                let mut mapa_pos = mapa[pos.x][pos.y].write().unwrap();
                self.backpack = *mapa_pos;
                *mapa_pos = 0;
            }
            self.state = AgentStates::CARRYING;
            self.rounds_carrying = 1;
            // println!("TOOK");
        }
    }

    fn should_take(&self, mapa: &MapaDef) -> bool {
        let pos = &self.pos;
        {
            let mapa_pos = mapa[pos.x][pos.y].read().unwrap();
            if *mapa_pos == 0 {
                return false;
            }
        }
        let qtd_objs = self.count_objs_around(mapa, 1);
        let prob = self.probability(qtd_objs, 1);

        let mut rng = rand::thread_rng();

        let value = rng.gen_range(0f64..=1f64);

        let decision = value >= prob;
        decision
    }

    fn update_carrying(&mut self, mapa: &MapaDef) {
        let pos = &self.pos;
        if !self.should_drop(mapa) {
            self.rounds_carrying += 1;
            return;
        }
        let prev = self.backpack;
        {
            let mut mapa_pos = mapa[pos.x][pos.y].write().unwrap();
            *mapa_pos = self.backpack;
        }
        self.backpack = 0;
        {
            let mapa_pos = mapa[pos.x][pos.y].read().unwrap();
            assert_ne!(prev, 0);
            assert_eq!(*mapa_pos, prev);
            assert_ne!(*mapa_pos, 0);
        }
        self.state = AgentStates::SEARCHING;
        self.rounds_carrying = 0;
        // println!("DROPPED");
    }

    fn should_drop(&self, mapa: &MapaDef) -> bool {
        let pos = &self.pos;
        {
            let mapa_pos = mapa[pos.x][pos.y].read().unwrap();
            if *mapa_pos != 0 {
                return false;
            }
        }

        let qtd_objs = self.count_objs_around(mapa, 1);
        let prob = self.probability(qtd_objs, 1);

        let mut rng = rand::thread_rng();

        let value = rng.gen_range(0f64..=1f64);

        let decision = value <= prob;
        decision // || self.rounds_carrying > 10
    }
}

fn worker(worker_id: usize, mapa: &MapaDef, qtd_iters: u32) {
    let mut rng = rand::thread_rng();
    let pos = Point {
        x: rng.gen_range(0..MAPA_WIDTH),
        y: rng.gen_range(0..MAPA_HEIGHT),
    };
    let mut agent = Agent::new(pos.x, pos.y);
    println!("Worker {} Started", worker_id);
    for _ in 1..=qtd_iters {
        // println!("Worker {} Updating Iter {}", worker_id, iter);
        agent.update_agent(mapa);
        // println!("Worker {} Updated", worker_id);
    }
    println!("Worker {} Finished", worker_id);
}

fn main() {
    let mapa = Arc::new(init_objs());
    show_mapa(&mapa);
    // let mut agents = create_agents();
    const MAX_ITERS: u32 = 20000u32;
    let mut handles = vec![];
    for id in 1..=QTD_AGENTS {
        let mapa = Arc::clone(&mapa);
        let handle = thread::spawn(move || {
            worker(id, &mapa, MAX_ITERS);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    show_mapa(&mapa);
}
