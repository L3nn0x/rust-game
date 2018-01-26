extern crate std;

use std::hash::Hash;
use std::collections::HashMap;
use std::rc::Rc;
use std::borrow::Borrow;

pub trait ResourceLoader<Resource> {
  type Args: ?Sized;
  fn load(&self, data: &Self::Args) -> Result<Resource, String>;
}

pub struct ResourceManager<Key, Resource, Loader>
    where Key: Hash + Eq,
          Loader: ResourceLoader<Resource> {
  loader: Loader,
  cache: HashMap<Key, Rc<Resource>>
}

impl<K, R, L> ResourceManager<K, R, L>
    where K: Hash + Eq,
          L: ResourceLoader<R> {
  pub fn new(loader: L) -> Self {
    ResourceManager{
      loader: loader,
      cache: HashMap::new()
    }
  }
  
  pub fn load<D>(&mut self, details: &D) -> Result<Rc<R>, String>
      where D: Eq + Hash + ?Sized,
            L: ResourceLoader<R, Args = D>,
            K: Borrow<D> + for<'a> From<&'a D> {
    self.cache.get(details).cloned().map_or_else(|| {
      let resource = Rc::new(self.loader.load(details)?);
      self.cache.insert(details.into(), resource.clone());
      Ok(resource)
    }, Ok)
  }
}
