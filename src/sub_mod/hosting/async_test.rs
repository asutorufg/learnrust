use futures::executor::block_on;

async fn async_1() {
    println!("async1");
}

async fn async_2() {
    async_1().await;
    println!("async_2");
}

struct Song {
    name: String,
}

async fn learn_song() -> Song {
    Song {
        name: String::from("a Song"),
    }
}

async fn sing_song(song: Song) {
    println!("sing {}", song.name);
}
async fn dance() {
    println!("dance");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn learn_sing_and_dance() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}

pub fn async_test() {
    let future = async_2();

    block_on(future);

    block_on(learn_sing_and_dance())
}
