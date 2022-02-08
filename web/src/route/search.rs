use yew::prelude::*;
use common::GetItemResponse;

type Msg = Result<Vec<GetItemResponse>, String>;

pub struct Search {
    data: Option<GetItemResponse>,
}

impl Component for Search {
    type Message = Msg;
    type Component = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        ctx.send_future(async {
            // get search data from the server
            Ok(SearchReponse {
                data: vec!["test1".into(), "test2".into()],
                count: 2,
            })
        });
        
        Self {
            data: None
        }
    }
    
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Ok(resp) => {
                self.data = resp;
                true
            }
            Err(why) => {
                log::error!("Cannot get search data\n{:#?}", why);
                false
            }
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.data {
            None => html! {
                <h2> { "Loading" } </h2>
            }
            
            Some(data) => {
                
            }
        }
    }
}