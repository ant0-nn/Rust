struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn main() {
    let excerpt = ImportantExcerpt {
        part: "This is an important excerpt.",
    };

    println!("Level: {}", excerpt.level());
}
