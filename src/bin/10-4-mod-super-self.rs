fn main() {
    my::indirect_call();
}

fn function() {
    println!("called `function()`");
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    pub fn indirect_call() {
        print!("called `my::indirect_call()`, that\n> ");

        self::function();
        function();

        self::cool::function();
        cool::function();

        super::function();

        {
            use cool::function as root;
            root();
        }
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}