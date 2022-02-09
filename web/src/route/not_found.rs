use gloo_timers::future::TimeoutFuture;
use yew::prelude::*;
use super::{Route, Redirect};

pub struct NotFound {
    count_down: u8,
}

impl Component for NotFound {
    type Message = ();
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(TimeoutFuture::new(1000));
        
        Self {
            count_down: 10,
        }
    }
    
    fn update(&mut self, ctx: &Context<Self>, _: Self::Message) -> bool {
        self.count_down -= 1;
        
        if self.count_down != 0 {
            ctx.link().send_future(TimeoutFuture::new(1000));
        }
        
        true
    }
    
    fn view(&self, _: &Context<Self>) -> Html {
        if self.count_down == 0 {
            return html! { 
                <Redirect to={Route::Index}/> 
            }
        }
                
        html!{
            <div class="section is-medium has-text-centered">
                <h1 class="title">{ "404 Not Found" }</h1>
                <h2 class="subtitle">{ format!("Redirect to / in {}s", self.count_down) }</h2>
            </div>
        }
    }
}