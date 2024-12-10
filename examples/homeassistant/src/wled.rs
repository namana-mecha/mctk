use lazy_static::lazy_static;
use mctk_core::context::Context;
use mctk_macros::Model;
use reqwest;
use serde_json::json;
use std::error::Error;

lazy_static! {
    pub static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    pub static ref WLED: Wled = Wled {
        r: Context::new(0.0),
        g: Context::new(0.0),
        b: Context::new(0.0),
        brightness: Context::new(0.0),
        is_on: Context::new(false),
    };
}

const IP_ADDRESS: &str = "192.168.150.7";

/// Represents a WLED device controller
struct WledController {
    base_url: String,
    client: reqwest::Client,
}

impl WledController {
    /// Create a new WLED controller
    ///
    /// # Arguments
    ///
    /// * `ip_address` - The IP address of the WLED device
    fn new(ip_address: &str) -> Self {
        WledController {
            base_url: format!("http://{}/json", ip_address),
            client: reqwest::Client::new(),
        }
    }

    /// Toggle the power state of the WLED device
    ///
    /// # Arguments
    ///
    /// * `state` - Optional boolean to set specific power state
    async fn toggle_power(&self, state: Option<bool>) -> Result<(), Box<dyn Error>> {
        let payload = match state {
            Some(true) => json!({"on": true}),
            Some(false) => json!({"on": false}),
            None => json!({"off": false}), // Toggle current state
        };

        self.send_state_request(payload).await
    }

    /// Set the color and brightness of the WLED device
    ///
    /// # Arguments
    ///
    /// * `red` - Red color value (0-255)
    /// * `green` - Green color value (0-255)
    /// * `blue` - Blue color value (0-255)
    /// * `brightness` - Brightness value (0-255)
    async fn set_color(
        &self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: Option<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let payload = json!({
            "on": true,
            "bri": brightness.unwrap_or(255),
            "seg": [{
                "col": [[red, green, blue]]
            }]
        });

        self.send_state_request(payload).await
    }

    /// Activate a specific WLED preset
    ///
    /// # Arguments
    ///
    /// * `preset_id` - ID of the preset to activate
    async fn set_preset(&self, preset_id: u8) -> Result<(), Box<dyn Error>> {
        let payload = json!({"ps": preset_id});

        self.send_state_request(payload).await
    }

    /// Internal method to send state request to WLED device
    async fn send_state_request(&self, payload: serde_json::Value) -> Result<(), Box<dyn Error>> {
        let response = self
            .client
            .post(&format!("{}/state", self.base_url))
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Error: HTTP {}", response.status()).into())
        }
    }
}

#[derive(Model)]
pub struct Wled {
    pub r: Context<f32>,
    pub g: Context<f32>,
    pub b: Context<f32>,

    pub brightness: Context<f32>,
    pub is_on: Context<bool>,
}

impl Wled {
    pub fn get() -> &'static Self {
        &WLED
    }

    pub fn set_r(r: f32) {
        RUNTIME.spawn(async move {
            Self::get().r.set(r);

            let wled = WledController::new(IP_ADDRESS);
            let r = r * 255.0 / 100.0;
            let g = *Self::get().g.get() * 255.0 / 100.0;
            let b = *Self::get().b.get() * 255.0 / 100.0;
            let brightness = *Self::get().brightness.get() * 255.0 / 100.0;

            let _ = wled
                .set_color(r as u8, g as u8, b as u8, Some(brightness as u8))
                .await;
        });
    }

    pub fn set_g(g: f32) {
        RUNTIME.spawn(async move {
            Self::get().g.set(g);

            let wled = WledController::new(IP_ADDRESS);
            let r = *Self::get().r.get() * 255.0 / 100.0;
            let g = g * 255.0 / 100.0;
            let b = *Self::get().b.get() * 255.0 / 100.0;
            let brightness = *Self::get().brightness.get() * 255.0 / 100.0;

            let _ = wled
                .set_color(r as u8, g as u8, b as u8, Some(brightness as u8))
                .await;
        });
    }

    pub fn set_b(b: f32) {
        RUNTIME.spawn(async move {
            Self::get().b.set(b);

            let wled = WledController::new(IP_ADDRESS);
            let r = *Self::get().r.get() * 255.0 / 100.0;
            let g = *Self::get().g.get() * 255.0 / 100.0;
            let b = b * 255.0 / 100.0;
            let brightness = *Self::get().brightness.get() * 255.0 / 100.0;
            let _ = wled
                .set_color(r as u8, g as u8, b as u8, Some(brightness as u8))
                .await;
        });
    }

    pub fn set_brightness(brightness: f32) {
        RUNTIME.spawn(async move {
            Self::get().brightness.set(brightness);

            let wled = WledController::new(IP_ADDRESS);
            let r = *Self::get().r.get() * 255.0 / 100.0;
            let g = *Self::get().g.get() * 255.0 / 100.0;
            let b = *Self::get().b.get() * 255.0 / 100.0;
            let brightness = brightness * 255.0 / 100.0;

            let _ = wled
                .set_color(r as u8, g as u8, b as u8, Some(brightness as u8))
                .await;
        });
    }

    pub fn set_state(state: bool) {
        RUNTIME.spawn(async move {
            Self::get().is_on.set(state);

            let wled = WledController::new(IP_ADDRESS);
            let _ = wled.toggle_power(Some(state)).await;
        });
    }
}
