use ant_cluster_data::{
    create_agents, init_objs, Agent, AgentStates, MapaDef, MAPA_HEIGHT, MAPA_WIDTH,
};
use graphics_engine::App;
use simulation::Sim;

fn main() {
    let title = "Ant Cluster";
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
    let mut sim = Sim::create(90, 50);

    fn handle_update(sim: &Sim ) -> Vec<graphics_engine::Object> {
        let mut objects: Vec<graphics_engine::Object> = vec![];
        sim.update();
        for agent in sim.agents.iter() {
            let pos = agent.get_pos();
            let y = pos.i;
            let x = pos.j;
            objects.push(graphics_engine::Object::create(x, y, graphics_engine::GREEN));
        }
        return objects;
    }
    let handler = Box::new(||{return handle_update(&mut sim)});
    let mut app = App::create("Ant Cluster", 90, 50, handler);

    app.run();
}
