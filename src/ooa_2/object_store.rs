use std::{
    collections::{hash_map::Values, HashMap},
    io,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::sarzak::{
    Associative, AssociativeReferent, AssociativeReferrer, Attribute, Binary, Isa, Object,
    Referent, Referrer, Relationship, Subtype, Supertype,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    obj: HashMap<String, Uuid>,
    rel: HashMap<String, Uuid>,
    object: HashMap<Uuid, Object>,
    relationship: HashMap<Uuid, Relationship>,
    attribute: HashMap<Uuid, Attribute>,
    binary: HashMap<Uuid, Binary>,
    referrer: HashMap<Uuid, Referrer>,
    referent: HashMap<Uuid, Referent>,
    isa: HashMap<Uuid, Isa>,
    supertype: HashMap<Uuid, Supertype>,
    subtype: HashMap<Uuid, Subtype>,
    associative_referrer: HashMap<Uuid, AssociativeReferrer>,
    associative_referent: HashMap<Uuid, AssociativeReferent>,
    associative: HashMap<Uuid, Associative>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            obj: HashMap::new(),
            rel: HashMap::new(),
            object: HashMap::new(),
            relationship: HashMap::new(),
            attribute: HashMap::new(),
            binary: HashMap::new(),
            referrer: HashMap::new(),
            referent: HashMap::new(),
            isa: HashMap::new(),
            supertype: HashMap::new(),
            subtype: HashMap::new(),
            associative_referrer: HashMap::new(),
            associative_referent: HashMap::new(),
            associative: HashMap::new(),
        }
    }

    pub fn objects(&self) -> Values<'_, Uuid, Object> {
        self.object.values()
    }

    pub fn relationships(&self) -> Values<'_, Uuid, Relationship> {
        self.relationship.values()
    }

    pub fn get_obj<S: AsRef<str>>(&self, obj: S) -> Option<&Object> {
        self.obj
            .get(obj.as_ref())
            .and_then(|id| self.object.get(id))
    }

    pub fn get_rel<S: AsRef<str>>(&self, rel: S) -> Option<&Relationship> {
        self.rel
            .get(rel.as_ref())
            .and_then(|id| self.relationship.get(id))
    }

    pub fn inter_object(&mut self, object: Object) {
        self.obj.insert(object.name.to_string().clone(), object.id);
        self.object.insert(object.id, object);
    }

    pub fn exhume_object(&self, id: &Uuid) -> Option<&Object> {
        self.object.get(id)
    }

    pub fn iter_object(&self) -> impl Iterator<Item = (&Uuid, &Object)> + '_ {
        self.object.iter()
    }

    pub fn inter_rel(&mut self, key: String, value: Uuid) {
        self.rel.insert(key, value);
    }

    /// Inter a Relationship
    ///
    /// This one is tricky, because [Relationship] doesn't have an id -- it's an enum.
    /// So the id that we will be forced to use is the id of the variant.
    pub fn inter_relationship(&mut self, relationship: Relationship) {
        self.relationship
            .insert(relationship.get_id().clone(), relationship);
    }

    pub fn iter_relationship(&self) -> impl Iterator<Item = (&Uuid, &Relationship)> + '_ {
        self.relationship.iter()
    }

    // pub fn sorted_relationship(&self) -> Box<dyn Fn() -> Iter<Relationship>> {
    //     let mut values: Vec<Relationship> = self.relationship.values().cloned().collect();
    //     values.sort();
    //     let foo = move || values.iter();
    //     Box::new(foo)
    // }

    pub fn exhume_relationship(&self, id: &Uuid) -> Option<&Relationship> {
        self.relationship.get(id)
    }

    pub fn inter_attribute(&mut self, attribute: Attribute) {
        self.attribute.insert(attribute.id, attribute);
    }

    pub fn iter_attribute(&self) -> impl Iterator<Item = (&Uuid, &Attribute)> + '_ {
        self.attribute.iter()
    }

    pub fn exhume_attribute(&self, id: &Uuid) -> Option<&Attribute> {
        self.attribute.get(id)
    }

    pub fn inter_binary(&mut self, binary: Binary) {
        self.binary.insert(binary.id, binary);
    }

    pub fn exhume_binary(&self, id: &Uuid) -> Option<&Binary> {
        self.binary.get(id)
    }

    pub fn iter_binary(&self) -> impl Iterator<Item = (&Uuid, &Binary)> + '_ {
        self.binary.iter()
    }

    pub fn inter_referrer(&mut self, referrer: Referrer) {
        self.referrer.insert(referrer.id, referrer);
    }

    pub fn exhume_referrer(&self, id: &Uuid) -> Option<&Referrer> {
        self.referrer.get(id)
    }

    pub fn inter_referent(&mut self, referent: Referent) {
        self.referent.insert(referent.id, referent);
    }

    pub fn exhume_referent(&self, id: &Uuid) -> Option<&Referent> {
        self.referent.get(id)
    }

    pub fn inter_isa(&mut self, isa: Isa) {
        self.isa.insert(isa.id, isa);
    }

    pub fn exhume_isa(&self, id: &Uuid) -> Option<&Isa> {
        self.isa.get(id)
    }

    pub fn iter_isa(&self) -> impl Iterator<Item = (&Uuid, &Isa)> + '_ {
        self.isa.iter()
    }

    pub fn inter_supertype(&mut self, supertype: Supertype) {
        self.supertype.insert(supertype.id, supertype);
    }

    pub fn exhume_supertype(&self, id: &Uuid) -> Option<&Supertype> {
        self.supertype.get(id)
    }

    pub fn iter_supertype(&self) -> impl Iterator<Item = (&Uuid, &Supertype)> + '_ {
        self.supertype.iter()
    }

    pub fn inter_subtype(&mut self, subtype: Subtype) {
        self.subtype.insert(subtype.id, subtype);
    }

    /// Get an iterator over the internal `HashMap<(&Uuid, Subtype)>` in the [`ObjectStore`]
    ///
    pub fn iter_subtype(&self) -> impl Iterator<Item = (&Uuid, &Subtype)> {
        self.subtype.iter()
    }

    pub fn exhume_subtype(&self, id: &Uuid) -> Option<&Subtype> {
        self.subtype.get(id)
    }

    pub fn inter_associative_referrer(&mut self, associative_referrer: AssociativeReferrer) {
        self.associative_referrer
            .insert(associative_referrer.id, associative_referrer);
    }

    pub fn exhume_associative_referrer(&self, id: &Uuid) -> Option<&AssociativeReferrer> {
        self.associative_referrer.get(id)
    }

    pub fn inter_associative_referent(&mut self, associative_referent: AssociativeReferent) {
        self.associative_referent
            .insert(associative_referent.id, associative_referent);
    }

    pub fn exhume_associative_referent(&self, id: &Uuid) -> Option<&AssociativeReferent> {
        self.associative_referent.get(id)
    }

    pub fn inter_associative(&mut self, associative: Associative) {
        self.associative.insert(associative.id, associative);
    }

    pub fn exhume_associative(&self, id: &Uuid) -> Option<&Associative> {
        self.associative.get(id)
    }
}

pub trait WriteObjectStore
where
    Self: io::Write,
{
    fn to_json(&mut self, store: &ObjectStore) -> io::Result<()>;
}

impl<W> WriteObjectStore for W
where
    W: io::Write,
{
    fn to_json(&mut self, store: &ObjectStore) -> io::Result<()> {
        let mut serializer = serde_json::Serializer::new(self);
        store.serialize(&mut serializer)?;
        Ok(())
    }
}

pub trait ReadObjectStore
where
    Self: io::Read,
{
    fn from_json(&mut self) -> io::Result<ObjectStore>;
}

impl<R: io::Read> ReadObjectStore for R {
    fn from_json(&mut self) -> io::Result<ObjectStore> {
        let mut deserializer = serde_json::Deserializer::from_reader(self);

        Ok(ObjectStore::deserialize(&mut deserializer)?)
    }
}
