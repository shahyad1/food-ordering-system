use colored::Colorize;
use std::io;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    password: String,
    role: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct MenuItem {
    name: String,
    price: u32,
}


#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    name: String,
    owner: String,
    category: String,
    menu: Vec<MenuItem>,
}


#[derive(Serialize, Deserialize, Debug)]
struct Order {
    username: String,
    restaurant: String,
    items: Vec<OrderItem>,
    total_price: u32,
    datetime: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct OrderItem {
    name: String,
    quantity: u32,
}


fn load_users() -> Result<Vec<User>, String> {
    if !Path::new("data.json").exists() {
        return Ok(vec![]);
    }

    let data = fs::read_to_string("data.json").expect("Failed to read data.json");
    if data.trim().is_empty() {
        return Ok(vec![]);
    }

    let users: Vec<User> = serde_json::from_str(&data).expect("Failed to parse JSON");
    Ok(users)
}


fn save_users(users: &Vec<User>) -> Result<(), String> {
    let data = serde_json::to_string_pretty(users).expect("Failed to serialize users");

    fs::write("data.json", data).expect("ailed to write to data.json");
    Ok(())
}


fn load_restaurants() -> Result<Vec<Restaurant>, String> {
    if !Path::new("restaurants.json").exists() {
        return Ok(vec![]);
    }

    let data = fs::read_to_string("restaurants.json").expect("Failed to read restaurants.json");
    if data.trim().is_empty() {
        return Ok(vec![]);
    }

    let restaurants: Vec<Restaurant> = serde_json::from_str(&data).expect("Failed to parse JSON");
    Ok(restaurants)
}


fn save_restaurants(restaurants: &Vec<Restaurant>) -> Result<(), String> {
    let data = serde_json::to_string_pretty(restaurants).expect("Failed to serialize restaurants");

    fs::write("restaurants.json", data).expect("Failed to write to restaurants.json");
    Ok(())
}


fn load_orders() -> Result<Vec<Order>, String> {
    if !Path::new("orders.json").exists() {
        return Ok(vec![]);
    }

    let data = fs::read_to_string("orders.json").expect("Failed to read orders.json");
    if data.trim().is_empty() {
        return Ok(vec![]);
    }

    let orders: Vec<Order> = serde_json::from_str(&data).expect("Failed to parse JSON");
    Ok(orders)
}


fn save_orders(orders: &Vec<Order>) -> Result<(), String> {
    let data = serde_json::to_string_pretty(orders).expect("Failed to serialize orders");

    fs::write("orders.json", data).expect("Failed to write to orders.json");
    Ok(())
}


fn register() {

    let mut username = String::new();
    let mut password = String::new();
    let role: String;

    println!("Enter username:");
    io::stdin().read_line(&mut username).expect("Failed to read username");
    println!("Enter password:");
    io::stdin().read_line(&mut password).expect("Failed to read password");

    println!("{}", "Select your role from the list below:".green());
    println!("{}", "1. User".yellow());
    println!("{}", "2. Restaurant owner".yellow());
    println!("{}", "3. Admin".yellow());

    let mut role_my_str = String::new();
    io::stdin().read_line(&mut role_my_str).expect("Failed to read input");

    role = match role_my_str.trim() {
        "1" => "user".to_string(),
        "2" => "restaurant_owner".to_string(),
        "3" => "admin".to_string(),
        _ => {
            println!("{}", "Wrong please try again!".red());
            return;
        }
    };

    let username = username.trim().to_string();
    let password = password.trim().to_string();

    if username.is_empty() || password.is_empty() {
        println!("{}", "Username or password cannot be empty!".red());
        return;
    }

    if password.len() < 4 {
        println!("{}", "Password must be at least 4 characters long!".red());
        return;
    }

    let mut users = match load_users() {
        Ok(users) => users,
        Err(e) => {
            println!("{}: {}", "Error loading users".red(), e);
            return;
        }
    };

    if users.iter().any(|u| u.username == username) {
        println!("{}", "Username exists!".red());
        return;
    }

    users.push(User {
        username,
        password,
        role,
    });
    
    if let Err(e) = save_users(&users) {
        println!("{}: {}", "Error saving users".red(), e);
        return;
    }
    
    println!("{}", "Registration successful!".green());
}


fn login() -> Option<User> {

    let mut username = String::new();
    let mut password = String::new();

    println!("Enter username:");
    io::stdin().read_line(&mut username).expect("Failed to read username");
    println!("Enter password:");
    io::stdin().read_line(&mut password).expect("Failed to read password");

    let username = username.trim().to_string();
    let password = password.trim().to_string();

    if username.is_empty() || password.is_empty() {
        println!("{}", "Username or password cannot be empty!".red());
        return None;
    }

    let users = match load_users() {
        Ok(users) => users,
        Err(e) => {
            println!("{}: {}", "Error loading users".red(), e);
            return None;
        }
    };

    for user in users {
        if user.username == username && user.password == password {
            return Some(user);
        }
    }
    None
}


//user توابع
fn view_restaurants(show_menu: bool) -> bool {

    let restaurants = match load_restaurants() {
        Ok(restaurants) => restaurants,
        Err(e) => {
            println!("{}: {}", "Error loading restaurants".red(), e);
            return false;
        }
    };

    if restaurants.is_empty() {
        println!("{}", "No restaurants available.".yellow());
        return false;
    }

    for (i, restaurant) in restaurants.iter().enumerate() {
        println!("{}. {} (Category: {})", i + 1, restaurant.name, restaurant.category);
        if show_menu && !restaurant.menu.is_empty() {
            println!("  Menu:");
            for item in &restaurant.menu {
                println!("  * {}: {}", item.name, item.price);
            }
        }
    }
    true
}


fn place_order(username: &str) {

    let restaurants = match load_restaurants() {
        Ok(restaurants) => restaurants,
        Err(e) => {
            println!("{}: {}", "Error loading restaurants".red(), e);
            return;
        }
    };

    if restaurants.is_empty() {
        println!("{}", "No restaurants available.".yellow());
        return;
    }

    println!("{}", "Select a restaurant:".cyan());
    for (i, restaurant) in restaurants.iter().enumerate() {
        println!("{}. {} (Category: {})", i + 1, restaurant.name, restaurant.category);
    }

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read choise");
    let index: usize = match choice.trim().parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "Invalid input!".red());
            return;
        }
    };

    if index == 0 || index > restaurants.len() {
        println!("{}", "Invalid restaurant selection!".red());
        return;
    }

    let selected_restaurant = &restaurants[index - 1];
    println!("Menu for {}:", selected_restaurant.name);
    for (i, item) in selected_restaurant.menu.iter().enumerate() {
        println!("{}. {} - {}", i + 1, item.name, item.price);
    }

    let mut order_items = Vec::new();
    let mut total_price = 0;

    loop {
        println!("Enter item number to add:");
        let mut item_choice = String::new();
        io::stdin().read_line(&mut item_choice).expect("Failed to read choise");
        let item_index: usize = match item_choice.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Invalid input! Please enter a number.".red());
                continue;
            }
        };

        if item_index == 0 {
            break;
        }

        if item_index > selected_restaurant.menu.len() {
            println!("{}", "Invalid item selection!".red());
            continue;
        }

        println!("Enter quantity:");
        let mut quantity = String::new();
        io::stdin().read_line(&mut quantity).expect("Failed to read quantity");
        let quantity: u32 = match quantity.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Invalid quantity! Please enter a number.".red());
                continue;
            }
        };

        if quantity == 0 {
            println!("{}", "Quantity must be greater than 0!".red());
            continue;
        }

        let selected_item = &selected_restaurant.menu[item_index - 1];
        order_items.push(OrderItem {
            name: selected_item.name.clone(),
            quantity,
        });
        total_price += selected_item.price * quantity;
    }

    if order_items.is_empty() {
        println!("{}", "No items selected. Order cancelled.".yellow());
        return;
    }

    let mut orders = match load_orders() {
        Ok(orders) => orders,
        Err(e) => {
            println!("{}: {}", "Error loading orders".red(), e);
            return;
        }
    };

    let datetime = Utc::now().to_rfc3339(); 
    orders.push(Order {
        username: username.to_string(),
        restaurant: selected_restaurant.name.clone(),
        items: order_items,
        total_price,
        datetime,
    });

    
    if let Err(e) = save_orders(&orders) {
        println!("{}: {}", "Error saving order".red(), e);
        return;
    }

    println!("{}", "Order placed successfully!".green());
    println!("Total Price: {}", total_price);
}


