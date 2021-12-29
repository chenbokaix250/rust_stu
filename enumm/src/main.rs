enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message{
    fn call(&self){}
}

fn main(){
    let g = Message::Quit;
    g.call();
}