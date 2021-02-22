#![allow(unused_variables)]
#![allow(dead_code)]

use super::to_option;
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CustomEvent, Element};
use yew::prelude::*;


/// The `snackbar` component
///
/// [Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/slider)
pub struct Slider {
    props: SliderProps,
    node_ref: NodeRef,
    input_listener: Option<EventListener>,
    change_listener: Option<EventListener>,
}

/// Props for [`Slider`]
///
/// Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/slider#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/slider#events)
#[derive(Clone, PartialEq, Properties)]
pub struct SliderProps {
    #[prop_or(0)]
    pub value: u32,
    #[prop_or(0)]
    pub min: u32,
    #[prop_or(100)]
    pub max: u32,
    #[prop_or(0)]
    pub step: u32,
    #[prop_or(false)]
    pub pin: bool,
    #[prop_or(false)]
    pub markers: bool,
    /// Binds to input on `slider`
    /// Type passed to callback is `CustomEvent` because `Slider` is
    /// undocumented See: <https://github.com/material-components/material-components-web-components/issues/1848>
    #[prop_or_default]
    pub oninput: Callback<CustomEvent>,
    /// Binds to change on `slider`
    /// Type passed to callback is `CustomEvent` because `Slider` is
    /// undocumented See: <https://github.com/material-components/material-components-web-components/issues/1848>
    #[prop_or_default]
    pub onchange: Callback<CustomEvent>,
}

impl Component for Slider {
    type Message = ();
    type Properties = SliderProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        // Slider::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            input_listener: None,
            change_listener: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        //         value=self.props.value
        //         min=self.props.min
        //         max=self.props.max
        //         step=self.props.step
        //         pin?=to_option(self.props.pin)
        //         markers?=to_option(self.props.markers)
        //         ref=self.node_ref.clone()

        html! {
            <div class="mdc-slider">
                <input class="mdc-slider__input" type="range" min="0" max="100" value="50" name="volume" aria-label="Continuous slider demo"/>
                <div class="mdc-slider__track">
                    <div class="mdc-slider__track--inactive"></div>
                    <div class="mdc-slider__track--active">
                        <div class="mdc-slider__track--active_fill"></div>
                    </div>
                </div>
                <div class="mdc-slider__thumb">
                    <div class="mdc-slider__thumb-knob"></div>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        // if first_render {
        //     let element = self.node_ref.cast::<Element>().unwrap();

        //     let oninput = self.props.oninput.clone();
        //     self.input_listener = Some(EventListener::new(&element, "input", move |event| {
        //         oninput.emit(JsValue::from(event).unchecked_into::<CustomEvent>())
        //     }));

        //     let onchange = self.props.onchange.clone();
        //     self.change_listener = Some(EventListener::new(&element, "change", move |event| {
        //         onchange.emit(JsValue::from(event).unchecked_into::<CustomEvent>())
        //     }));
        // }
    }
}
