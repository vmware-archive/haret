use std::path::Path;
use std::collections::BTreeMap;
use std::collections::Bound::{Excluded, Unbounded};
use super::messages::{Req, Rsp};
use element::{ElementType, Version};

// TODO: Parameterize data based on element type (i.e. don't ignore it)
struct Element {
    version: Version,
    ty: ElementType,
    data: Vec<u8>
}

pub struct VrBackend {
    tree: BTreeMap<String, Element>,
}

impl VrBackend {
    pub fn new() -> VrBackend {
        VrBackend {
            tree: BTreeMap::new(),
        }
    }

    pub fn call(&mut self, version: Version, msg: Req) -> Rsp {
        let resp = match msg {
             Req::Create {path, ty} => self.create_element(version, path, ty),
             Req::Put {path, data, cas_tag} => self.put(version, path, data, cas_tag),
             Req::Delete {path, cas_tag} => self.delete(version, path, cas_tag),
             Req::Get {path, cas} => self.get(path, cas),
             Req::List {path} => self.list(path)
        };
        match resp {
            Ok(msg) => msg,
            Err(msg) => msg
        }
    }

    fn create_element(&mut self, version: Version, path: String, ty: ElementType) -> Result<Rsp, Rsp> {
        try!(self.parent_exists(&path));
        try!(self.does_not_exist(&path));
        let el = Element { version: version, ty: ty, data: Vec::new()};
        self.tree.insert(path, el);
        Ok(Rsp::Ok)
    }

    fn get(&self, path: String, cas: bool) -> Result<Rsp, Rsp> {
        if let Some(element) = self.tree.get(&path) {
            if cas {
                return Ok(Rsp::Element {data: element.data.clone(),
                                        cas_tag: Some(element.version.clone())})
            }
            Ok(Rsp::Element {data: element.data.clone(), cas_tag: None})
        } else {
            Err(Rsp::Error {msg: format!("No element found for {:?}", path)})
        }
    }

    fn list(&self, path: String) -> Result<Rsp, Rsp> {
        if &path != "/" && !self.tree.contains_key(&path) {
            return Err(Rsp::Error {msg: format!("{} does not exist", path)});
        }
        let mut keys = Vec::new();
        for (ref key, _) in self.tree.range::<String, String>(Excluded(&path), Unbounded) {
            keys.push((*key).clone());
        }
        Ok(Rsp::KeyList {keys: keys})
    }

    fn delete(&mut self, version: Version, path: String, cas_tag: Option<Version>)
        -> Result<Rsp, Rsp> {

        if let Some(element) = self.tree.remove(&path) {
            match cas_tag {
                Some(tag) => {
                    if element.version == tag {
                        Ok(Rsp::Ok)
                    } else {
                        self.tree.insert(path.clone(), element);
                        Err(Rsp::Error {msg: format!("Element {} does not exist", path)})
                    }
                },
                None => Ok(Rsp::Ok)
            }
        } else {
            Err(Rsp::Error {msg: format!("Element {} does not exist", path)})
        }
    }

    fn put(&mut self, version: Version, path: String, data: Vec<u8>, cas_tag: Option<Version>)
        -> Result<Rsp, Rsp> {

        match self.tree.get_mut(&path) {
            None => Err(Rsp::Error {msg: format!("Element {} does not exist", path)}),
            Some(element) => match cas_tag {
                Some(tag) => {
                    if element.version == tag {
                        VrBackend::put_always(element, version, data)
                    } else {
                        Err(Rsp::Error {msg: format!("CAS failed. Version = {:?}, Expected  {:?}",
                                                     element.version, tag)})
                    }
                },
                None => {
                    VrBackend::put_always(element, version, data)
                }
            }
        }
    }

    // TODO: Try to convert data to it's proper type based on element.ty
    fn put_always(element: &mut Element, version: Version, data: Vec<u8>) -> Result<Rsp, Rsp> {
        (*element).version = version;
        (*element).data = data;
        Ok(Rsp::Ok)
    }

    fn does_not_exist(&self, path: &str) -> Result<(), Rsp> {
        if self.tree.contains_key(path) {
            Err(Rsp::Error {msg: format!("Element {:?} already exists", path)})
        } else {
            Ok(())
        }
    }

    fn parent_exists(&self, path: &str) -> Result<(), Rsp> {
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            // We don't have an explicit root
            if parent == Path::new("/") { return Ok(()) };
            let parent = parent.to_str().unwrap();
            if self.tree.contains_key(parent) {
                Ok(())
            } else {
                Err(Rsp::Error {msg: format!("Parent element {} does not exist", parent)})
            }
        } else {
            Ok(())
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use vr_api::messages::*;
    use element::{ElementType, Version};

    fn assert_ok(res: Result<Rsp, Rsp>) {
        match res {
            Ok(val) => assert_eq!(val, Rsp::Ok),
            Err(e) => {
                println!("assert_ok failed with error: {:?}", e);
                assert!(false)
            }
        }
    }

    fn assert_err(res: Result<Rsp, Rsp>) {
        match res {
            Ok(_) => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn create_element() {
        let mut backend = VrBackend::new();
        let mut version = Version::new();
        assert_ok(backend.create_element(version.inc(), "/foo".to_string(), ElementType::Binary));
        assert_err(backend.create_element(version.inc(), "/foo/bar/wat".to_string(), ElementType::Binary));
        assert_err(backend.create_element(version.inc(), "relative_path".to_string(), ElementType::Binary));
        assert_err(backend.create_element(version.inc(), "rel/path".to_string(), ElementType::Binary));
        assert_ok(backend.create_element(version.inc(), "/foo/bar".to_string(), ElementType::Binary));
    }

    #[test]
    fn put() {
        let mut backend = VrBackend::new();
        let mut version = Version::new();
        assert_err(backend.put(version.inc(), "/foo".to_string(), Vec::new(), None));
        assert_err(backend.put(version.inc(),"/foo".to_string(), Vec::new(), Some(version.clone())));
        assert_ok(backend.create_element(version.inc(), "/foo".to_string(), ElementType::Binary));
        assert_ok(backend.put(version.inc(), "/foo".to_string(), vec![5,6,7], None));
        assert_err(backend.put(version.inc(), "/foo".to_string(), vec![8,9,10], Some(version.clone())));
        assert_err(backend.put(version.inc(), "/foo".to_string(), vec![8,9,10], Some(version.inc())));
    }

    #[test]
    fn get() {
        let mut backend = VrBackend::new();
        let mut version = Version::new();
        assert_err(backend.get("/foo".to_string(), false));
        assert_err(backend.get("/foo".to_string(), true));
        assert_ok(backend.create_element(version.inc(), "/foo".to_string(), ElementType::Binary));
        let elem = backend.get("/foo".to_string(), false);
        if let Ok(Rsp::Element {cas_tag, ..}) = elem {
            assert_eq!(cas_tag, None);
        } else {
            assert!(false);
        }
        let elem = backend.get("/foo".to_string(), true);
        if let Ok(Rsp::Element {cas_tag, ..}) = elem {
            assert_eq!(cas_tag, Some(Version::new().inc()));
        } else {
            assert!(false);
        }
    }


}
