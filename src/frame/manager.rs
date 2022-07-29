use std::{collections::HashMap, sync::Arc};

use super::{item::{Item,ItemChild}, frame::FrameInterface};

use ulid::Ulid;

struct ItemManager {
    id:Ulid,
    map:HashMap<Ulid,Item>,
}

impl FrameInterface for ItemManager {
    fn get_settings(&self) -> String {
        todo!()
    }

    fn get_ulid(&self) -> Ulid {
        self.id
    }

    fn process(&self,f:Option<super::frame::Frame>,json:&str) -> Result<Option<super::frame::Frame>,String> {
        let mut ff : Option<super::frame::Frame> = f;
        for i in self.map.iter() {
          ff = (*i.1).process(ff, json)?;
        }

        return Ok(ff);
    }
}

impl ItemManager {
    fn add(&mut self,item:Item) -> Ulid{
        let id = item.id.clone();
        self.map.insert(item.id, item);
        return id
    }

    fn del(&mut self,id:&Ulid) {
        self.map.remove(id);
    }

    fn add_child(&mut self,parent:&mut Item,child:ItemChild) -> Ulid{
        let c_id = match &child {
            ItemChild::FI(fi) => fi.get_ulid(),
            ItemChild::Item(item) => item.id,
        };
        parent.map_child.insert(c_id, child);
        return c_id
    }

    fn del_child(&mut self,parent:&mut Item,child_id:&Ulid) {
        parent.map_child.remove(child_id);
    }

    fn get(&self,id:&Ulid) -> Option<&Item>{
        self.map.get(id)
    }

    fn mov(&mut self,id:&Ulid,layer:usize,lr:(usize,usize)) -> bool{
        let mut collid = false;
        let collid = self.map.iter().any(|f|
          ((*f.1).layer == layer) && Self::is_collid(lr,(*f.1).lr)
        );

        if collid {
          return false;
        } else {
          self.map.get_mut(id).unwrap().layer = layer;
          self.map.get_mut(id).unwrap().lr = lr;
          return true;
        }
    }

    fn is_collid(lr1:(usize,usize),lr2:(usize,usize)) -> bool {
      !((lr1.1 < lr2.0) || (lr2.1 < lr1.0))
    }
}