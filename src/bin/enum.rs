enum SomeEvent{
    OnClick(bool),
    OnPause(bool),
}

fn main() {
    
    let click = SomeEvent::OnClick(true);
    let pause = SomeEvent::OnPause(false);

    let event:SomeEvent = click;

    match event {
        SomeEvent::OnClick(s) => println!("click status {}", s),
        SomeEvent::OnPause(s) => println!("pause status {}", s),
    }
}