fn view_orders(username: &str) {

    let orders = match load_orders() {
        Ok(orders) => orders,
        Err(e) => {
            println!("{}: {}", "Error loading orders".red(), e);
            return;
        }
    };

    let user_orders: Vec<&Order> = orders.iter().filter(|o| o.username == username).collect();
    if user_orders.is_empty() {
        println!("{}", "No orders found.".yellow());
        return;
    }

    println!("{}", "Your Orders:".green());
    for order in user_orders {
        println!("Restaurant: {}", order.restaurant);
        println!("Items:");
        for item in &order.items {
            println!("* {} (x{})", item.name, item.quantity);
        }
        println!("Total Price: {}", order.total_price);
        println!("Date: {}", order.datetime);
        println!("****");
    }
}


fn user_menu(user: &User) {
    loop {
        println!("{}", "\nUser Menu:".cyan());
        println!("{}", "1. View Restaurants".blue());
        println!("{}", "2. View Menu".blue());
        println!("{}", "3. Place Order".blue());
        println!("{}", "4. View Order History".blue());
        println!("{}", "5. Logout".red());

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read choice");

        match choice.trim() {
            "1" => {
                view_restaurants(false);
            }
            "2" => {
                view_restaurants(true);
            }
            "3" => place_order(&user.username),
            "4" => view_orders(&user.username),
            "5" => break,
            _ => println!("{}", "Wrong please try again!!".red()),
        }
    }
}


