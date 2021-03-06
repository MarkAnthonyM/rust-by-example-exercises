mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

pub fn indirect_access() {
    println!("called `my::indirect_access()`");
    private_function();
    inaccessible::function();
}

fn private_function() {
    println!("> called my::private_function()");
}