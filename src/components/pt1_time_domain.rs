use input_rs::yew::Input;
use ndarray::{Array, Ix1};
use plotly::{
    Layout, Scatter,
    color::NamedColor,
    layout::{Axis, Shape, ShapeLine, ShapeType},
};
use yew::prelude::*;
use yew_plotly::Plotly;
use yew_plotly::plotly::Plot;
use yew_plotly::plotly::common::{DashType, Mode};

use yew_accordion::{Accordion, AccordionButton, AccordionItem};

use control_box::pt1::PT1;

#[derive(Properties, PartialEq)]
pub struct PlotlyPT1Props {
    #[prop_or(10_u32)]
    t1: u32, // T1 in milliseconds
    #[prop_or(1_u32)]
    ts: u32, // Sampling time in milliseconds
    #[prop_or(1.0_f32)]
    kp: f32, // Amplification
}

#[function_component(PlotlyPT1)]
pub fn plotly_pt1(PlotlyPT1Props { t1, ts, kp }: &PlotlyPT1Props) -> Html {
    fn validate_sample_time(sample_time: String) -> bool {
        let st: f32 = sample_time.parse::<f32>().unwrap_or(-1.0);
        st > 0.0
    }

    fn validate_t1(t1: String) -> bool {
        let st: f32 = t1.parse::<f32>().unwrap_or(-1.0);
        st > 0.0
    }

    fn validate_kp(kp: String) -> bool {
        let st: f32 = kp.parse::<f32>().unwrap_or(-1.0);
        st > 0.0
    }

    let sample_time_ref = use_node_ref();
    let sample_time_handle = use_state(String::default);
    let sample_time_valid_handle = use_state(|| true);
    let sample_time_value = (*sample_time_handle).clone();

    let t1_ref = use_node_ref();
    let t1_handle = use_state(String::default);
    let t1_valid_handle = use_state(|| true);
    let t1_value = (*t1_handle).clone();

    let kp_ref = use_node_ref();
    let kp_handle = use_state(String::default);
    let kp_valid_handle = use_state(|| true);
    let kp_value = (*kp_handle).clone();

    let t: Array<f64, Ix1> = Array::range(-5.0f64 * (*ts as f64), (5 * t1).into(), (*ts).into());
    // Step function
    let u: Array<f64, Ix1> = t.iter().map(|v| if *v > 0. { 1. } else { 0. }).collect();

    // PT1 response
    let mut pt1 = PT1::<f64>::new(*ts as f32 * 1000.0, *t1 as f32 * 1000.0, *kp);
    let y: Array<f64, Ix1> = u.iter().map(|v| pt1.transfer(*v)).collect();

    let mut plot = Plot::new();
    let trace = Scatter::from_array(t.clone(), u)
        .mode(Mode::LinesMarkers)
        .show_legend(true)
        .name("Step function stimulus");
    plot.add_trace(trace);
    let trace = Scatter::from_array(t, y)
        .mode(Mode::LinesMarkers)
        .show_legend(true)
        .name("PT1 Response");
    plot.add_trace(trace);

    let mut layout = Layout::new()
        .title("<b>PT1 Element</b>".into())
        .x_axis(Axis::new().title("time [ms]".into()));
    // Gradient line
    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Line)
            .x0(0.0)
            .y0(0.0)
            .x1(*t1)
            .y1(*kp)
            .line(
                ShapeLine::new()
                    .color(NamedColor::LightSeaGreen)
                    .width(3.)
                    .dash(DashType::DashDot),
            ),
    );
    plot.set_layout(layout);

    html! {
        <Accordion
        expanded_element={html! {<AccordionButton class={"bg-blue-500 text-white p-2 rounded"}>{ "PT1 Response ts: " }{ts}{"ms T1: "} {t1} </AccordionButton>}}
        collapsed_element={html! {<AccordionButton class={"bg-green-500 text-white p-2 rounded"}>{ "PT1 Response" }</AccordionButton>}}

        aria_controls="example-accordion"
        container_class="my-custom-class bg-gray-800 p-4 rounded border border-gray-400"
        expanded_element_class="my-expanded-class bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
        collapsed_element_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
        content_container_class="my-content-class bg-gray-500 p-4 rounded border-t border-gray-700"
    >
        <ul>
        <AccordionItem
            item_class="my-list-item-class border-b p-2 "
        >
        <form>
            <Input
                r#type="number"
                min="1"
                name="sample_time"
                r#ref={sample_time_ref}
                handle={sample_time_handle}
                valid_handle={sample_time_valid_handle}
                validate_function={validate_sample_time}
                placeholder="1"
                label="Sample time [ms]"
                required={true}
                error_message="Must be a positive number"
                class="form-field"
                label_class="block text-sm text-gray-300 mb-2"
                input_class="w-full p-2 border border-gray-600 rounded text-gray-100"
                error_class="text-red-800"
            />
            <Input
                r#type="number"
                name="t1"
                min="1"
                r#ref={t1_ref}
                handle={t1_handle}
                valid_handle={t1_valid_handle}
                validate_function={validate_t1}
                placeholder="1"
                label="T1 time [ms]"
                required={true}
                error_message="Must be a positive number"
                class="form-field"
                label_class="block text-sm text-gray-300 mb-2"
                input_class="w-full p-2 border border-gray-600 rounded text-gray-100"
                error_class="error-text"
            />
            <Input
                r#type="number"
                name="kp"
                r#ref={kp_ref}
                handle={kp_handle}
                valid_handle={kp_valid_handle}
                validate_function={validate_kp}
                placeholder="1"
                label="Amplification Kp"
                required={true}
                error_message="Must be a positive number"
                class="form-field"
                label_class="block text-sm text-gray-300 mb-2"
                input_class="w-full p-2 border border-gray-600 rounded text-gray-100"
                error_class="error-text"
            />
        </form>
        <p>{"name: "}<h5>{sample_time_value.clone()}  {t1_value.clone()} {kp_value.clone()}</h5></p>

        </AccordionItem>
            <AccordionItem
                item_class="my-list-item-class border-b p-2 hover:bg-gray-700 transition duration-300 ease-in-out"
            >
                <Plotly plot={plot}/>
            </AccordionItem>
        </ul>
        </Accordion>
     }
}
