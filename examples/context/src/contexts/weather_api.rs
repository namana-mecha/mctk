use lazy_static::lazy_static;
use mctk_core::context::Context;

lazy_static! {
    pub static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    pub static ref WEATHER_API: WeatherAPI = WeatherAPI {
        temperature: Context::new(0.0),
        is_loading: Context::new(false)
    };
}

pub struct WeatherAPI {
    pub temperature: Context<f32>,
    pub is_loading: Context<bool>,
}

impl WeatherAPI {
    pub fn get() -> &'static Self {
        &WEATHER_API
    }

    pub fn fetch() {
        RUNTIME.spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                WeatherAPI::get()
                    .temperature
                    .set(rand::random::<f32>() * 100.0);
            }
        });
    }
}
