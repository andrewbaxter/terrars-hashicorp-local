use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ProviderLocalData {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
}

struct ProviderLocal_ {
    data: RefCell<ProviderLocalData>,
}

pub struct ProviderLocal(Rc<ProviderLocal_>);

impl ProviderLocal {
    pub fn provider_ref(&self) -> String {
        let data = self.0.data.borrow();
        if let Some(alias) = &data.alias {
            format!("{}.{}", "local", alias)
        } else {
            "local".into()
        }
    }

    pub fn set_alias(self, alias: impl ToString) -> Self {
        self.0.data.borrow_mut().alias = Some(alias.to_string());
        self
    }
}

impl Provider for ProviderLocal_ {
    fn extract_type_tf_id(&self) -> String {
        "local".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "hashicorp/local",
            "version": "2.3.0",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderLocal {}

impl BuildProviderLocal {
    pub fn build(self, stack: &mut Stack) -> ProviderLocal {
        let out =
            ProviderLocal(Rc::new(ProviderLocal_ { data: RefCell::new(ProviderLocalData { alias: None }) }));
        stack.add_provider(out.0.clone());
        out
    }
}
