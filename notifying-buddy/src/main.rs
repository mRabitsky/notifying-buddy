use lib_notifier;

fn main() {
    lib_notifier::alert("Title", "Body message goes here");
    match lib_notifier::schedule("Scheduled Message", "Today will be an acceptable day.", "now") {
        Ok(_) => println!("scheduled"),
        Err(e) => eprintln!("Error: {}", e),
    };
}
