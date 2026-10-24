use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderLocal;

#[derive(Serialize)]
struct FileData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_base64: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    directory_permission: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_permission: std::option::Option<terrars::PrimField<std::string::String>>,
    filename: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitive_content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct File_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FileData>,
}

#[derive(Clone)]
pub struct File(std::rc::Rc<File_>);

impl File {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderLocal) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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
        "Set the field `content`.\nContent to store in the file, expected to be a UTF-8 encoded string.\n Conflicts with `sensitive_content`, `content_base64` and `source`.\n Exactly one of these four arguments must be specified."]
    pub fn set_content(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().content = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `content_base64`.\nContent to store in the file, expected to be binary encoded as base64 string.\n Conflicts with `content`, `sensitive_content` and `source`.\n Exactly one of these four arguments must be specified."]
    pub fn set_content_base64(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().content_base64 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `directory_permission`.\nPermissions to set for directories created (before umask), expressed as string in\n [numeric notation](https://en.wikipedia.org/wiki/File-system_permissions#Numeric_notation).\n Default value is `\"0777\"`."]
    pub fn set_directory_permission(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().directory_permission = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_permission`.\nPermissions to set for the output file (before umask), expressed as string in\n [numeric notation](https://en.wikipedia.org/wiki/File-system_permissions#Numeric_notation).\n Default value is `\"0777\"`."]
    pub fn set_file_permission(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().file_permission = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sensitive_content`.\nSensitive content to store in the file, expected to be an UTF-8 encoded string.\n Will not be displayed in diffs.\n Conflicts with `content`, `content_base64` and `source`.\n Exactly one of these four arguments must be specified.\n If in need to use _sensitive_ content, please use the [`local_sensitive_file`](./sensitive_file.html)\n resource instead."]
    pub fn set_sensitive_content(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().sensitive_content = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source`.\nPath to file to use as source for the one we are creating.\n Conflicts with `content`, `sensitive_content` and `content_base64`.\n Exactly one of these four arguments must be specified."]
    pub fn set_source(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nContent to store in the file, expected to be a UTF-8 encoded string.\n Conflicts with `sensitive_content`, `content_base64` and `source`.\n Exactly one of these four arguments must be specified."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_base64` after provisioning.\nContent to store in the file, expected to be binary encoded as base64 string.\n Conflicts with `content`, `sensitive_content` and `source`.\n Exactly one of these four arguments must be specified."]
    pub fn content_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_base64", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `directory_permission` after provisioning.\nPermissions to set for directories created (before umask), expressed as string in\n [numeric notation](https://en.wikipedia.org/wiki/File-system_permissions#Numeric_notation).\n Default value is `\"0777\"`."]
    pub fn directory_permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_permission", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `file_permission` after provisioning.\nPermissions to set for the output file (before umask), expressed as string in\n [numeric notation](https://en.wikipedia.org/wiki/File-system_permissions#Numeric_notation).\n Default value is `\"0777\"`."]
    pub fn file_permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_permission", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filename` after provisioning.\nThe path to the file that will be created.\n Missing parent directories will be created.\n If the file already exists, it will be overridden with the given content."]
    pub fn filename(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe hexadecimal encoding of the checksum of the file content"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sensitive_content` after provisioning.\nSensitive content to store in the file, expected to be an UTF-8 encoded string.\n Will not be displayed in diffs.\n Conflicts with `content`, `content_base64` and `source`.\n Exactly one of these four arguments must be specified.\n If in need to use _sensitive_ content, please use the [`local_sensitive_file`](./sensitive_file.html)\n resource instead."]
    pub fn sensitive_content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensitive_content", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nPath to file to use as source for the one we are creating.\n Conflicts with `content`, `sensitive_content` and `content_base64`.\n Exactly one of these four arguments must be specified."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

impl terrars::Referable for File {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for File { }

impl terrars::ToListMappable for File {
    type O = terrars::ListRef<FileRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for File_ {
    fn extract_resource_type(&self) -> std::string::String {
        "local_file".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFile {
    pub tf_id: std::string::String,
    #[doc =
        "The path to the file that will be created.\n Missing parent directories will be created.\n If the file already exists, it will be overridden with the given content."]
    pub filename: terrars::PrimField<std::string::String>,
}

impl BuildFile {
    pub fn build(self, stack: &mut terrars::Stack) -> File {
        let out = File(std::rc::Rc::new(File_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FileData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                content: core::default::Default::default(),
                content_base64: core::default::Default::default(),
                directory_permission: core::default::Default::default(),
                file_permission: core::default::Default::default(),
                filename: self.filename,
                sensitive_content: core::default::Default::default(),
                source: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FileRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nContent to store in the file, expected to be a UTF-8 encoded string.\n Conflicts with `sensitive_content`, `content_base64` and `source`.\n Exactly one of these four arguments must be specified."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_base64` after provisioning.\nContent to store in the file, expected to be binary encoded as base64 string.\n Conflicts with `content`, `sensitive_content` and `source`.\n Exactly one of these four arguments must be specified."]
    pub fn content_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_base64", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `directory_permission` after provisioning.\nPermissions to set for directories created (before umask), expressed as string in\n [numeric notation](https://en.wikipedia.org/wiki/File-system_permissions#Numeric_notation).\n Default value is `\"0777\"`."]
    pub fn directory_permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_permission", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `file_permission` after provisioning.\nPermissions to set for the output file (before umask), expressed as string in\n [numeric notation](https://en.wikipedia.org/wiki/File-system_permissions#Numeric_notation).\n Default value is `\"0777\"`."]
    pub fn file_permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_permission", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filename` after provisioning.\nThe path to the file that will be created.\n Missing parent directories will be created.\n If the file already exists, it will be overridden with the given content."]
    pub fn filename(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe hexadecimal encoding of the checksum of the file content"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sensitive_content` after provisioning.\nSensitive content to store in the file, expected to be an UTF-8 encoded string.\n Will not be displayed in diffs.\n Conflicts with `content`, `content_base64` and `source`.\n Exactly one of these four arguments must be specified.\n If in need to use _sensitive_ content, please use the [`local_sensitive_file`](./sensitive_file.html)\n resource instead."]
    pub fn sensitive_content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensitive_content", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nPath to file to use as source for the one we are creating.\n Conflicts with `content`, `sensitive_content` and `content_base64`.\n Exactly one of these four arguments must be specified."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}
