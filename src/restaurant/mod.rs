mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: bool,
        pub topping: String,
        pub size_in_oz: f32,
    }

    impl Pizza {
        pub fn breakfast() -> Pizza {
            Pizza {
                dough: String::from("no-dough"),
                cheese: false,
                topping: String::from("no"),
                size_in_oz: 0.0,
            }
        }
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: String::from("pizza-dough"),
                cheese: true,
                topping: String::from(topping),
                size_in_oz: 28.2,
            }
        }
        pub fn dinner(cheese: bool, topping: &str, size_in_oz: f32) -> Pizza {
            Pizza {
                dough: String::from("pizza-dough"),
                cheese: cheese,
                topping: String::from(topping),
                size_in_oz: size_in_oz.to_owned(),
            }
        }
    }

    pub mod help_customer {
        use crate::restaurant::meal::Meal;

        fn seat_at_table() {
            println!("seated...");
        }
        pub fn take_order(meal: Meal) {
            seat_at_table();
            let cust_piz = match meal {
                Meal::Lunch => Ok(super::Pizza::lunch("veggies")),
                Meal::Dinner => Ok(super::Pizza::dinner(true, "pepper", 33.3)),
                Meal::Breakfast => {
                    Err("System Error: Contact Admin - \"Breakfast isn't a time for pizza...\"")
                }
            };
            let mut pizza = match cust_piz {
                Ok(pizza) => pizza,
                Err(error) => panic!("Problem creating pizza: {:?}", error),
            };
            serve_customer(pizza);
        }
        fn serve_customer(cust_piz: super::Pizza) {
            println!(
                "The Customer is served a {}oz {} pizza with{} cheeze and {} toppings.",
                cust_piz.size_in_oz,
                cust_piz.dough,
                if cust_piz.cheese {
                    String::from("")
                } else {
                    String::from("out")
                },
                if cust_piz.topping.is_empty() {
                    String::from("no")
                } else {
                    cust_piz.topping
                }
            );
        }
    }
}

pub mod meal {
    pub enum Meal {
        Breakfast,
        Lunch,
        Dinner,
    }

    impl Meal {
        pub fn is_pizzatime(&self) -> bool {
            match self {
                Meal::Lunch | Meal::Dinner => true,
                _ => false,
            }
        }
    }
}

pub fn order_food(meal: meal::Meal) {
    crate::restaurant::pizza_order::help_customer::take_order(meal);
}
