mod helpers;
use helpers::*;
use vizia::{prelude::*, vg::svg::Ellipse};

#[derive(Debug, Lens)]
pub struct AppData {
    slider_val: f32,
    label: String,
}

pub enum AppEvent {
    SetValue(f32),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetValue(val) => {
                self.slider_val = *val;
                self.label = 
                    // TODO: compare scaling with dspMixFx
                    if *val == 0.0 {    
                        String::from("∞")
                    } else if *val < 103.0 {
                        // scale from -74dB to 0dB
                        format!("{:.2}", -75.0 + (*val * 74.0 / 102.0))
                    } else if *val >= 103.0 {
                        // scale from 0dB to +6dB
                        format!("{:.2}", (*val - 103.0) * 6.0 / 24.0 )
                    } else {
                        panic!("Unexpected slider value!")
                    }
            }
        });
    }
}

pub fn run_gui() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        AppData { slider_val: 0.0, label: String::from("∞")}.build(cx);

        ExamplePage::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Slider::new(cx, AppData::slider_val)
                    .range(0.0..127.0)
                    .step(1.0)
                    .on_changing(move |cx, val| cx.emit(AppEvent::SetValue(val)))
                    .class("vertical");
                Label::new(cx, AppData::label)
                    .child_space(Stretch(1.0))
                    .width(Pixels(50.0));
            })
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0))
            .row_between(Pixels(8.0));
        });
    })
    .title("Slider")
    .run()
}
