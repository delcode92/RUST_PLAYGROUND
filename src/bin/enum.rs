enum SomeEvent{
    OnClick(bool),
    OnPause(bool),
}

// enum variant that has no type
enum ThingsInSky{
    Moon,
    Sun
}


// filter based on value range,
// then return the enum variant
fn create_skystate(time:i32) -> ThingsInSky{
    match time {
        0..=18 => ThingsInSky::Sun,
        _ => ThingsInSky::Moon
    }
}

// enum variant will check/filter on this function then, print the result
fn check_skystate(state:ThingsInSky){
    match state {
        ThingsInSky::Sun => println!("time for the sun"),
        ThingsInSky::Moon => println!("time for the moon")
    }
}

fn main() {
 
    // sample 1
    let click = SomeEvent::OnClick(true);
    let pause = SomeEvent::OnPause(false);

    let event:SomeEvent = click;

    match event {
        SomeEvent::OnClick(s) => println!("click status {}", s),
        SomeEvent::OnPause(s) => println!("pause status {}", s),
    }

    
    // sample 2
    let sky_state = create_skystate(5);
    check_skystate(sky_state);
}
