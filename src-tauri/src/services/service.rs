use crate::themes::Parameter;

pub trait Service {
    fn subscribe(
        &mut self,
        subscription_data: Option<Vec<String>>,
        data_names: Option<Vec<String>>,
    );
    fn get(&self) -> Result<Vec<ServiceResult>, &'static str>;
    fn options(&self) -> Option<Vec<ServiceOption>>;
    fn suspend(&mut self) -> Result<(), &'static str>;
    fn settings(&self) -> Option<Vec<Parameter>>;
    fn change_settings(&mut self, setting: Option<(String, String)>);
}

pub struct ServiceResult {
    pub value: String,
    pub origin: String,
    pub value_type: String,
    pub code_name: String,
}

pub struct ServiceOption {
    pub subscription_data: String,
}

pub struct ServicesContainer {
    services: Vec<Box<dyn Service>>,
}
