use std::fs::File;

use nut::codegen::{get_referent, get_referrer, get_subtypes, get_supertype, SarzakObjectStore};
use nut::sarzak::{Object, RelSide};
use nut::{ReadCuckooModel, Schema_v0, Schema_v1};

#[test]
fn test_drive() {
    let cuckoo = File::open("tests/cat_dog.json")
        .unwrap()
        .from_json()
        .unwrap();

    let ooa_0: Schema_v0 = cuckoo.into();
    let ooa_1: Schema_v1 = ooa_0.into();
    let store: SarzakObjectStore = ooa_1.into();

    // I see `unwrap()` in this context as a debugging aid.
    let dog = store.get_obj("Dog").unwrap();

    // I'd love something that did all this for you. Maybe a macro?
    // Getting the RelPointer is no big deal. We can match on RelPointer::side.
    // The issue is getting what's on the other side.
    // What if it were stored as a Relationship? At least then we'd have some
    // basis for extracting something.
    // Then again, maybe I'm thinking about this all wrong. Maybe I don't need to
    // worry about what's on the other side? I can get away with an impl Trait as
    // long as the trait makes sense. If I approach this backwards from my thinking,
    // there is a trait that implements some conversion, or something. We pass
    // the Object to the trait, and it does something to it? Inside out translation.
    // I need to think on this.
    let r1_ptr = dog.rels.get("R1").unwrap();
    assert_eq!(r1_ptr.side, RelSide::Subtype);
    let r1 = store.exhume_isa(&r1_ptr.value).unwrap();

    // This is going to be ugly... All that talk above is silly. This is what relationships
    // are for. I don't want to think about this this now, but what about selecting instances
    // of these things? Does that make sense? I am seriously stoned right now.
    //
    // This is MACRO FODDER!!!! Exactly what I was thinking when I was taking a shit
    // this morning. My typing is getting better. I wonder if my hand is getting better,
    // or if I'm just adapting well.
    //
    // So I wish I knew what I was thinking when I was rambling above. Maybe I really do
    // work better stoned?
    //
    // At some point, we could use relationship phrases to traverse these. That point
    // would be when we are using what this eventually generates from the sarzak ooa model.
    // Until that point, we'll find something that works.
    //
    let cat = store
        .exhume_object(
            &store
                .exhume_referent(
                    &store
                        .exhume_binary(&dog.rels.get("R2").unwrap().value)
                        .unwrap()
                        .to,
                )
                .unwrap()
                .obj_id,
        )
        .unwrap();
    assert_eq!(cat, store.get_obj("Cat").unwrap());

    let cat_2 = get_referent!(dog("R2"), store);
    assert_eq!(cat, cat_2);

    let dog_2 = get_referrer!(cat("R2"), store);
    assert_eq!(dog, dog_2);
    let r2 = cat.rels.get("R2").unwrap();
    assert_eq!(dog_2, get_referrer!(r2, store));

    // I don't know how useful this will be as it returns a relationship, which is
    // basically useless except to disambiguate relationship types.
    let r1_rel = store.get_rel("R1").unwrap();
    assert_eq!(&r1.id, r1_rel.get_id());

    // Let's try our hand at attributes...
    // Well, not much to that.
    cat.attributes.iter().for_each(|(name, a_id)| {
        let attr = store.exhume_attribute(&a_id).unwrap();
        assert_eq!(name, attr.name.inner());
    });

    // Let's make a get_supertype macro...that was easy.
    let animal = get_supertype!(cat("R1"), store);
    assert_eq!(animal, store.get_obj("Animal").unwrap());

    // And the dual macro now, but it's tricky because we need to return a list.
    // Not really all that tricky after all.
    let animals = get_subtypes!(animal("R1"), store);
    assert_eq!(animals.iter().find(|&&a| a == dog).unwrap(), &dog_2);
    assert_eq!(animals.iter().find(|&&a| a == cat).unwrap(), &cat_2);

    // Traverse a relationship with just a relationship. This is what really matters
    // during code generation, I'm finding.
    let animal = store.get_obj("Animal").unwrap();
    let r1 = animal.rels.get("R1").unwrap();
    let animals = get_subtypes!(r1.value, store);
    assert_eq!(animals.iter().find(|&&a| a == dog).unwrap(), &dog_2);
    assert_eq!(animals.iter().find(|&&a| a == cat).unwrap(), &cat_2);
}
