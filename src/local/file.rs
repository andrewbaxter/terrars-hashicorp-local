use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderLocal;

#[derive(Serialize)]
struct FileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_base64: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    directory_permission: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_permission: Option<PrimField<String>>,
    filename: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensitive_content: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
}

struct File_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FileData>,
}

#[derive(Clone)]
pub struct File(Rc<File_>);

impl File {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderLocal) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `content`.\nContent to store in the file, expected to be a UTF-8 encoded string.\n Conflicts with `sensitive_content`, `content_base64` and `source`.\n Exactly one of these four arguments must be specified."]
    pub fn set_content(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content = Some(v.into());
        self
    }

    #[doc= "Set the field `content_base64`.\nContent to store in the file, expected to be binary encoded as base64 string.\n Conflicts with `content`, `sensitive_content` and `source`.\n Exactly one of these four arguments must be specified."]
    pub fn set_content_base64(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_base64 = Some(v.into());
        self
    }

    #[doc= "Set the field `directory_permission`.\nPermissions to set for directories created (before umask), expressed as string in\n [numeric notation](https://en.wikipedia.org/wiki/File-system_permissions#Numeric_notation).\n Default value is `\"0777\"`."]
    pub fn set_directory_permission(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().directory_permission = Some(v.into());
        self
    }

    #[doc= "Set the field `file_permission`.\nPermissions to set for the output file (before umask), expressed as string in\n [numeric notation](https://en.wikipedia.org/wiki/File-system_permissions#Numeric_notation).\n Default value is `\"0777\"`."]
    pub fn set_file_permission(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().file_permission = Some(v.into());
        self
    }

    #[doc= "Set the field `sensitive_content`.\nSensitive content to store in the file, expected to be an UTF-8 encoded string.\n Will not be displayed in diffs.\n Conflicts with `content`, `content_base64` and `source`.\n Exactly one of these four arguments must be specified.\n If in need to use _sensitive_ content, please use the [`local_sensitive_file`](./sensitive_file.html)\n resource instead."]
    pub fn set_sensitive_content(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sensitive_content = Some(v.into());
        self
    }

    #[doc= "Set the field `source`.\nPath to file to use as source for the one we are creating.\n Conflicts with `content`, `sensitive_content` and `content_base64`.\n Exactly one of these four arguments must be specified."]
    pub fn set_source(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\nContent to store in the file, expected to be a UTF-8 encoded string.\n Conflicts with `sensitive_content`, `content_base64` and `source`.\n Exactly one of these four arguments must be specified."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_base64` after provisioning.\nContent to store in the file, expected to be binary encoded as base64 string.\n Conflicts with `content`, `sensitive_content` and `source`.\n Exactly one of these four arguments must be specified."]
    pub fn content_base64(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_base64", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `directory_permission` after provisioning.\nPermissions to set for directories created (before umask), expressed as string in\n [numeric notation](https://en.wikipedia.org/wiki/File-system_permissions#Numeric_notation).\n Default value is `\"0777\"`."]
    pub fn directory_permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_permission", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_permission` after provisioning.\nPermissions to set for the output file (before umask), expressed as string in\n [numeric notation](https://en.wikipedia.org/wiki/File-system_permissions#Numeric_notation).\n Default value is `\"0777\"`."]
    pub fn file_permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_permission", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filename` after provisioning.\nThe path to the file that will be created.\n Missing parent directories will be created.\n If the file already exists, it will be overridden with the given content."]
    pub fn filename(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filename", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe hexadecimal encoding of the checksum of the file content"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sensitive_content` after provisioning.\nSensitive content to store in the file, expected to be an UTF-8 encoded string.\n Will not be displayed in diffs.\n Conflicts with `content`, `content_base64` and `source`.\n Exactly one of these four arguments must be specified.\n If in need to use _sensitive_ content, please use the [`local_sensitive_file`](./sensitive_file.html)\n resource instead."]
    pub fn sensitive_content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sensitive_content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\nPath to file to use as source for the one we are creating.\n Conflicts with `content`, `sensitive_content` and `content_base64`.\n Exactly one of these four arguments must be specified."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

impl Resource for File {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for File {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for File {
    type O = ListRef<FileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for File_ {
    fn extract_resource_type(&self) -> String {
        "local_file".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFile {
    pub tf_id: String,
    #[doc= "The path to the file that will be created.\n Missing parent directories will be created.\n If the file already exists, it will be overridden with the given content."]
    pub filename: PrimField<String>,
}

impl BuildFile {
    pub fn build(self, stack: &mut Stack) -> File {
        let out = File(Rc::new(File_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FileData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
    shared: StackShared,
    base: String,
}

impl Ref for FileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FileRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\nContent to store in the file, expected to be a UTF-8 encoded string.\n Conflicts with `sensitive_content`, `content_base64` and `source`.\n Exactly one of these four arguments must be specified."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_base64` after provisioning.\nContent to store in the file, expected to be binary encoded as base64 string.\n Conflicts with `content`, `sensitive_content` and `source`.\n Exactly one of these four arguments must be specified."]
    pub fn content_base64(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_base64", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `directory_permission` after provisioning.\nPermissions to set for directories created (before umask), expressed as string in\n [numeric notation](https://en.wikipedia.org/wiki/File-system_permissions#Numeric_notation).\n Default value is `\"0777\"`."]
    pub fn directory_permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_permission", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_permission` after provisioning.\nPermissions to set for the output file (before umask), expressed as string in\n [numeric notation](https://en.wikipedia.org/wiki/File-system_permissions#Numeric_notation).\n Default value is `\"0777\"`."]
    pub fn file_permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_permission", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filename` after provisioning.\nThe path to the file that will be created.\n Missing parent directories will be created.\n If the file already exists, it will be overridden with the given content."]
    pub fn filename(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filename", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe hexadecimal encoding of the checksum of the file content"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sensitive_content` after provisioning.\nSensitive content to store in the file, expected to be an UTF-8 encoded string.\n Will not be displayed in diffs.\n Conflicts with `content`, `content_base64` and `source`.\n Exactly one of these four arguments must be specified.\n If in need to use _sensitive_ content, please use the [`local_sensitive_file`](./sensitive_file.html)\n resource instead."]
    pub fn sensitive_content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sensitive_content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\nPath to file to use as source for the one we are creating.\n Conflicts with `content`, `sensitive_content` and `content_base64`.\n Exactly one of these four arguments must be specified."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}
