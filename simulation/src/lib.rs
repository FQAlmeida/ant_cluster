use agent::Agent;
use data_retrieve::Data;
use map::{init_objs, MapaDef};

enum SimState {
    RUNNING,
    FINISHING,
    DONE,
}

pub struct Sim {
    pub mapa: MapaDef,
    pub agents: Vec<Agent<Data>>,
    state: SimState,
    iter_atual: usize,
    max_iters: usize,
    pub extra_iters: usize,
}

impl Sim {
    pub fn create(mapa_height: usize, mapa_width: usize) -> Self {
        let mapa = init_objs(mapa_height, mapa_width);
        let agents = Agent::create_agents(1, 20, mapa_height, mapa_width);
        Self {
            mapa,
            agents,
            state: SimState::RUNNING,
            iter_atual: 0,
            max_iters: 100000,
            extra_iters: 0,
        }
    }

    fn update_done(&self) {
        return;
    }

    fn update_running(&mut self) {
        for agent in self.agents.iter_mut() {
            agent.update_agent(&mut self.mapa);
        }
        self.iter_atual += 1;
        if self.iter_atual == self.max_iters {
            self.state = SimState::FINISHING;
            for agent in self.agents.iter_mut() {
                agent.set_finishing();
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
            agent.update_agent(&mut self.mapa);
        }
        self.extra_iters += 1;
    }

    pub fn update(&mut self) {
        match self.state {
            SimState::RUNNING => self.update_running(),
            SimState::FINISHING => self.update_finishing(),
            SimState::DONE => self.update_done(),
        }
    }
}
