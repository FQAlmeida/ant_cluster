use ant_cluster_data::{create_agents, init_objs, show_mapa, AgentStates, Agent};

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
