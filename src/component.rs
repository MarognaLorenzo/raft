pub struct Component<S> {
    _state: std::marker::PhantomData<S>,
}

pub struct Off;
pub struct On;
pub struct Broken;

impl Component<Off> {
    pub fn new() -> Self {
        Component{
            _state: std::marker::PhantomData
        }
    }

    pub fn turn_on(self) -> Component<On> {
        println!("Swithching on!");
        Component{
            _state : std::marker::PhantomData
        }
    }
}

impl Component<On> {
    pub fn turn_off(self) -> Component<Off> {
        println!("Swithching off!");
        Component{
            _state: std::marker::PhantomData
        }
    }

    pub fn drop(self) -> Component<Broken> {
        println!("Oh no your dropping me!");
        Component{
            _state: std::marker::PhantomData
        }
    }
}

impl Component<Broken> {
    pub fn lament (self) {
        println!("Oh no I am broken forever");
    }

}