//restaurant توابع
fn add_restaurant(owner: &str) {

    let mut name = String::new();
    let mut category = String::new();

    println!("Enter restaurant name:");
    io::stdin().read_line(&mut name).expect("Failed to read input");
    println!("Enter category:");
    io::stdin().read_line(&mut category).expect("Failed to read input");

    let name = name.trim().to_string();
    let category = category.trim().to_string();

    if name.is_empty() || category.is_empty() {
        println!("{}", "Name or category cannot be empty!".red());
        return;
    }

    let mut restaurants = match load_restaurants() {
        Ok(restaurants) => restaurants,
        Err(e) => {
            println!("{}: {}", "Error loading restaurants".red(), e);
            return; 
        }
    };

    if restaurants.iter().any(|r| r.name == name) {
        println!("{}", "Restaurant exists!".red());
        return;
    }

    restaurants.push(Restaurant {
        name,
        owner: owner.to_string(),
        category,
        menu: vec![],
    });

    
    if let Err(e) = save_restaurants(&restaurants) {
        println!("{}: {}", "Error saving restaurant".red(), e);
        return;
    }

    println!("{}", "Restaurant added successfully!".green());
}


fn manage_menu(owner: &str) {

    let mut restaurants = match load_restaurants() {
        Ok(restaurants) => restaurants,
        Err(e) => {
            println!("{}: {}", "Error loading restaurants".red(), e);
            return;
        }
    };

    let owner_restaurants: Vec<&Restaurant> = restaurants.iter().filter(|r| r.owner == owner).collect();
    if owner_restaurants.is_empty() {
        println!("{}", "You don't own any restaurants.".yellow());
        return;
    }

    println!("{}", "Your Restaurants:".cyan());
    for (i, restaurant) in owner_restaurants.iter().enumerate() {
        println!("{}. {}", i + 1, restaurant.name);
    }

    println!("Select a restaurant:");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read choise");
    let index: usize = match choice.trim().parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "Invalid input! Please enter a number.".red());
            return;
        }
    };

    if index == 0 || index > owner_restaurants.len() {
        println!("{}", "Invalid restaurant selection!".red());
        return;
    }

    let restaurant_name = owner_restaurants[index - 1].name.clone();
    let restaurant_index = restaurants.iter().position(|r| r.name == restaurant_name).unwrap();

    loop {
        println!("\nManaging menu for {}:", restaurant_name);
        println!("{}", "1. Add Item".blue());
        println!("{}", "2. Edit Item".blue());
        println!("{}", "3. Remove Item".blue());
        println!("{}", "4. Back".red());

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                let mut item_name = String::new();
                let mut item_price = String::new();

                println!("Enter item name:");
                io::stdin().read_line(&mut item_name).expect("Failed to read input");
                println!("Enter item price:");
                io::stdin().read_line(&mut item_price).expect("Failed to read input");

                let item_name = item_name.trim().to_string();
                let item_price: u32 = match item_price.trim().parse::<u32>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("{}", "Invalid price! Please enter a number.".red());
                        continue;
                    }
                };

                if item_name.is_empty() {
                    println!("{}", "Item name cannot be empty!".red());
                    continue;
                }

                restaurants[restaurant_index].menu.push(MenuItem {
                    name: item_name,
                    price: item_price,
                });

                if let Err(e) = save_restaurants(&restaurants) {
                    println!("{}: {}", "Error saving menu".red(), e);
                    return;
                }
                println!("{}", "Item added successfully!".green());
            }
            "2" => {
                if restaurants[restaurant_index].menu.is_empty() {
                    println!("{}", "Menu is empty.".yellow());
                    continue;
                }

                println!("Menu:");
                for (i, item) in restaurants[restaurant_index].menu.iter().enumerate() {
                    println!("{}. {} - {}", i + 1, item.name, item.price);
                }

                println!("Enter item number to edit (0 to cancel):");
                let mut item_choice = String::new();
                io::stdin().read_line(&mut item_choice).expect("Failed to read input");
                let item_index: usize = match item_choice.trim().parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("{}", "Invalid input! Please enter a number.".red());
                        continue;
                    }
                };

                if item_index == 0 {
                    continue;
                }

                if item_index > restaurants[restaurant_index].menu.len() {
                    println!("{}", "Invalid item selection!".red());
                    continue;
                }

                let mut new_name = String::new();
                let mut new_price = String::new();

                println!("Enter new item name:");
                io::stdin().read_line(&mut new_name).expect("Failed to read input");
                println!("Enter new item price (or press Enter to keep current):");
                io::stdin().read_line(&mut new_price).expect("Failed to read input");

                let new_name = new_name.trim();
                let new_price: Option<u32> = if new_price.trim().is_empty() {
                    None
                } else {
                    match new_price.trim().parse::<u32>() {
                        Ok(num) => Some(num),
                        Err(_) => {
                            println!("{}", "Invalid price! Keeping current price.".yellow());
                            None
                        }
                    }
                };

                if !new_name.is_empty() {
                    restaurants[restaurant_index].menu[item_index - 1].name = new_name.to_string();
                }
                if let Some(price) = new_price {
                    restaurants[restaurant_index].menu[item_index - 1].price = price;
                }

                if let Err(e) = save_restaurants(&restaurants) {
                    println!("{}: {}", "Error saving menu".red(), e);
                    return;
                }
                println!("{}", "Item edited successfully!".green());
            }
            "3" => {
                if restaurants[restaurant_index].menu.is_empty() {
                    println!("{}", "Menu is empty.".yellow());
                    continue;
                }

                println!("Menu:");
                for (i, item) in restaurants[restaurant_index].menu.iter().enumerate() {
                    println!("{}. {} - {}", i + 1, item.name, item.price);
                }

                println!("Enter item number to remove (0 to cancel):");
                let mut item_choice = String::new();
                io::stdin().read_line(&mut item_choice).expect("Failed to read input");
                let item_index: usize = match item_choice.trim().parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("{}", "Invalid input! Please enter a number.".red());
                        continue;
                    }
                };

                if item_index == 0 {
                    continue;
                }

                if item_index > restaurants[restaurant_index].menu.len() {
                    println!("{}", "Invalid item selection!".red());
                    continue;
                }

                restaurants[restaurant_index].menu.remove(item_index - 1);
                if let Err(e) = save_restaurants(&restaurants) {
                    println!("{}: {}", "Error saving menu".red(), e);
                    return;
                }
                println!("{}", "Item removed successfully!".green());
            }
            "4" => break,
            _ => println!("{}", "Wrong please try again!".red()),
        }
    }
}


