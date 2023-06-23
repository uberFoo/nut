use std::fs::File;

use nut::sarzak::RelSide;
use nut::{ReadCuckooModel, Relationship_V1 as Relationship, Schema_v0, Schema_v1};

#[test]
fn test_consistency() {
    // The magic happens in from_schema.
    let cuckoo = File::open("tests/cat_dog.json")
        .unwrap()
        .from_json()
        .unwrap();

    let ooa_0: Schema_v0 = cuckoo.into();
    let ooa_1: Schema_v1 = ooa_0.into();

    assert!(ooa_1.obj.len() == 3);
    assert!(ooa_1.rel.len() == 2);

    assert!(ooa_1.obj.get("Cat").is_some());
    assert!(ooa_1.obj.get("Dog").is_some());
    assert!(ooa_1.obj.get("Animal").is_some());

    // TODO: Need a way to make this disappear.
    let dog_id = ooa_1.obj.get("Dog").unwrap();
    let dog = ooa_1.objects_.get(dog_id).unwrap();

    assert!(dog.rels.get("R1").is_some());
    assert!(dog.rels.get("R2").is_some());

    let cat_id = ooa_1.obj.get("Cat").unwrap();
    let cat = ooa_1.objects_.get(cat_id).unwrap();
    assert!(cat.rels.get("R1").is_some());
    assert!(cat.rels.get("R2").is_some());

    let animal_id = ooa_1.obj.get("Animal").unwrap();
    let animal = ooa_1.objects_.get(animal_id).unwrap();

    let r1_ptr = animal.rels.get("R1").unwrap();
    assert_ne!(r1_ptr, dog.rels.get("R1").unwrap());
    assert_ne!(r1_ptr, cat.rels.get("R1").unwrap());

    assert_eq!(r1_ptr.side, RelSide::Supertype);

    let r1 = ooa_1.relationships_.get(&r1_ptr.value).unwrap();

    match r1 {
        Relationship::Isa(i) => {
            assert_eq!(animal.id, i.supertype.obj_id);
            // What about the subtypes? We could iterate through the Vec of subtypes.
            // That's lame. We could store them in a hashmap and look them up.
            // What about storing them in a table of subtypes? Indexed by id, but
            // that doesn't help us find them from this subtype. So they should also
            // be accessible as a set, based on isa.id.
            // That's actually generic functionality I think:
            //   * look up item by item.id
            //     This is equivalent to directly traversing a relationship where you
            //     are formalizing the relationship, i.e., you have a direct pointer
            //     (as an attribute artifact) to what's on the other side of the
            //     relationship.
            //
            //   * look up item by item.owner.id, for some value of owner
            //     This is the inverse of above, approximately. We don't have a
            //     direct pointer to the thing on the other side of the relationship,
            //     so we look it up using our id. This is the non-formalizing case.
            //     This implies that we need to store that back-pointer in all
            //     data structures. Our model does this when we draw a relationship.
            //     Our code does not. I wonder if now might not be a good time to
            //     use some generated code? It's tempting, but I think it would
            //     be more trouble than it's worth.
        }
        Relationship::Binary(_) => panic!("should be an isa, not a binary"),
        Relationship::Associative(_) => panic!("should be an isa, not an associative"),
    }

    assert_ne!(dog.rels.get("R2"), cat.rels.get("R2"));

    let r2_ptr = dog.rels.get("R2").unwrap();
    assert_eq!(r2_ptr.side, RelSide::Referrer);

    let r2 = ooa_1.relationships_.get(&r2_ptr.value).unwrap();

    match r2 {
        Relationship::Isa(_) => panic!("should be a binary, not an isa"),
        Relationship::Binary(b) => {
            assert_eq!(b.from.obj_id, dog.id);
            assert_eq!(b.to.obj_id, cat.id);
        }
        Relationship::Associative(_) => panic!("should be a binary, not an associative"),
    }
}
