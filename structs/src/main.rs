struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//User构造器
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

//简化版
fn build_user_simple(email: String, username: String) -> User{
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("cookiemakefun"),
        email: String::from("cookie@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("helloworld@example.com");

    println!("user1: username:{},email:{}",user1.username,user1.email);
    
    let mut email2 = String::from("zhangsan@gmail.com");
    let mut name2 = String::from("zhangsan");

    //调用build_user
    let mut user2 = build_user(email2,name2);
    println!("user2: username:{},email:{}",user2.username,user2.email);



    let mut email3 = String::from("kunkun@gmail.com");
    let mut name3 = String::from("kunkun");

    //调用build_user_simple
    let mut user3 = build_user_simple(email3,name3);
    println!("user3: username:{},email:{}",user3.username,user3.email);



    //创建实例用另外一个实例
    //一般方法，较为繁琐
    let user1_copy = User {
        active: user1.active,
        username: user1.username,
        email: user1.email,
        sign_in_count: user1.sign_in_count,
    };

    //使用struct update syntax
    let user2_copy = User {
        //除了email的内容不一样，其他成员变量值都和user2一样
        email: String::from("newemail@example.com"),
        ..user2
    };
    
    // user2.username; //value used here after move   user2已经被move到user2_copy中，不能再调用了
    // user2.active;
    // user2.sign_in_count;   active和sign_in_count不会报错，因为这两种类型都是copy trait
}
