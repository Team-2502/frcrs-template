use frcrs::container;

pub struct Ferris;

// Use this function to declare subsystems, joysticks, or anything needed on startup
pub async fn configure() {

    // This will call the container function in a loop, by default the loop is capped to 500hz
    container!(container, Ferris {});
}

// Put your commands here
pub async fn container() {

}