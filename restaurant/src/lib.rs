mod front_of_house;

mod customer {
    use crate::front_of_house::hosting;
    use crate::back_of_house::Appetizer;
    use crate::back_of_house::Breakfast;

    pub fn eat_at_restaurant() {
        // Absolute path
        hosting::add_to_waitlist();

        // Relative path
        hosting::add_to_waitlist();

        // Order a breakfast in the summer with Rye toast
        let mut meal = Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");

        let order1 = Appetizer::Soup;
        let order2 = Appetizer::Salad;
    }
}

fn deliver_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
