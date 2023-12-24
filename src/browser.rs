mod ops

struct Browser {
    entries ;
    pointer u32;
    dir_stack ;
}

impl Browser {
    fn up() {

    }
    fn down() {

    }
    fn left() {

    }
    fn right() {

    }
}

fn process_input() {

}

fn init() {
    let browser = Browser() {
        entries:,
        pointer:,
        dir_stack:
    };
    while true {
        match process_input() {
            ops::UP => browser.up();
            ops::DOWN => browser.down();
            ops::LEFT => browser.left();
            ops::RIGHT => browser.right();
        }
        // display()
    }
}
