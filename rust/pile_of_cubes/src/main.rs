struct CubeSeries {
    acc: u64,
    n: u64,
}

impl Iterator for CubeSeries {
    type Item = (u64, u64);
    fn next(&mut self) -> Option<Self::Item> {
        let next = ( self.acc, self.n );
        self.acc = self.acc + u64::pow(self.n + 1, 3);
        self.n = self.n + 1;
        Some(next)
    }
}

fn make_cube_series () -> CubeSeries {
    CubeSeries { acc: 0, n: 0 }
}

fn find_nb_2(m: u64) -> i32 {
   match make_cube_series().take_while(|x| x.0 <= m).find(|x| x.0 == m) {
    Some((_, n)) => n as i32,
    _ => -1
   }
}

fn find_nb(m: u64) -> i32 {
    let n = f64::sqrt(8. * f64::sqrt(m as f64)+ 1.) / 2. - 1. / 2.;
    if n.trunc() == n { n.trunc() as i32 } else { -1 }
}

fn do_test(n: u64, exp: i32) {
    assert_eq!(find_nb(n), exp, 
                "\nYour result (left) did not match expected output (right) for n={n}");
}

#[test]
fn basics_find_nb() {
    let cases = [
        (4,               -1),
        (16,              -1),
        (4183059834009,   2022),
        (24723578342962,  -1),
        (135440716410000, 4824),
        (40539911473216,  3568),
        (26825883955641,  3218),
    ];
    for (n, expected) in cases {
        do_test(n, expected);
    }
}