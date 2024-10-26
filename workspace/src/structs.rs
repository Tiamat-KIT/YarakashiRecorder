use validator::Validate;
use dioxus::prelude::*;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Validate,PartialEq,Clone,Debug,Props)]
pub struct Inputs {
    #[validate(required,length(min = 3))]
    pub title: Option<String>,
    #[validate(required,length(min = 3))]
    pub description:  Option<String>,
    pub optional_info: Option<String>,
}

impl Inputs {
    pub fn new(binding: &HashMap<String, FormValue>) -> Result<Self, validator::ValidationErrors> {
        let inputs = Self {
            title: binding.get("title")
                .expect("NotFound")
                .0.get(0).cloned(),


            description:  binding.get("description")
                .expect("NotFound")
                .0.get(0).cloned(),

            optional_info: binding.get("optional_info")
                .expect("NotFound")
                .0.get(0).cloned()
        };
        let result = match inputs.validate() {
            Ok(_) => {
                Ok(inputs)
            },
            Err(e) => {
                return Err(e)
            },
        };
        return result
    }
}
