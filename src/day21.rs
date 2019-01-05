use aoc_runner_derive::aoc;

#[aoc(day21, part1)]
fn find_r0_part1(_input: &str) -> u32 {
    let mut r3 = 0u32;
    let mut r2;
    'outer: loop {
        r2 = r3 | 65536;
        r3 = 832312;
        //println!("loop 1: r2={}", r2);

        'inner: loop {
            r3 = (((r3 & 16777215) + (r2 & 255)) * 65899) & 16777215;
            //let tr3 = (((r3 % (1 << 25)) + (r2 % 256)) * 65899) % (1 << 25);
            //println!("loop 2: r2={} r3={} -> {}", r2, r3, tr3);
            //r3 = tr3;
            if r2 < 256 {
                break;
            }
            r2 = r2 / 256;
        }
        println!("r3={}", r3);
        break;
    }
    r3
}

// Too high: 24251965
//           30693365
//           9258470

#[aoc(day21, part2)]
fn find_r0_part2(_input: &str) -> u32 {
    let mut r3 = 0u32;
    let mut r2;
    let mut reg_hist: Vec<u32> = Vec::new();
    'outer: loop {
        match reg_hist.iter().enumerate().find(|&item| *item.1 == r3) {
            Some((idx, _r3val)) => {
                println!("r3={}, pos={}, len={}, regs={:?}, ... {:?}", r3, idx, reg_hist.len(), &reg_hist[0..10], &reg_hist[reg_hist.len()-10..]);
                break;
            }
            None => {
                reg_hist.push(r3);
            }
        }
        r2 = r3 | 65536;
        r3 = 832312;
        //println!("loop 1: r2={}", r2);

        'inner: loop {
            r3 = (((r3 & 16777215) + (r2 & 255)) * 65899) & 16777215;
            //let tr3 = (((r3 % (1 << 25)) + (r2 % 256)) * 65899) % (1 << 25);
            //println!("loop 2: r2={} r3={} -> {}", r2, r3, tr3);
            //r3 = tr3;
            if r2 < 256 {
                break;
            }
            r2 = r2 / 256;
        }
    }
    let ans = *reg_hist.last().unwrap();
    ans
}