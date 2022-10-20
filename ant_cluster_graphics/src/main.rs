use agent::AgentStates;
use graphics_engine::{App, EventsBridge};
use object::Object;
use simulation::{Sim, SimConfig};

fn main() {
    // let title = "Ant Cluster";
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
    let config = SimConfig {
        max_iters,
        mapa_height: 100,
        mapa_width: 100,
        qtd_agents: 40,
        agent_vision_radius: radius,
    };
    let mut sim = Sim::create(config);

    fn handle_update(sim: &mut Sim) -> Vec<graphics_engine::Object> {
        let mut objects: Vec<graphics_engine::Object> = vec![];
        sim.update();

        for i in 0..sim.mapa.len() {
            for j in 0..sim.mapa[i].len() {
                if !sim.mapa[i][j].is_empty() {
                    let color = match sim.mapa[i][j].group {
                        1 => [0.6, 0.6, 0.6, 1.0],
                        2 => [0.7, 0.6, 0.8, 1.0],
                        3 => [0.1, 0.9, 0.6, 1.0],
                        4 => [0.9, 0.5, 0.3, 1.0],
                        _ => [1.0; 4],
                    };
                    objects.push(graphics_engine::Object::create(j, i, color));
                }
            }
        }

        for agent in sim.agents.iter() {
            let pos = agent.get_pos();
            let y = pos.i;
            let x = pos.j;
            let color = match agent.get_state() {
                AgentStates::CARRYING => graphics_engine::BLUE,
                AgentStates::SEARCHING => graphics_engine::GREEN,
                AgentStates::FINISHING => graphics_engine::BLUE,
                AgentStates::DONE => graphics_engine::BLACK,
            };
            objects.push(graphics_engine::Object::create(x, y, color));
        }

        return objects;
    }
    let mut app = App::create("Ant Cluster", config.mapa_height, config.mapa_width);

    let mut events = EventsBridge::create();
    while let Some(e) = events.next(&mut app.window_handle) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args, handle_update(&mut sim));
        }
    }
}
