fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }

    let n = ((n % len as isize) + len as isize) % len as isize;
    let split = len - n as usize;

    format!("{}{}", &s[split..], &s[..split])
}

#[test]
fn test() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    shifts.iter().for_each(|(n, exp)| {
        assert_eq!(
            rotate(s.clone(), *n),
            exp.to_string()
        );
    });
}

fn main() {
    let result = rotate("abcdefgh".to_string(), 3);
    println!("{}", result);
}