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
    // hosting::error_test::error();
    // hosting::trait_test::get_max_number();
    // hosting::trait_test::struct_def();
    // hosting::trait_test::trait_test();

    // hosting::lifecycle_test::longest_test();
    // hosting::lifecycle_test::it();

    // hosting::closure_test::closure();
    // hosting::closure_test::closure2();
    // hosting::closure_test::closure3();

    // hosting::iterator_test::filters_by_size();
    // hosting::iterator_test::counter();

    // hosting::smartpointers_test::smart_pointers();
    // hosting::smartpointers_test::deref_trait();
    // hosting::smartpointers_test::force_deref_trait();
    // hosting::smartpointers_test::drop_trait();
    // hosting::smartpointers_test::rc_c();

    // hosting::thread_test::thread_t();
    // hosting::thread_test::thread_t_move();
    // hosting::thread_test::thread_t_channel();
    // hosting::thread_test::thread_t_multiple_thread_to_one_channel();
    // hosting::thread_test::thread_t_mutex();
    // hosting::thread_test::thread_t_mutex_arc();

    hosting::object_test::object_t_average();
    hosting::object_test::struct_trait_test();
    hosting::object_test::object_t_post();
}
