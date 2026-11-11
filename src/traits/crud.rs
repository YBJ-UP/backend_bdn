pub trait Get {
    fn get_many();
    fn get_one();
}

pub trait Post {
    fn post_many();
    fn post_one();
}

pub trait Update {
    fn update_many();
    fn update_one();
}

pub trait Delete {
    fn delete_many();
    fn delete_one();
}
