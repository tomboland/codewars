fn _nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    let target_pop = p as f64;
    let augment = aug as f64;
    let mut current_pop = p0 as f64;
    for t in 1.. {
        current_pop = current_pop + current_pop * (percent / 100.) + augment;
        if current_pop > target_pop {
            return t;
        }
    }
    return 0;
}

fn nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    (1..)
        .scan(p0, |curr_pop, x| {
            *curr_pop = *curr_pop + ((*curr_pop as f64) * (percent / 100.)) as i32 + aug;
            Some((*curr_pop, x))
        })
        .find(|(curr_pop, _)| curr_pop >= &p)
        .unwrap()
        .1
}

fn dotest(p0: i32, percent: f64, aug: i32, p: i32, exp: i32) -> () {
    println!("p0: {:?};", p0);
    println!("percent: {:?};", percent);
    println!("aug: {:?};", aug);
    println!("p: {:?};", p);
    let ans = nb_year(p0, percent, aug, p);
    println!("actual:\n{:?};", ans);
    println!("expect:\n{:?};", exp);
    println!("{};", ans == exp);
    assert_eq!(ans, exp);
    println!("{};", "-");
}

#[test]
fn basic_tests() {
    dotest(1500, 5.0, 100, 5000, 15);
    dotest(1500000, 2.5, 10000, 2000000, 10);
}
