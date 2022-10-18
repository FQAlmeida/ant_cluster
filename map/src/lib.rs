use data_retrieve::{get_data, Data, DATA_1_FP};
use rand::Rng;
use object::Object;

pub type CarryValueType = Data;
pub type MapaDef = Vec<Vec<CarryValueType>>;

fn init_map(mapa_height: usize, mapa_width: usize) -> MapaDef {
    let mut mapa = vec![];
    for i in 0..mapa_height {
        mapa.push(vec![]);
        for _ in 0..mapa_width {
            mapa[i].push(Data::clone_empty());
        }
    }
    return mapa;
}

pub fn init_objs(mapa_height: usize, mapa_width: usize) -> MapaDef {
    let mut mapa = init_map(mapa_height, mapa_width);
    let mut rng = rand::thread_rng();
    let mut qtd_done = 0;
    let data = get_data(DATA_1_FP);
    assert_eq!(data.len(), 400);
    while qtd_done < data.len() {
        let i: usize = rng.gen_range(0..mapa_height);
        let j: usize = rng.gen_range(0..mapa_width);
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