fn view_restaurant_orders(owner: &str) {

    let restaurants = match load_restaurants() {
        Ok(restaurants) => restaurants,
        Err(e) => {
            println!("{}: {}", "Error loading restaurants".red(), e);
            return;
        }
    };

    let owner_restaurants: Vec<&Restaurant> = restaurants.iter().filter(|r| r.owner == owner).collect();
    if owner_restaurants.is_empty() {
        println!("{}", "You don't own any restaurants.".yellow());
        return;
    }

    let orders = match load_orders() {
        Ok(orders) => orders,
        Err(e) => {
            println!("{}: {}", "Error loading orders".red(), e);
            return;
        }
    };

    let mut found_orders = false;
    for restaurant in owner_restaurants {
        let restaurant_orders: Vec<&Order> = orders.iter().filter(|o| o.restaurant == restaurant.name).collect();
        if !restaurant_orders.is_empty() {
            found_orders = true;
            println!("{}", format!("Orders for {}:", restaurant.name).green());
            for order in restaurant_orders {
                println!("User: {}", order.username);
                println!("Items:");
                for item in &order.items {
                    println!("* {} (x{})", item.name, item.quantity);
                }
                println!("Total Price: {}", order.total_price);
                println!("Date: {}", order.datetime);
                println!("****");
            }
        }
    }

    if !found_orders {
        println!("{}", "No orders found for your restaurants.".yellow());
    }
}


