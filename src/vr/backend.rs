use std::path::Path;
use std::collections::BTreeMap;
use std::collections::Bound::{Excluded, Unbounded};
use super::vr_api_messages::{VrApiReq, VrApiRsp};
use super::ElementType;

// TODO: Parameterize data based on element type (i.e. don't ignore it)
#[derive(Debug, Clone, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Element {
    pub op: u64,
    pub ty: ElementType,
    pub data: Vec<u8>
}

#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct VrBackend {
    // only public for testing
    pub tree: BTreeMap<String, Element>,
}

impl VrBackend {
    pub fn new() -> VrBackend {
        VrBackend {
            tree: BTreeMap::new(),
        }
    }

    pub fn call(&mut self, op: u64, msg: VrApiReq) -> VrApiRsp {
        match msg {
             VrApiReq::Create {path, ty} => self.create_element(op, path, ty),
             VrApiReq::Put {path, data, cas_tag} => self.put(op, path, data, cas_tag),
             VrApiReq::Delete {path, cas_tag} => self.delete(op, path, cas_tag),
             VrApiReq::Get {path, cas} => self.get(path, cas),
             VrApiReq::List {path} => self.list(path),
             VrApiReq::Null => VrApiRsp::Ok,
             msg => {
                 // This is really bad. Some message has made it through consensus to the baackend. We need to
                 // save all state to disk and panic!.
                 // TODO: Log this error
                 // TODO: Take a snapshot and persist any other state
                 panic!(format!("Backend received unknown msg {:?}", msg))
             }
        }
    }

    fn create_element(&mut self, op: u64, path: String, ty: ElementType) -> VrApiRsp {
        if let Err(rsp) = self.parent_exists(&path) {
            return rsp;
        }
        if let Err(rsp) = self.does_not_exist(&path) {
            return rsp;
        }
        let el = Element { op: op, ty: ty, data: Vec::new()};
        self.tree.insert(path, el);
        VrApiRsp::Ok
    }

    fn get(&self, path: String, cas: bool) -> VrApiRsp {
        if let Some(element) = self.tree.get(&path) {
            if cas {
                return VrApiRsp::Element {data: element.data.clone(),
                                        cas_tag: Some(element.op.clone())}
            }
            VrApiRsp::Element {data: element.data.clone(), cas_tag: None}
        } else {
            VrApiRsp::ElementNotFoundError(path)
        }
    }

    fn list(&self, path: String) -> VrApiRsp {
        if &path != "/" && !self.tree.contains_key(&path) {
            return VrApiRsp::Error {msg: format!("{} does not exist", path)};
        }
        let mut keys = Vec::new();
        for (ref key, _) in self.tree.range::<String, String>(Excluded(&path), Unbounded) {
            keys.push((*key).clone());
        }
        VrApiRsp::KeyList {keys: keys}
    }

    fn delete(&mut self, _op: u64, path: String, cas_tag: Option<u64>)
        -> VrApiRsp {

        if let Some(element) = self.tree.remove(&path) {
            match cas_tag {
                Some(tag) => {
                    if element.op == tag {
                        VrApiRsp::Ok
                    } else {
                        self.tree.insert(path.clone(), element);
                        VrApiRsp::Error {msg: format!("Element {} does not exist", path)}
                    }
                },
                None => VrApiRsp::Ok
            }
        } else {
            VrApiRsp::Error {msg: format!("Element {} does not exist", path)}
        }
    }

    fn put(&mut self, op: u64, path: String, data: Vec<u8>, cas_tag: Option<u64>)
        -> VrApiRsp {

        match self.tree.get_mut(&path) {
            None => VrApiRsp::ElementNotFoundError(path.clone()),
            Some(element) => match cas_tag {
                Some(tag) => {
                    if element.op == tag {
                        VrBackend::put_always(element, op, data)
                    } else {
                        VrApiRsp::CasFailedError {path: path.clone(),
                                                      expected: tag,
                                                      actual: element.op}
                    }
                },
                None => {
                    VrBackend::put_always(element, op, data)
                }
            }
        }
    }

    // TODO: Try to convert data to it's proper type based on element.ty
    fn put_always(element: &mut Element, op: u64, data: Vec<u8>) -> VrApiRsp {
        (*element).op = op;
        (*element).data = data;
        VrApiRsp::Ok
    }

    fn does_not_exist(&self, path: &str) -> Result<(), VrApiRsp> {
        if self.tree.contains_key(path) {
            Err(VrApiRsp::ElementAlreadyExistsError)
        } else {
            Ok(())
        }
    }

    fn parent_exists(&self, path: &str) -> Result<(), VrApiRsp> {
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            // We don't have an explicit root
            if parent == Path::new("/") { return Ok(()) };
            let parent = parent.to_str().unwrap();
            if self.tree.contains_key(parent) {
                Ok(())
            } else {
                Err(VrApiRsp::ParentNotFoundError)
            }
        } else {
            Ok(())
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use vr::vr_api_messages::VrApiRsp;
    use vr::ElementType;

    fn assert_ok(res: VrApiRsp) {
        assert_eq!(res, VrApiRsp::Ok)
    }

    fn assert_err(res: VrApiRsp) {
        assert!(res.is_err())
    }

    #[test]
    fn create_element() {
        let mut backend = VrBackend::new();
        assert_ok(backend.create_element(1, "/foo".to_string(), ElementType::Binary));
        assert_err(backend.create_element(2, "/foo/bar/wat".to_string(), ElementType::Binary));
        assert_err(backend.create_element(3, "relative_path".to_string(), ElementType::Binary));
        assert_err(backend.create_element(4, "rel/path".to_string(), ElementType::Binary));
        assert_ok(backend.create_element(5, "/foo/bar".to_string(), ElementType::Binary));
    }

    #[test]
    fn put() {
        let mut backend = VrBackend::new();
        assert_err(backend.put(1, "/foo".to_string(), Vec::new(), None));
        assert_err(backend.put(2,"/foo".to_string(), Vec::new(), Some(1)));
        assert_ok(backend.create_element(3, "/foo".to_string(), ElementType::Binary));
        assert_ok(backend.put(4, "/foo".to_string(), vec![5,6,7], None));
        assert_err(backend.put(5, "/foo".to_string(), vec![8,9,10], Some(3)));
        assert_err(backend.put(6, "/foo".to_string(), vec![8,9,10], Some(5)));
    }

    #[test]
    fn get() {
        let mut backend = VrBackend::new();
        assert_err(backend.get("/foo".to_string(), false));
        assert_err(backend.get("/foo".to_string(), true));
        assert_ok(backend.create_element(1, "/foo".to_string(), ElementType::Binary));
        let elem = backend.get("/foo".to_string(), false);
        if let VrApiRsp::Element {cas_tag, ..} = elem {
            assert_eq!(cas_tag, None);
        } else {
            assert!(false);
        }
        let elem = backend.get("/foo".to_string(), true);
        if let VrApiRsp::Element {cas_tag, ..} = elem {
            assert_eq!(cas_tag, Some(1));
        } else {
            assert!(false);
        }
    }
}
