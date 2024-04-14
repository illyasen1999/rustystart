// if the project gets larger its better to separate the moduls into files
pub mod hosting; // the definition of the hosting module can also be extracted to another file of the same name as the module

fn _deliver_order() {} // this function is outside of the module but can be called by using the "super::" keyword

pub mod serving {
    fn _fix_incorrect_order(){
        // this function is accessible because its part of the module
        _cook_order();

        // deliver_order() function is accessed from outside of the module using the "super::" syntax
        super::_deliver_order();
    }

    pub struct MainCourse {
        main_dish: String,
        main_dessert: String,
        serving_size: i32,
        currently_serving: bool,
    }

    impl MainCourse {
        pub fn now_serving() {
            let serving = MainCourse {
                main_dish: String::from("Wagyu Steak"),
                main_dessert: String::from("Halo Halo"),
                serving_size: 5,
                currently_serving: true,
            };
            println!("\nMain Dish: {} \nMain Dessert: {} \nServing Size: {} \nCurrently Serving: {}", serving.main_dish, serving.main_dessert, serving.serving_size, serving.currently_serving);
        }
    }

    fn _cook_order() {
        println!("Cook order");
    }

    fn _take_order() {
        println!("Take order");
    }

    fn _serve_order() {
        println!("Serve order");
    }

    fn _take_payment() {
        println!("Take payment");
    }
}