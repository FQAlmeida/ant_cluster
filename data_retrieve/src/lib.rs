use std::fs;

pub const DATA_1_FP: &str = "data/data_1.txt";

#[derive(Copy, Clone)]
pub struct Data {
    pub x: f64,
    pub y: f64,
    pub group: u8,
}

impl Data {
    pub fn new_empty_data() -> Data {
        Data {
            x: 0.0,
            y: 0.0,
            group: 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.group == 0
    }
}

pub fn get_data(fp: &str) -> Vec<Data> {
    let mut data = vec![];
    let fd = fs::read_to_string(fp).expect("To be able to open the file");
    let lines = fd.lines();
    for line in lines {
        let items = line.split_whitespace().collect::<Vec<&str>>();
        // println!("{}", line.to_string());
        let x: f64 = items[0].parse().unwrap();
        let y: f64 = items[1].parse().unwrap();
        let group: u8 = items[2].parse().unwrap();
        // println!("|{}\t||{}\t||{}|", x, y, group);
        data.push(Data { x, y, group });
    }
    return data;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = get_data("../data/data_1.txt");
        assert!(data.len() == 400);
        for d in data{
            assert_ne!(d.x, 0.0);
            assert_ne!(d.y, 0.0);
            assert_ne!(d.group, 0);
        }
    }
}
