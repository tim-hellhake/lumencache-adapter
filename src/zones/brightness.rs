/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::controller::Controller;
use crate::request::RequestResponse;
use async_trait::async_trait;
use gateway_addon_rust::{
    property,
    property::{AtType as PropertyType, Property},
    PropertyDescription, PropertyStructure,
};

#[property]
pub struct BrightnessProperty {
    controller: Controller,
    dm_id: u8,
}

impl BrightnessProperty {
    pub fn new(controller: Controller, dm_id: u8) -> Self {
        BrightnessProperty { controller, dm_id }
    }
}

impl PropertyStructure for BrightnessProperty {
    type Value = f64;

    fn name(&self) -> String {
        String::from("brightness")
    }

    fn description(&self) -> PropertyDescription<Self::Value> {
        PropertyDescription::default()
            .at_type(PropertyType::BrightnessProperty)
            .title("Brightness")
            .minimum(0)
            .maximum(100)
            .multiple_of(1)
            .read_only(false)
            .value(0_f64)
            .visible(true)
    }
}

#[async_trait]
impl Property for BuiltBrightnessProperty {
    async fn on_update(&mut self, value: Self::Value) -> Result<(), String> {
        let dm_id = self.dm_id;

        let receiver = self
            .controller
            .set_value(dm_id, (value / 100_f64 * 255_f64).round() as u8)
            .await;

        match receiver.await {
            Ok(RequestResponse::Response(_)) => Ok(()),
            Ok(RequestResponse::Timeout) => Err(format!(
                "Failed to set {} of {}: timeout",
                self.property_handle.name, self.dm_id
            )),
            Err(err) => Err(format!(
                "Failed to set {} of {}: {}",
                self.property_handle.name, self.dm_id, err
            )),
        }
    }
}
