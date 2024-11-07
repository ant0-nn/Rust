struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let excerpt = ImportantExcerpt {
        part: "This is a very important part of the text.",
    };
    let announcement = "New announcement!";
    let result = excerpt.announce_and_return_part(announcement);
    
    println!("Result: {}", result);
    println!("Success!");
}
