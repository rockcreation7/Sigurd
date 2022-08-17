
use std::collections::HashMap;

pub struct Product {
    pub name: String,
    pub price: f32,
}

pub fn get_product() ->HashMap<i32, Product>{
   let product_list:  HashMap<i32, Product> = HashMap::from([
        (
            1,
            Product {
                name: "Tofu 280g".to_owned(),
                price: 2.00,
            },
        ),
        (
            2,
            Product {
                name: "Beef Ball 250g".to_owned(),
                price: 4.00,
            },
        ),
    ]);

    return product_list
}
 