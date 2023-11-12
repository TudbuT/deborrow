use deborrow::AsReference;
use std::{thread, time::Duration};

// this is an example of a loading-bar-ish using deborrow's reference
// type to share a ref between threads.
//
// this is having worker threads writing to set regions of a buffer and
// one reader thread displaying it independently. reading while writing
// isn't a problem there due to the different regions being written to.
// when rendered, it'd cause there to be a cut somewhere on the screen,
// but even this can be fixed by having a front and back buffer and
// syncing the render to the swapping of the two.
fn main() {
    let mut done = [false; 5];
    unsafe {
        let done_ref = done.as_deborrowed_mut_reference();
        thread::spawn(move || {
            for i in 0..done_ref.as_ref().len() {
                thread::spawn(move || {
                    // do things
                    done_ref.as_mut()[i] = true;
                    println!("Thread {i} finished.");
                });
            }
        });
    }
    while done.iter().any(|x| !*x) {
        print_loading_bar(&done);
        thread::sleep(Duration::from_micros(100));
    }
    print_loading_bar(&done);
}

fn print_loading_bar(done: &[bool; 5]) {
    println!(
        "{} things done so far.",
        done.iter().filter(|x| **x).count()
    );
}
