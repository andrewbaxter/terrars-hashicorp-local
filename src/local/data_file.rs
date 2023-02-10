use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderLocal;

#[derive(Serialize)]
struct DataFileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    filename: PrimField<String>,
}

struct DataFile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataFileData>,
}

#[derive(Clone)]
pub struct DataFile(Rc<DataFile_>);

impl DataFile {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderLocal) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\nRaw content of the file that was read, as UTF-8 encoded string. Files that do not contain UTF-8 text will have invalid UTF-8 sequences in `content`\n  replaced with the Unicode replacement character. "]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_base64` after provisioning.\nBase64 encoded version of the file content (use this when dealing with binary data)."]
    pub fn content_base64(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_base64", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filename` after provisioning.\nPath to the file that will be read. The data source will return an error if the file does not exist."]
    pub fn filename(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filename", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe hexadecimal encoding of the checksum of the file content."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl Datasource for DataFile {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataFile {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataFile {
    type O = ListRef<DataFileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(self))
    }
}

impl Datasource_ for DataFile_ {
    fn extract_datasource_type(&self) -> String {
        "local_file".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataFile {
    pub tf_id: String,
    #[doc= "Path to the file that will be read. The data source will return an error if the file does not exist."]
    pub filename: PrimField<String>,
}

impl BuildDataFile {
    pub fn build(self, stack: &mut Stack) -> DataFile {
        let out = DataFile(Rc::new(DataFile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataFileData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                filename: self.filename,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataFileRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataFileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataFileRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\nRaw content of the file that was read, as UTF-8 encoded string. Files that do not contain UTF-8 text will have invalid UTF-8 sequences in `content`\n  replaced with the Unicode replacement character. "]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_base64` after provisioning.\nBase64 encoded version of the file content (use this when dealing with binary data)."]
    pub fn content_base64(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_base64", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filename` after provisioning.\nPath to the file that will be read. The data source will return an error if the file does not exist."]
    pub fn filename(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filename", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe hexadecimal encoding of the checksum of the file content."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}
