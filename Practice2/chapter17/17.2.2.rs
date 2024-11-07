#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}

static mut CONFIG_STORAGE: Option<Config> = None;
static mut CONFIG: Option<&mut Config> = None;

fn init() -> Option<&'static mut Config> {
    unsafe {
        CONFIG_STORAGE = Some(Config {
            a: "A".to_string(),
            b: "B".to_string(),
        });
        CONFIG_STORAGE.as_mut()
    }
}

fn main() {
    unsafe {
        CONFIG = init();

        println!("{:?}", CONFIG);
    }
}