fn view_sales_history(owner: &str) {

    let restaurants = match load_restaurants() {
        Ok(restaurants) => restaurants,
        Err(e) => {
            println!("{}: {}", "Error loading restaurants".red(), e);
            return;
        }
    };

    let owner_restaurants: Vec<&Restaurant> = restaurants.iter().filter(|r| r.owner == owner).collect();
    if owner_restaurants.is_empty() {
        println!("{}", "You don't own any restaurants.".yellow());
        return;
    }

    let orders = match load_orders() {
        Ok(orders) => orders,
        Err(e) => {
            println!("{}: {}", "Error loading orders".red(), e);
            return;
        }
    };

    let mut total_revenue = 0;
    let mut found_orders = false;
    for restaurant in owner_restaurants {
        let restaurant_orders: Vec<&Order> = orders.iter().filter(|o| o.restaurant == restaurant.name).collect();
        if !restaurant_orders.is_empty() {
            found_orders = true;
            println!("{}", format!("Sales for {}:", restaurant.name).green());
            let mut restaurant_revenue = 0;
            for order in restaurant_orders {
                restaurant_revenue += order.total_price;
                println!("Order by {}:", order.username);
                println!("Items:");
                for item in &order.items {
                    println!("* {} (x{})", item.name, item.quantity);
                }
                println!("Total: {}", order.total_price);
                println!("Date: {}", order.datetime);
                println!("****");
            }
            println!("Total revenue for {}: {}", restaurant.name, restaurant_revenue);
            total_revenue += restaurant_revenue;
        }
    }

    if !found_orders {
        println!("{}", "No sales found for your restaurants.".yellow());
    } else {
        println!("{}", format!("Total Revenue Across All Restaurants: {}", total_revenue).cyan());
    }
}


fn restaurant_owner_menu(user: &User) {

    loop {
        println!("{}", "\nRestaurant Owner Menu:".cyan());
        println!("{}", "1. Register Restaurant".blue());
        println!("{}", "2. Add/Edit Food".blue());
        println!("{}", "3. View Orders".blue());
        println!("{}", "4. View Sales History".blue());
        println!("{}", "5. Logout".red());

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read choise");

        match choice.trim() {
            "1" => add_restaurant(&user.username),
            "2" => manage_menu(&user.username),
            "3" => view_restaurant_orders(&user.username),
            "4" => view_sales_history(&user.username),
            "5" => break,
            _ => println!("{}", "Wrong please try again!".red()),
        }
    }
}


// admin توابع
fn view_users() {
    let users = match load_users() {
        Ok(users) => users,
        Err(e) => {
            println!("{}: {}", "Error loading users".red(), e);
            return;
        }
    };

    if users.is_empty() {
        println!("{}", "No users found.".yellow());
        return;
    }

    println!("{}", "Users:".green());
    for user in users {
        println!("- {} (Role: {})", user.username, user.role);
    }
}


fn manage_users() {
    let mut users = match load_users() {
        Ok(users) => users,
        Err(e) => {
            println!("{}: {}", "Error loading users".red(), e);
            return;
        }
    };

    if users.is_empty() {
        println!("{}", "No users found.".yellow());
        return;
    }

    println!("{}", "Users:".cyan());
    for (i, user) in users.iter().enumerate() {
        println!("{}. {} (Role: {})", i + 1, user.username, user.role);
    }

    println!("Enter user number to delete (0 to cancel):");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read choise");
    let index: usize = match choice.trim().parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "Invalid input! Please enter a number.".red());
            return;
        }
    };

    if index == 0 {
        return;
    }

    if index > users.len() {
        println!("{}", "Invalid user selection!".red());
        return;
    }

    let removed_user = users.remove(index - 1);
    
    if let Err(e) = save_users(&users) {
        println!("{}: {}", "Error saving users".red(), e);
        return;
    }
    println!("{}: {}", "User deleted".red(), removed_user.username);
}


fn view_restaurants_admin() {
    let restaurants = match load_restaurants() {
        Ok(restaurants) => restaurants,
        Err(e) => {
            println!("{}: {}", "Error loading restaurants".red(), e);
            return;
        }
    };

    if restaurants.is_empty() {
        println!("{}", "No restaurants found.".yellow());
        return;
    }

    println!("{}", "Restaurants:".green());
    for restaurant in restaurants {
        println!("* {} (Owner: {}, Category: {})", restaurant.name, restaurant.owner, restaurant.category);
    }
}


