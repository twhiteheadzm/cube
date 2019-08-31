pub fn get_rots() -> Vec<Vec<i32>> {
    let mut rots: Vec<Vec<i32>> = Vec::new();
    fn remainder(rots: &mut Vec<Vec<i32>>, m: Vec<i32>, loc: i32) {
        if loc == 9 {
            let perm = is_perm(&m);
            if !perm {
                return;
            }
            let d = det(&m);
            if d != 1 {
                return;
            }
            rots.push(m);
        // console_log!("Matrix {:?} {} ", m, d);
        } else {
            for value in -1..2 {
                let mut m2 = m.clone();
                m2.push(value);
                remainder(&mut *rots, m2, loc + 1);
            }
        }
    }

    let m = Vec::new();
    remainder(&mut rots, m, 0);
    rots
}

fn is_perm(values: &Vec<i32>) -> bool {
    for row in 0..3 {
        let mut sum = 0;
        for col in 0..3 {
            sum += values[col * 3 + row].abs();
        }
        if sum != 1 {
            return false;
        }
    }
    for col in 0..3 {
        let mut sum = 0;
        for row in 0..3 {
            sum += values[col * 3 + row].abs();
        }
        if sum != 1 {
            return false;
        }
    }

    true
}

fn det(values: &Vec<i32>) -> i32 {
    values[0] * values[4] * values[8]
        + values[3] * values[7] * values[2]
        + values[6] * values[1] * values[5]
        - values[6] * values[4] * values[2]
        - values[3] * values[1] * values[8]
        - values[0] * values[7] * values[5]
}
