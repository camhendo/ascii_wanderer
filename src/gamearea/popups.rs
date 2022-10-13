use yew::{events::{KeyboardEvent, MouseEvent}, html::{self, IntoPropValue}, Component, Context, Html, NodeRef};
use web_sys::{Element, Node, HtmlElement, Document, DomRect};
// use wasm_bindgen::JsValue;
use gloo_utils::document;
use yew::prelude::*;
use yew_utils::components::table::Table;
use yew_utils::vdom::*;
use slab::Slab;
use gloo::console;


pub struct PopUp {
    pub container_ref: NodeRef,
}


#[derive(Clone, Properties, PartialEq)]
pub struct PopUpProps {
    pub thing: String,
    pub location: (f64, f64, f64, f64),
    pub bounding_rect: DomRect,
    pub destroy_popup: Callback<()>
}

impl PopUp {
    pub fn gen_popups(_: String) -> String { 

        "Temp Holder Value".to_owned()
    }
}

impl Component for PopUp {

    type Message = ();

    type Properties = PopUpProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            container_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let destroy_popup = ctx.props().destroy_popup.clone();

        
        html! {<>
                    <div ref={self.container_ref.clone()} class="popup-container">
                        <div class="popup-content">
                            <div id="popup-top">
                                <span id="popup-title">{"New NFT Found!"}</span>
                                <span id="popup-closer" onclick={Callback::from(move |_| destroy_popup.emit(()))}>{"X"}</span>
                            </div>
                            <p id="popup-message">{"HTML content for the popup"}</p>
                        </div>
                    </div> 
                </>}
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }
        let container_offset = self.container_ref.cast::<Element>(); 
        let container_rect = container_offset.clone().unwrap().get_bounding_client_rect(); 
        let loc = ctx.props().location.clone();
        let bounded = ctx.props().bounding_rect.clone();
        let cx = loc.0.clone() - container_rect.clone().width();
        let cy = loc.1.clone() - container_rect.clone().height();
        let top: String;
        let left: String;
        
        if cx >= bounded.x() && cy >= bounded.y() {
            top = ((cy).to_string()+"px;").to_string();
            left = ((cx).to_string()+"px;").to_string();
        } else if cx >= bounded.x() && cy < bounded.y() {
            top = ((loc.1.clone() + loc.3.clone()).to_string()+"px;").to_string();
            left = ((cx).to_string()+"px;").to_string();
        } else if cx < bounded.x() && cy >= bounded.y() {
            top = ((cy).to_string()+"px;").to_string();
            left = ((loc.0.clone() + loc.2.clone()).to_string()+"px;").to_string();
        } else {
            top = ((loc.1.clone() + loc.3.clone()).to_string()+"px;").to_string();
            left = ((loc.0.clone() + loc.2.clone()).to_string()+"px;").to_string();
        }

        let style = format!("top:{} left:{}",top,left);
        
        container_offset.unwrap().set_attribute("style", style.as_str()).unwrap();

    }
}