fn manage_restaurants() {
    let mut restaurants = match load_restaurants() {
        Ok(restaurants) => restaurants,
        Err(e) => {
            println!("{}: {}", "Error loading restaurants".red(), e);
            return;
        }
    };

    if restaurants.is_empty() {
        println!("{}", "No restaurants found.".yellow());
        return;
    }

    println!("{}", "Restaurants:".cyan());
    for (i, restaurant) in restaurants.iter().enumerate() {
        println!("{}. {} (Owner: {})", i + 1, restaurant.name, restaurant.owner);
    }

    println!("Enter restaurant number to delete (0 to cancel):");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let index: usize = match choice.trim().parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "Invalid input! Please enter a number.".red());
            return;
        }
    };

    if index == 0 {
        return;
    }

    if index > restaurants.len() {
        println!("{}", "Invalid restaurant selection!".red());
        return;
    }

    let removed_restaurant = restaurants.remove(index - 1);
    
    if let Err(e) = save_restaurants(&restaurants) {
        println!("{}: {}", "Error saving restaurants".red(), e);
        return;
    }
    println!("{}: {}", "Restaurant deleted".red(), removed_restaurant.name);
}


fn view_all_orders() {
    let orders = match load_orders() {
        Ok(orders) => orders,
        Err(e) => {
            println!("{}: {}", "Error loading orders".red(), e);
            return;
        }
    };

    if orders.is_empty() {
        println!("{}", "No orders found.".yellow());
        return;
    }

    println!("{}", "All Orders:".green());
    for order in orders {
        println!("User: {}", order.username);
        println!("Restaurant: {}", order.restaurant);
        println!("Items:");
        for item in &order.items {
            println!("* {} (x{})", item.name, item.quantity);
        }
        println!("Total Price: {}", order.total_price);
        println!("Date: {}", order.datetime);
        println!("****");
    }
}


fn admin_menu(_user: &User) {
    loop {
        println!("{}", "\nAdmin Menu:".cyan());
        println!("{}", "1. View User List".blue());
        println!("{}", "2. Manage Users".blue());
        println!("{}", "3. View Restaurant List".blue());
        println!("{}", "4. Manage Restaurants".blue());
        println!("{}", "5. View All Orders".blue());
        println!("{}", "6. Logout".red());

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read choice");

        match choice.trim() {
            "1" => view_users(),
            "2" => manage_users(),
            "3" => view_restaurants_admin(),
            "4" => manage_restaurants(),
            "5" => view_all_orders(),
            "6" => break,
            _ => println!("{}", "Wrong please try again!".red()),
        }
    }
}


fn main() {
    println!();
    println!(
        "                        ~~~~~~~~~~~~~~~~~~~~~~~~>{}<~~~~~~~~~~~~~~~~~~~~~~~~",
        "Welcome to SnapFood!".white().on_blue().bold()
    );
    
    if !Path::new("data.json").exists() {
        fs::write("data.json", "[]").expect("Failed to create data.json");
    }
    if !Path::new("restaurants.json").exists() {
        fs::write("restaurants.json", "[]").expect("Failed to create restaurants.json");
    }
    if !Path::new("orders.json").exists() {
        fs::write("orders.json", "[]").expect("Failed to create orders.json");
    }
    loop {
        println!("{}", "\n1. Register".blue());
        println!("{}", "2. Login".blue());
        println!("{}", "3. Exit".red());

        let mut my_str = String::new();
        io::stdin().read_line(&mut my_str).expect("Failed to read input");

        match my_str.trim() {
            "1" => register(),
            
            "2" => match login() {
                Some(user) => {
                    match user.role.as_str() {
                        "user" => user_menu(&user),
                        "restaurant_owner" => restaurant_owner_menu(&user),
                        "admin" => admin_menu(&user),
                        _ => println!("{}", "Invalid role!".red()),
                    }
                }
                None => {
                    println!("{}", "Login failed. Incorrect username or password.".red());
                }
            },

            "3" => {
                println!(
                    "                          ~~~~~~~~~~~~~~~~~~~~~~~~~~~>{}<~~~~~~~~~~~~~~~~~~~~~~~~~~~",
                    "Have good time!".white().on_blue().bold()
                );
                break;
            }

            _ => println!("{}", "Wrong please try again!".red().bold()),
        }
    }
}
