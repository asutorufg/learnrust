mod sub_mod;

pub use sub_mod::hosting;
pub use sub_mod::hosting::test as first_test;

pub fn sub_mod_func() {
    hosting::hosting_func();
}

fn main() {
    // sub_mod_func();
    // first_test::test();
    // first_test::test2();
    // hosting::ip_addr::test10();
    // hosting::test::test3(3, "tes3".to_string());
    // hosting::ip_addr::test11(hosting::ip_addr::IpAddrKind::V4(String::from("127.0.0.1")));
    // hosting::ip_addr::test11(hosting::ip_addr::IpAddrKind::V6(String::from("::ff")));

    // hosting::vector_test::vct();
    // hosting::string_test::string();
    // hosting::hashmap_test::hashmap();
    hosting::error_test::error();
}
