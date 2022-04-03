use std::collections::HashMap;

fn main() {
    let triangles = get_triangles();
    let mut map_vec: Vec<HashMap<u32, Vec<u32>>> = vec![
        get_squares_map(),
        get_pentagonals_map(),
        get_hexagonals_map(),
        get_heptagonals_map(),
        get_octagonals_map(),
    ];

    for t in triangles.iter() {
        let mut result: Vec<u32> = vec![*t];

        search(*t, &mut result, &mut map_vec);
    }
}

fn search(num: u32, result: &mut Vec<u32>, map_vec: &mut Vec<HashMap<u32, Vec<u32>>>) {
    if result.len() >= 6 {
        if result.first().unwrap() / 100 == result.last().unwrap() % 100 {
            println!("{} ({:?})", result.iter().sum::<u32>(), result);
        }

        return;
    }

    let last_2_digits: u32 = num % 100;

    for i in 0..map_vec.len() {
        let vec: Vec<u32> = match map_vec.get(i).unwrap().get(&last_2_digits) {
            None => Vec::<u32>::new(),
            Some(v) => v.to_vec(),
        };

        for v in vec.iter() {
            let map: &HashMap<u32, Vec<u32>> = map_vec.get(i).unwrap();
            let mut m: HashMap<u32, Vec<u32>> = HashMap::<u32, Vec<u32>>::new();
            m.extend(map.iter().map(|(k, v)| (k.clone(), v.clone())));

            result.push(*v);
            map_vec.remove(i);

            search(*v, result, map_vec);

            map_vec.insert(i, m);
            result.pop();
        }
    }
}

fn get_triangles() -> Vec<u32> {
    let mut triangles: Vec<u32> = Vec::<u32>::new();
    let mut i: u32 = 1;
    let mut t: u32 = triangle(i);

    while t < 10000 {
        if t >= 1000 {
            triangles.push(t);
        }

        i += 1;
        t = triangle(i);
    }

    triangles
}

fn triangle(i: u32) -> u32 {
    i * (i + 1) / 2
}

fn get_squares_map() -> HashMap<u32, Vec<u32>> {
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut i: u32 = 1;
    let mut q: u32 = triangle(i);

    while q < 10000 {
        if q >= 1000 {
            match map.get(&(q / 100)) {
                None => {
                    map.insert(q / 100, vec![q]);
                }
                Some(v) => {
                    let mut vec: Vec<u32> = v.to_vec();
                    vec.push(q);
                    map.insert(q / 100, vec);
                }
            }
        }

        i += 1;
        q = square(i);
    }

    map
}

fn square(i: u32) -> u32 {
    i * i
}

fn get_pentagonals_map() -> HashMap<u32, Vec<u32>> {
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut i: u32 = 1;
    let mut p: u32 = triangle(i);

    while p < 10000 {
        if p >= 1000 {
            match map.get(&(p / 100)) {
                None => {
                    map.insert(p / 100, vec![p]);
                }
                Some(v) => {
                    let mut vec: Vec<u32> = v.to_vec();
                    vec.push(p);
                    map.insert(p / 100, vec);
                }
            }
        }

        i += 1;
        p = pentagonal(i);
    }

    map
}

fn pentagonal(i: u32) -> u32 {
    i * (3 * i - 1) / 2
}

fn get_hexagonals_map() -> HashMap<u32, Vec<u32>> {
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut i: u32 = 1;
    let mut p: u32 = triangle(i);

    while p < 10000 {
        if p >= 1000 {
            match map.get(&(p / 100)) {
                None => {
                    map.insert(p / 100, vec![p]);
                }
                Some(v) => {
                    let mut vec: Vec<u32> = v.to_vec();
                    vec.push(p);
                    map.insert(p / 100, vec);
                }
            }
        }

        i += 1;
        p = hexagonal(i);
    }

    map
}

fn hexagonal(i: u32) -> u32 {
    i * (2 * i - 1)
}

fn get_heptagonals_map() -> HashMap<u32, Vec<u32>> {
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut i: u32 = 1;
    let mut p: u32 = triangle(i);

    while p < 10000 {
        if p >= 1000 {
            match map.get(&(p / 100)) {
                None => {
                    map.insert(p / 100, vec![p]);
                }
                Some(v) => {
                    let mut vec: Vec<u32> = v.to_vec();
                    vec.push(p);
                    map.insert(p / 100, vec);
                }
            }
        }

        i += 1;
        p = heptagonal(i);
    }

    map
}

fn heptagonal(i: u32) -> u32 {
    i * (5 * i - 3) / 2
}

fn get_octagonals_map() -> HashMap<u32, Vec<u32>> {
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut i: u32 = 1;
    let mut p: u32 = triangle(i);

    while p < 10000 {
        if p >= 1000 {
            match map.get(&(p / 100)) {
                None => {
                    map.insert(p / 100, vec![p]);
                }
                Some(v) => {
                    let mut vec: Vec<u32> = v.to_vec();
                    vec.push(p);
                    map.insert(p / 100, vec);
                }
            }
        }

        i += 1;
        p = octagonal(i);
    }

    map
}

fn octagonal(i: u32) -> u32 {
    i * (3 * i - 2)
}
