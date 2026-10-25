#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderLocal;

#[derive(Serialize)]
struct DataCommandData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_non_zero_exit_code: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arguments: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    command: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stdin: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    working_directory: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataCommand_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCommandData_>,
}

#[derive(Clone)]
pub struct DataCommand(std::rc::Rc<DataCommand_>);

impl DataCommand {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderLocal) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc =
        "Set the field `allow_non_zero_exit_code`.\nIndicates that the command returning a non-zero exit code should be treated as a successful execution. Further assertions can be made of the `exit_code` value with the [`check` block](https://developer.hashicorp.com/terraform/language/block/check). Defaults to false."]
    pub fn set_allow_non_zero_exit_code(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_non_zero_exit_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `arguments`.\nArguments to be passed to the given command. Any `null` arguments will be removed from the list."]
    pub fn set_arguments(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().arguments = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `stdin`.\nData to be passed to the given command's standard input as a UTF-8 string. [Terraform values](https://developer.hashicorp.com/terraform/language/expressions/types) can be encoded by any Terraform encode function, for example, [`jsonencode`](https://developer.hashicorp.com/terraform/language/functions/jsonencode)."]
    pub fn set_stdin(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().stdin = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `working_directory`.\nThe directory path where the command should be executed, either an absolute path or relative to the Terraform working directory. If not provided, defaults to the Terraform working directory."]
    pub fn set_working_directory(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().working_directory = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `allow_non_zero_exit_code` after provisioning.\nIndicates that the command returning a non-zero exit code should be treated as a successful execution. Further assertions can be made of the `exit_code` value with the [`check` block](https://developer.hashicorp.com/terraform/language/block/check). Defaults to false."]
    pub fn allow_non_zero_exit_code(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_non_zero_exit_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `arguments` after provisioning.\nArguments to be passed to the given command. Any `null` arguments will be removed from the list."]
    pub fn arguments(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.arguments", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `command` after provisioning.\nExecutable name to be discovered on the PATH or absolute path to executable."]
    pub fn command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.command", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exit_code` after provisioning.\nThe exit code returned by the command. By default, if the exit code is non-zero, the data source will return a diagnostic to Terraform. If a non-zero exit code is expected by the command, set `allow_non_zero_exit_code` to `true`."]
    pub fn exit_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exit_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stderr` after provisioning.\nData returned from the command's standard error stream. The data is returned directly from the command as a UTF-8 string and will be populated regardless of the exit code returned."]
    pub fn stderr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stderr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stdin` after provisioning.\nData to be passed to the given command's standard input as a UTF-8 string. [Terraform values](https://developer.hashicorp.com/terraform/language/expressions/types) can be encoded by any Terraform encode function, for example, [`jsonencode`](https://developer.hashicorp.com/terraform/language/functions/jsonencode)."]
    pub fn stdin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stdin", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stdout` after provisioning.\nData returned from the command's standard output stream. The data is returned directly from the command as a UTF-8 string, which can then be decoded by any Terraform decode function, for example, [`jsondecode`](https://developer.hashicorp.com/terraform/language/functions/jsondecode)."]
    pub fn stdout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stdout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `working_directory` after provisioning.\nThe directory path where the command should be executed, either an absolute path or relative to the Terraform working directory. If not provided, defaults to the Terraform working directory."]
    pub fn working_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_directory", self.extract_ref()))
    }
}

impl terrars::Referable for DataCommand {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCommand { }

impl terrars::ToListMappable for DataCommand {
    type O = terrars::ListRef<DataCommandRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCommand_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "local_command".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCommand {
    pub tf_id: std::string::String,
    #[doc = "Executable name to be discovered on the PATH or absolute path to executable."]
    pub command: terrars::PrimField<std::string::String>,
}

impl BuildDataCommand {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCommand {
        let out = DataCommand(std::rc::Rc::new(DataCommand_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCommandData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                allow_non_zero_exit_code: core::default::Default::default(),
                arguments: core::default::Default::default(),
                command: self.command,
                stdin: core::default::Default::default(),
                working_directory: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCommandRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCommandRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCommandRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `allow_non_zero_exit_code` after provisioning.\nIndicates that the command returning a non-zero exit code should be treated as a successful execution. Further assertions can be made of the `exit_code` value with the [`check` block](https://developer.hashicorp.com/terraform/language/block/check). Defaults to false."]
    pub fn allow_non_zero_exit_code(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_non_zero_exit_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `arguments` after provisioning.\nArguments to be passed to the given command. Any `null` arguments will be removed from the list."]
    pub fn arguments(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.arguments", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `command` after provisioning.\nExecutable name to be discovered on the PATH or absolute path to executable."]
    pub fn command(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.command", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `exit_code` after provisioning.\nThe exit code returned by the command. By default, if the exit code is non-zero, the data source will return a diagnostic to Terraform. If a non-zero exit code is expected by the command, set `allow_non_zero_exit_code` to `true`."]
    pub fn exit_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exit_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stderr` after provisioning.\nData returned from the command's standard error stream. The data is returned directly from the command as a UTF-8 string and will be populated regardless of the exit code returned."]
    pub fn stderr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stderr", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stdin` after provisioning.\nData to be passed to the given command's standard input as a UTF-8 string. [Terraform values](https://developer.hashicorp.com/terraform/language/expressions/types) can be encoded by any Terraform encode function, for example, [`jsonencode`](https://developer.hashicorp.com/terraform/language/functions/jsonencode)."]
    pub fn stdin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stdin", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stdout` after provisioning.\nData returned from the command's standard output stream. The data is returned directly from the command as a UTF-8 string, which can then be decoded by any Terraform decode function, for example, [`jsondecode`](https://developer.hashicorp.com/terraform/language/functions/jsondecode)."]
    pub fn stdout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stdout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `working_directory` after provisioning.\nThe directory path where the command should be executed, either an absolute path or relative to the Terraform working directory. If not provided, defaults to the Terraform working directory."]
    pub fn working_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_directory", self.extract_ref()))
    }
}
