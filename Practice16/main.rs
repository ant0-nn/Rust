fn find_solutions() {
    let mut solutions = 0;

    for m in 1..=8 {
        for u in 1..=8 {
            if u == m { continue; }
            for x in 1..=8 {
                if x == m || x == u { continue; }
                for a in 1..=8 {
                    if a == m || a == u || a == x { continue; }
                    for s in 1..=8 {
                        if s == m || s == u || s == x || s == a { continue; }
                        for l in 1..=8 {
                            if l == m || l == u || l == x || l == a || l == s { continue; }
                            for o in 1..=8 {
                                if o == m || o == u || o == x || o == a || o == s || o == l { continue; }
                                for n in 1..=8 {
                                    if n == m || n == u || n == x || n == a || n == s || n == l || n == o { continue; }

                                    let muxa = 1000 * m + 100 * u + 10 * x + a;
                                    let slon = 1000 * s + 100 * l + 10 * o + n;

                                    if muxa * a == slon {
                                        solutions += 1;
                                        println!("{:4}\n×   {}\n------\n{:4}\n", muxa, a, slon);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Кількість рішень: {}", solutions);
}

fn main() {
    find_solutions();
}