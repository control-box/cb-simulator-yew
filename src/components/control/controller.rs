use accordion_rs::yew::{Accordion, Item, List};
use accordion_rs::Size;
use log::{debug, info};
use yew::prelude::*;

use cb_controller::pid::{PidController, PidCoreBuilder, PidOutputLimit, PidSetpointRange};

use super::pid_core::PidControllerDialog;
use super::pid_output::PidControllerOutputDialog;
use super::pid_input::PidControllerInputDialog;
use super::pid_dead_band::PidControllerDeadBandDialog;

#[derive(Properties, PartialEq)]
pub struct AccordeonControllerProps {
    // pub controller: PidController<f64>,
    pub sampling_interval: f64,
    pub update: Callback<PidController<f64>>,

}

#[function_component(AccordeonController)]
pub fn accordeon_elements(props: &AccordeonControllerProps) -> Html {
    let expand = use_state(|| true);

    let core_builder: UseStateHandle<PidCoreBuilder<f64>> = use_state(
        || PidCoreBuilder::default().sampling_interval(props.sampling_interval as f32));
    let output_config: UseStateHandle<Option<PidOutputLimit<f64>>>   = use_state ( || None);
    let input_config: UseStateHandle<Option<PidSetpointRange<f64>>>   = use_state ( || None);
    let dead_band_config: UseStateHandle<Option<f64>>   = use_state ( || None);

    let on_core_update: Callback<PidCoreBuilder<f64>> = {
        let update = props.update.clone();
        let core_builder = core_builder.clone();
        let output_config = output_config.clone();
        Callback::from(move |builder: PidCoreBuilder<f64>| {
            info!("AccordeonController Core Builder Update to: {:?}", builder);

            core_builder.set(builder.clone());
            let mut pid = builder.build();

            match (*output_config).clone() {
                Some(c) => pid.set_output_limit(c),
                None => pid.reset_output_limit(),
            }
            debug!("Update on PID-Controller {:?}", pid);
            update.emit(pid);
        })
    };

    if props.sampling_interval != (*core_builder).dt as f64 {
        let mut cb = (*core_builder).clone();
        cb.dt = props.sampling_interval as f32;
        core_builder.clone().set(cb.clone());
        let pid = cb.build();
        debug!("Update on sampling interval of PID-Controller {:?}", props.sampling_interval);
        props.update.emit(pid);
    }

    let on_output_update: Callback<Option<PidOutputLimit<f64>>> = {
        let update = props.update.clone();
        let core_builder = core_builder.clone();
        let output_config = output_config.clone();
        Callback::from(move |config: Option<PidOutputLimit<f64>>| {
            info!("AccordeonController Output Limit Update to: {:?}", config);
            let mut pid = (*core_builder).clone().build();
            match config.clone() {
                Some(c) => pid.set_output_limit(c),
                None => pid.reset_output_limit(),
            }
            output_config.set(config);
            debug!("Update on PID-Controller {:?}", pid);
            update.emit(pid);
        })
    };

    let on_input_update: Callback<Option<PidSetpointRange<f64>>> = {
        let update = props.update.clone();
        let core_builder = core_builder.clone();
        let input_config = input_config.clone();
        Callback::from(move |config: Option<PidSetpointRange<f64>>| {
            info!("AccordeonController Input Range to: {:?}", config);
            let mut pid = (*core_builder).clone().build();
            match config.clone() {
                Some(c) => pid.set_setpoint_range(c),
                None => pid.reset_setpoint_range(),
            }
            input_config.set(config);
            debug!("Update on PID-Controller {:?}", pid);
            update.emit(pid);
        })
    };

    let on_dead_band_update: Callback<Option<f64>> = {
        let update = props.update.clone();
        let core_builder = core_builder.clone();
        let dead_band_config = dead_band_config.clone();
        Callback::from(move |config: Option<f64>| {
            info!("AccordeonController Dead Band Tolerance to: {:?}", config);
            let mut pid = (*core_builder).clone().build();
            match config.clone() {
                Some(c) => pid.set_dead_band_tolerance(c),
                None => pid.reset_dead_band_tolerance(),
            }
            dead_band_config.set(config);
            debug!("Update on PID-Controller {:?}", pid);
            update.emit(pid);
        })
    };

    html! {
        <Accordion
            expand={expand}
            expanded={html! { "PID Controller Parameter" }}
            collapsed={html! {<>
                 { "Set PID Controller Parameter" }
            </>}}
            size={Size::Custom("auto")}
            class=" p-4 rounded border border-gray-400 dark:border-gray-600"
            expanded_class=" bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
            collapsed_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
        >
            <List>
                <Item class="flex flex-wrap max-w-full">
                    <div class="flex flex-col w-32 pt-4">
                        <label class="block text-sm mb-2 form-fieldtext-gray-300 dark:text-gray-700" for="step_function_label"> { "Sampling Interval" } </label>
                        <div id="step_function_label" class="pt-2 text-lg"> { props.sampling_interval.to_string() } </div>
                    </div>
                    <PidControllerDialog  builder={(*core_builder).clone()} on_update={on_core_update} />
                    <PidControllerOutputDialog  config={(*output_config).clone()} on_update={on_output_update} />
                    <PidControllerInputDialog  config={(*input_config).clone()} on_update={on_input_update} />
                    <PidControllerDeadBandDialog config={(*dead_band_config).clone()} on_update={on_dead_band_update} />
                </Item>
            </List>
        </Accordion>

    }
}
