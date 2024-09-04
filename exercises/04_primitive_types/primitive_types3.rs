fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a = "112212312313123123123123112212312313123123123123112212312313123123123123112212312313123123123123112212312313123123123123112212312313123123123123";

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
