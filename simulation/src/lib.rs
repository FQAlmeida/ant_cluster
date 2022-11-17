use agent::Agent;
use data_retrieve::Data;
use map::{init_objs, MapaDef};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SimState {
    RUNNING,
    FINISHING,
    DONE,
}

#[derive(Clone, Copy)]
pub struct SimConfig {
    pub max_iters: usize,
    pub mapa_height: usize,
    pub mapa_width: usize,
    pub qtd_agents: usize,
    pub agent_vision_radius: usize,
}

pub struct Sim {
    pub mapa: MapaDef,
    pub agents: Vec<Agent<Data>>,
    state: SimState,
    iter_atual: usize,
    pub extra_iters: usize,
    pub config: SimConfig,
}

impl Sim {
    pub fn create(config: SimConfig) -> Self {
        let mapa = init_objs(config.mapa_height, config.mapa_width);
        // show_mapa(&mapa, mapa_width);
        let agents = Agent::create_agents(
            config.agent_vision_radius,
            config.qtd_agents,
            config.mapa_height,
            config.mapa_width,
        );
        Self {
            mapa,
            agents,
            state: SimState::RUNNING,
            iter_atual: 0,
            extra_iters: 0,
            config,
        }
    }

    fn update_done(&self) {
        return;
    }

    fn update_running(&mut self) {
        let agents = self.agents.iter_mut();
        let mapa = self.mapa.as_mut();
        for agent in agents {
            let old_state = agent.get_state();
            let pos = agent.get_pos();
            let vision = agent.update_agent(mapa);
            let new_state = agent.get_state();
            if new_state != old_state {
                match new_state {
                    agent::AgentStates::CARRYING => {
                        assert_ne!(agent.backpack.group, 0);
                        assert_ne!(mapa[pos.i][pos.j].group, 0);
                        assert_eq!(vision[1][1].group, 0);
                    }
                    agent::AgentStates::SEARCHING => {
                        assert_eq!(agent.backpack.group, 0);
                        assert_eq!(mapa[pos.i][pos.j].group, 0);
                        assert_ne!(vision[1][1].group, 0);
                    }
                    agent::AgentStates::FINISHING => {
                        assert_ne!(agent.backpack.group, 0);
                        assert_ne!(mapa[pos.i][pos.j].group, 0);
                        assert_eq!(vision[1][1].group, 0);
                    }
                    agent::AgentStates::DONE => {
                        assert_eq!(agent.backpack.group, 0);
                        assert_eq!(mapa[pos.i][pos.j].group, 0);
                        assert_ne!(vision[1][1].group, 0);
                    }
                }
            }
            mapa[pos.i][pos.j] = vision[1][1].clone();
            // for i in 0..vision.len() {
            //     let real_i = if pos.i + i >= 1 {
            //         (pos.i + i - 1) % 90
            //     } else {
            //         90 + pos.i + i - 1
            //     };
            //     for j in 0..vision[i].len() {
            //         let real_j = if pos.j + j >= 1 {
            //             (pos.j + j - 1) % 50
            //         } else {
            //             50 + pos.j + j - 1
            //         };
            //         show_mapa(mapa, self.config.mapa_width);
            //         println!("{} {} {} {}", i, j, real_i, real_j);
            //         assert_eq!(mapa[real_i][real_j].x, vision[i][j].x);
            //         assert_eq!(mapa[real_i][real_j].y, vision[i][j].y);
            //         assert_eq!(mapa[real_i][real_j].group, vision[i][j].group);
            //     }
            // }
        }
        self.iter_atual += 1;
        if self.iter_atual == self.config.max_iters {
            self.state = SimState::FINISHING;
            for agent in self.agents.iter_mut() {
                agent.finish();
            }
        }
    }

    fn update_finishing(&mut self) {
        let mut remaining = self
            .agents
            .iter_mut()
            .filter(|agent| agent.is_finishing())
            .collect::<Vec<&mut Agent<Data>>>();
        if remaining.is_empty() {
            self.state = SimState::DONE;
            return;
        }
        for agent in remaining.iter_mut() {
            let pos = agent.get_pos();
            let vision = agent.update_agent(&mut self.mapa);
            self.mapa[pos.i][pos.j] = vision[1][1];
        }
        self.extra_iters += 1;
    }

    pub fn get_state(&self) -> SimState{
        self.state
    }

    pub fn update(&mut self) {
        match self.state {
            SimState::RUNNING => self.update_running(),
            SimState::FINISHING => self.update_finishing(),
            SimState::DONE => self.update_done(),
        }
        if self.iter_atual % 10000 == 0 {
            println!("{} {}", self.iter_atual, self.extra_iters);
        }
    }
}
