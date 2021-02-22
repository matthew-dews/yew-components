#![allow(unused_variables)]
#![allow(dead_code)]

use crate::list::request_selected::request_selected_listener;
use crate::list::{GraphicType, RequestSelectedDetail};
use crate::to_option;
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use yew::prelude::*;


/// The `check-list-item` component
///
/// [Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/list#checklist)
pub struct CheckListItem {
    props: CheckListItemProps,
    node_ref: NodeRef,
    request_selected_listener: Option<EventListener>,
}

/// Props for [`CheckListItem`]
///
/// Documentation for [properties](https://github.com/material-components/material-components-web-components/tree/master/packages/list#mwc-check-list-item)
/// and [events](https://github.com/material-components/material-components-web-components/tree/master/packages/list#mwc-check-list-item-1)
#[derive(Debug, Properties, Clone)]
pub struct CheckListItemProps {
    #[prop_or_default]
    pub left: bool,
    #[prop_or(GraphicType::Control)]
    pub graphic: GraphicType,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub on_request_selected: Callback<RequestSelectedDetail>,
    pub children: Children,
}

impl Component for CheckListItem {
    type Message = ();
    type Properties = CheckListItemProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        // CheckListItem::ensure_loaded();
        // Self {
        //     props,
        //     node_ref: NodeRef::default(),
        //     request_selected_listener: None,
        // }
        unimplemented!()
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        //         left?=to_option(self.props.left)
        //         graphic=self.props.graphic.to_string()
        //         disabled=self.props.disabled
        //         ref=self.node_ref.clone()
        //     >{ self.props.children.clone() }
        
        unimplemented!()
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.request_selected_listener = Some(request_selected_listener(
                &self.node_ref,
                self.props.on_request_selected.clone(),
            ));
        }
    }
}
