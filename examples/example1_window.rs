use hydra::{app::{App, EventHandler, Frame}, context::Context};
use winit::{event::ElementState, event_loop::EventLoopWindowTarget, keyboard::KeyCode::*};


struct State{
    pub x: i32,
}

fn init(_app: &App<State>,_context: &Context) -> State{
    println!("Creates state!");
    State {  x: 1}
}



fn update(state: &mut State,ctx: &Context){
    state.x +=1;
}

fn render(state: &State,ctx: &Context,frame: Frame){
    println!("value {}",state.x);
}

fn key_input(state: &mut State,key: hydra::app::Key,key_state: ElementState,control: &EventHandler){
    println!("key: {:#?}",key);
    match key{
        Escape => control.exit(),
        _ => {}
    }
}



fn main(){
    App::new(init)
    .update(update)
    .render(render)
    .on_key(key_input)
    .with_title("example1_window".to_string())
    .run();
}