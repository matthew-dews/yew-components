#![allow(unused_variables)]
#![allow(dead_code)]

use yew::prelude::*;

const SLOT: &str = "appContent";

/// Props for [`DrawerAppContent`]
#[derive(Properties, Clone)]
pub struct DrawerAppContentProps {
    pub children: Children,
}

/// Defines `appContent` for [`Drawer`][crate::Drawer].
///
/// If the child passed is an element (a `VTag`), then it is modified to include
/// the appropriate attributes. Otherwise, the child is wrapped in a `span`
/// containing said attributes.
pub struct DrawerAppContent {
    props: DrawerAppContentProps,
}

impl Component for DrawerAppContent {
    type Message = ();
    type Properties = DrawerAppContentProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        // let children = self
        //     .props
        //     .children
        //     .iter()
        //     .map(|child| {
        //         match child {
        //             Html::VTag(mut vtag) => {
        //                 vtag.add_attribute("slot", "appContent");
        //                 Html::VTag(vtag)
        //             }
        //             _ => {
        //                 html! {
        //                     <span slot=SLOT>
        //                         { child }
        //                     </span>
        //                 }
        //             }
        //         }
        //     })
        //     .collect::<Html>();

        // html! {
        //     { children }
        // }
        unimplemented!()
    }
}
