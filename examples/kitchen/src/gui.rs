use std::hash::Hash;

use mctk_core::component::{self, Component, RootComponent};
use mctk_core::widgets::{Carousel, Div, SlideBar, SlideBarType};
use mctk_core::{lay, msg, rect, size, size_pct, Color};
use mctk_core::{node, node::Node};
use mctk_macros::{component, state_component_impl};

use crate::pages::buttons::Buttons;
use crate::pages::icon_buttons::IconButtons;
use crate::pages::radios::Radios;
use crate::pages::textboxes::TextBoxes;
use crate::pages::toggles::Toggles;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
enum Message {
    Button { name: String },
}

#[derive(Debug)]
pub struct KitchenState {
    counter: u64,
    velocity: f64,
    start: Option<Instant>,
}

#[component(State = "KitchenState")]
#[derive(Debug, Default)]
pub struct Kitchen {}

#[derive(Debug, Clone)]
pub struct KitchenParams {}

#[state_component_impl(KitchenState)]
impl Component for Kitchen {
    fn render_hash(&self, hasher: &mut component::ComponentHasher) {
        // println!("Kitchen::render_hash()");
        self.state_ref().counter.hash(hasher);
    }

    fn init(&mut self) {
        self.state = Some(KitchenState {
            counter: 0,
            start: None,
            velocity: 0.,
        });
    }

    // fn on_tick(&mut self, _event: &mut mctk_core::event::Event<mctk_core::event::Tick>) {
    //     let start = self.state_ref().start;
    //     if start.is_none() {
    //         self.state_mut().start = Some(Instant::now());
    //         return;
    //     }

    //     // println!("time elapsed: {:?}", Instant::now().duration_since(start.unwrap()));

    //     if Instant::now().duration_since(start.unwrap()) >= Duration::from_secs(1) {
    //         // println!("1 second completed");
    //         // println!("counter {:?}", self.state_ref().counter);
    //         // std::process::exit(0);
    //         return;
    //     }

    //     let mut counter = self.state_ref().counter;

    //     // increment counter
    //     counter += 1;
    //     self.state_mut().counter = counter;
    // }

