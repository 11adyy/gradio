use std::collections::HashMap;

fn test(m1: i32, m2: i32, m3: i32, m4: i32, m5: i32, max_c: &mut HashMap<usize, f64>) -> f64 {
    let d1: f64 = 0.005;
    let mut first: f64 = 0.5;
    let d2: f64 = 0.05;
    let d3: f64 = 0.1;
    let d4: f64 = 1.0;
    let d5: f64 = 10.0;

    let n1: i32 = (m1 as f64 - first / d1) as i32;
    let n2: i32 = (m2 as f64 - m1 as f64 * d1 / d2) as i32;
    let n3: i32 = (m3 as f64 - m2 as f64 * d2 / d3) as i32;
    let n4: i32 = (m4 as f64 - m3 as f64 * d3 / d4) as i32;
    let n5: i32 = (m5 as f64 - m4 as f64 * d4 / d5 + 1.0) as i32;

    let mut set_size: Vec<i32> = Vec::new();

    for _ in 0..n1 {
        first += d1;
        first = (first * 1000.0).round() / 1000.0;
        set_size.push(first as i32);
    }

    for _ in 0..n2 {
        first += d2;
        first = (first * 1000.0).round() / 1000.0;
        set_size.push(first as i32);
    }

    for _ in 0..n3 {
        first += d3;
        first = (first * 1000.0).round() / 1000.0;
        set_size.push(first as i32);
    }

    for _ in 0..n4 {
        first += d4;
        first = (first * 1000.0).round() / 1000.0;
        set_size.push(first as i32);
    }

    for _ in 0..n5 {
        first += d5;
        first = (first * 1000.0).round() / 1000.0;
        set_size.push(first as i32);
    }

    // let gen = SizeGen::new(set_size.clone());
    let delta: f64 = 0.005;
    let mut first_size = 0.0;
    let mut end_size = 0.0;
    let mut coef = 0.0;
    let mut ipk = 0.0;

    // let _ksr = gen.find_sequence_correct(delta, &mut first_size, &mut end_size, &mut coef, &mut ipk);

    let entry = max_c.entry(set_size.len()).or_insert(coef);
    if coef > *entry {
        *entry = coef;
    }

    return coef;
}

fn search1(max_c: &mut HashMap<usize, f64>) {
    let max_k: i32 = 100;

    for k1 in -151..max_k {
        for k2 in (k1 + 1)..max_k {
            for k3 in (k2 + 1)..max_k {
                for k4 in 0..max_k {
                    if -9 * k4 - 9 * k3 + 8 * k1 > 0 {
                        let m1: i32 = 10 * k4;
                        let m2: i32 = 2 * (-9 * k4 - 9 * k3 + 8 * k1);
                        let m3: i32 = 10 * (k3 - k2);
                        let m4: i32 = 10 * (k2 - k1);
                        let m5: i32 = k1 + 152;

                        if (m1 as f64) >= 11.0
                            && (m2 as f64 - 0.1 * m1 as f64) > 0.0
                            && (m3 as f64 - 0.5 * m2 as f64) > 0.0
                            && (m4 as f64 - 0.1 * m3 as f64) > 0.0
                            && (m5 as f64 - 0.1 * m4 as f64) > 0.0
                        {
                            test(m1, m2, m3, m4, m5, max_c);
                        }
                    }
                }
            }
        }
    }
}

fn search2(max_c: &mut HashMap<usize, f64>) {
    let mut max_coef: f64 = 0.0;

    for m1 in (140..171).step_by(10) {
        for m2 in (12..63).step_by(2) {
            for m3 in (10..61).step_by(10) {
                for m4 in (10..53).step_by(10) {
                    for m5 in 1..11 {
                        if (m2 as f64 - 0.1 * m1 as f64) > 0.0
                            && (m3 as f64 - 0.5 * m2 as f64) > 0.0
                            && (m4 as f64 - 0.1 * m3 as f64) > 0.0
                            && (m5 as f64 - 0.1 * m4 as f64) > 0.0
                        {
                            let coef = test(m1, m2, m3, m4, m5, max_c);

                            if coef > max_coef {
                                max_coef = coef;
                                println!("coef = {}", coef);
                                println!("{} {} {} {} {}", m1, m2, m3, m4, m5);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn test2(m1: i32, m2: i32, m3: i32, m4: i32, m5: i32, max_c: &mut HashMap<usize, f64>) -> f64 {
    let d1 = 0.005;
    let first1 = 0.5;
    let d2 = 0.01;
    let d3 = 0.1;
    let d4 = 0.5;
    let d5 = 10.0;

    let n1 = m1 - 200;
    let n2 = (m2 as f64 - 0.5 * m1 as f64 + 1.5) as i32;
    let n3 = (m3 as f64 - 0.1 * m2 as f64) as i32;
    let n4 = (m4 as f64 - 0.2 * m3 as f64) as i32;
    let n5 = (m5 as f64 - 0.05 * m4 as f64) as i32;

    let mut set_size: Vec<f64> = Vec::new();
    set_size.push(first1);

    let mut first = 2.0 * first1 + d1;
    for _ in 0..n1 {
        first = ((first * 1000.0) as i32 as f64) / 1000.0;
        set_size.push(first);
        first += d1;
    }

    first = 2.0 * first1;
    for _ in 0..(n2 - 1) {
        first = ((first * 1000.0) as i32 as f64) / 1000.0;
        set_size.push(first);
        first += d2;
    }

    for _ in 0..n3 {
        first = ((first * 1000.0) as i32 as f64) / 1000.0;
        set_size.push(first);
        first += d3;
    }

    for _ in 0..n4 {
        first = ((first * 1000.0) as i32 as f64) / 1000.0;
        set_size.push(first);
        first += d4;
    }

    for _ in 0..(n5 + 1) {
        first = ((first * 1000.0) as i32 as f64) / 1000.0;
        set_size.push(first);
        first += d5;
    }

    // let gen = SizeGen::new(set_size.clone());
    let delta = 0.005;
    let coef: f64 = 0.;
    // let (_ksr, _first_size, _end_size, coef, _ipk) = gen.find_sequence_correct(delta);

    let entry = max_c.entry(set_size.len()).or_insert(coef);
    if coef > *entry {
        *entry = coef;
    }

    return coef;
}

fn search3(max_c: &mut HashMap<usize, f64>) {
    let mut max_coef: f64 = 0.0;

    for m1 in (201..202).step_by(2) {
        let mut d = (0.5 * m1 as f64 - 1.5) as i32;
        while d % 10 != 0 {
            d += 1;
        }

        let mut m2 = d;
        while m2 < 201 {
            println!("m2 = {}", m2);

            d = (0.1 * m2 as f64) as i32;
            while d % 5 != 0 {
                d += 1;
            }

            let mut m3 = d;
            while m3 < 101 {
                d = (0.2 * m3 as f64) as i32;
                while d % 20 != 0 {
                    d += 1;
                }

                let mut m4 = d;
                while m4 < 101 {
                    d = 8;
                    for m5 in (d + 1)..11 {
                        let coef = test2(m1, m2, m3, m4, m5, max_c);
                        if coef > max_coef {
                            max_coef = coef;
                            println!("coef = {}", coef);
                            println!("{} {} {} {} {}", m1, m2, m3, m4, m5);
                        }
                    }

                    m4 += 20;
                }

                m3 += 5;
            }

            m2 += 10;
        }
    }
}