    fn view(&self) -> Option<Node> {
        let buttons = node!(Buttons {}, lay![size_pct: [100]]);
        let textboxes = node!(TextBoxes {}, lay![size_pct: [100]]);
        let radios = node!(Radios {}, lay![size_pct: [100]]);
        let icon_buttons = node!(IconButtons {}, lay![size_pct: [100]]);
        let toggles = node!(Toggles {}, lay![size_pct: [100]]);
        let slide_bar = node!(
            SlideBar::new()
                .value(50)
                .slider_type(SlideBarType::Box)
                .active_color(Color::rgb(15., 168., 255.))
                // .on_slide(Box::new(|value| msg!(gui::Message::SliderChanged(
                //     gui::SliderSettingsNames::Brightness { value }
                // ))))
                .col_spacing(7.75)
                .row_spacing(7.75)
                .col_width(4.),
            lay![size: [480, 46], margin:[10., 0., 0., 0.]]
        );
        Some(slide_bar)

        // // println!("counter is {:?}", self.state_ref().counter);
        // // let start = 0.;
        // // let end = 440.;
        // // let control1 = 0.28;
        // // let control2 = 0.49;
        // let counter = self.state_ref().counter;

        // // println!("{:?}", t);

        // // let start_coord = (0.28, 0.03);
        // // let end_coord = (0.49, 1.05);

        // // let p = start_coord.0 * (1.0 - t).powi(3)
        // // + start_coord.1 * t * (1.0 - t).powi(2) * 3.0
        // // + end_coord.0 * t.powi(2) * (1.0 - t) * 3.0
        // // + end_coord.1 * t.powi(3);

        // // println!("{:?}", p);

        // // let p = easing::cubic_inout(0., 440., 60).nth(self.state_ref().counter as usize).unwrap();

        // let translate_x: [f64; 60] = [
        //     0.,
        //     4.918703703703703,
        //     9.682962962962963,
        //     14.305,
        //     18.797037037037036,
        //     23.17129629629629,
        //     27.44,
        //     31.615370370370364,
        //     35.70962962962963,
        //     39.735,
        //     43.703703703703695,
        //     47.62796296296297,
        //     51.519999999999996,
        //     55.39203703703703,
        //     59.25629629629629,
        //     63.124999999999986,
        //     67.01037037037038,
        //     70.92462962962964,
        //     74.88,
        //     78.8887037037037,
        //     82.96296296296296,
        //     87.11499999999998,
        //     91.35703703703702,
        //     95.70129629629628,
        //     100.16,
        //     104.74537037037037,
        //     109.46962962962962,
        //     114.34499999999998,
        //     119.3837037037037,
        //     124.59796296296295,
        //     129.99999999999997,
        //     135.602037037037,
        //     141.41629629629628,
        //     147.455,
        //     153.7303703703704,
        //     160.2546296296297,
        //     167.0400000000001,
        //     174.0987037037038,
        //     181.4429629629631,
        //     189.08500000000015,
        //     197.03703703703724,
        //     205.3112962962965,
        //     213.92000000000027,
        //     222.87537037037066,
        //     232.18962962963002,
        //     241.8750000000004,
        //     251.94370370370416,
        //     262.40796296296344,
        //     273.28000000000054,
        //     284.57203703703766,
        //     296.296296296297,
        //     308.4650000000007,
        //     321.0903703703712,
        //     334.1846296296305,
        //     347.76000000000096,
        //     361.82870370370466,
        //     376.402962962964,
        //     391.49500000000126,
        //     407.11703703703836,
        //     435.28129629629757,
        // ];

        // let translate_y: [f64; 60] = [
        //     0.,
        //     0.3625925925925926,
        //     1.4340740740740738,
        //     3.19,
        //     5.605925925925925,
        //     8.657407407407407,
        //     12.319999999999999,
        //     16.569259259259255,
        //     21.38074074074074,
        //     26.729999999999997,
        //     32.59259259259259,
        //     38.94407407407407,
        //     45.75999999999999,
        //     53.01592592592592,
        //     60.687407407407406,
        //     68.74999999999997,
        //     77.17925925925927,
        //     85.95074074074073,
        //     95.03999999999999,
        //     104.42259259259258,
        //     114.07407407407408,
        //     123.97,
        //     134.0859259259259,
        //     144.3974074074074,
        //     154.87999999999997,
        //     165.50925925925924,
        //     176.26074074074072,
        //     187.10999999999999,
        //     198.03259259259258,
        //     209.00407407407405,
        //     219.99999999999994,
        //     230.99592592592592,
        //     241.9674074074074,
        //     252.89000000000004,
        //     263.7392592592593,
        //     274.4907407407409,
        //     285.1200000000002,
        //     295.6025925925927,
        //     305.9140740740743,
        //     316.03000000000026,
        //     325.9259259259261,
        //     335.5774074074077,
        //     344.96000000000026,
        //     354.04925925925954,
        //     362.82074074074103,
        //     371.2500000000003,
        //     379.312592592593,
        //     386.98407407407444,
        //     394.2400000000003,
        //     401.05592592592626,
        //     407.4074074074078,
        //     413.2700000000003,
        //     418.61925925925954,
        //     423.43074074074104,
        //     427.6800000000003,
        //     431.34259259259284,
        //     434.39407407407424,
        //     436.8100000000002,
        //     438.5659259259261,
        //     439.63740740740747,
        // ];
        // Some(
        //     node!(Div::new().bg(Color::BLUE), lay![ size_pct: [100]],)
        //         .push(node!(
        //             Div::new().bg(Color::GREEN),
        //             lay![size: [40], margin: [0., 440., 0., 0. ]]
        //         ))
        //         .push(
        //             node!(
        //                 Div::new().bg(Color::YELLOW),
        //                 lay![
        //                 size: [40],
        //                 position_type: PositionType::Absolute,
        //                 position: [0., translate_x[(counter as usize).min(59)], 0., 0.]
        //                 ],
        //             )
        //             .key(self.state_ref().counter),
        //         ),
        // )
    }
}

impl RootComponent<KitchenParams> for Kitchen {}
