//! Test SarzakModel, plus navigating the Drawing domain.
//!
//! The first part of this test I'm doing things the "hard Way" because I don't have
//! any macros. I'm going to use this first part to implement the macros.
//!
//! The second part of the test I'm going to be using, and testing, the macros that
//! I wrote.
use nut::codegen::SarzakModel;
use nut::drawing::get_oui_across_r1;

#[test]
fn load_drawing() {
    let model = SarzakModel::load_cuckoo_model("models/cat_dog.json").unwrap();
    // dbg!(&model);

    // Looks like macro fodder. The relationship is cross domain, so numberings
    // are not unique. Also, where would this macro live? The relationship is in
    // the drawing domain, so I guess there. Let's take a stab at it:
    // `let ui = get_bui_across_r12(obj)`
    //
    // Not too bad. It's framed in terms of the domain model, which I like, and
    // should make generating it simple.
    //
    // I was thinking about making it work like `get_bui_across_r12("Dog")`, and
    // I'm not sure that it'll be very useful, since I don't imagine loading specific
    // named instances during code generation.
    let obj = model.sarzak.get_obj("Dog").unwrap();
    // Imported object
    // This is going to be tricky to implement. Maybe not tricky, but burdensome.
    // We'll need to go to the imported domain... maybe tricky after all.
    //
    // # ✨Algorithm✨
    // First off, this is just weird. In fact, it's totally backwards. This test
    // is silly anyway, because it was before I figured out what I was going
    // to do, ultimately.
    //
    // This is backwards because OBJ is imported into Drawing. OBJ would never
    // navigate across this relationship. If it were to, it would import OUI
    // from Drawing, and then do something tricky like this.
    //
    // This is interesting as a problem. You'd somehow need to get the relationship.
    // I guess you could peek at the relationship in the other domain and pick
    // the one you are involved with.
    //
    // Once you have the relationship you can grab the referential attribute
    // to figure out what you need to match your id against. Not really tricky.
    // Just weird, and not useful.
    //
    // Going the other direction you should just need to exhume the value using
    // your referential attribute as an index. Easy peasy.
    //
    let from_sarzak = model
        .drawing
        .iter_object_ui()
        .find(|o| o.1.object == obj.id)
        .unwrap();
    let ui = model.drawing.exhume_object_ui(&from_sarzak.1.id).unwrap();
    assert_eq!(ui.width, 204);
    assert_eq!(ui.height, 182);

    // I'd like to grab the Point that's associated with ObjectUI::origin. A macro
    // to do that would look like this: `let origin = get_p_across_r13(ui)`.
    //
    // But what does the code look like?
    //
    // Ugh. That was too easy. No macro needed? Which is has less cognitive load?
    // I really like the uniformity of the macro. I think that maybe we should
    // provide macros for all relationship navigation, no matter how easy. How's
    // that sound?
    let origin = model.drawing.exhume_point(&ui.origin).unwrap();
    assert_eq!(origin.x, 281);
    assert_eq!(origin.y, 574);

    // Macro
    let obj = model.sarzak.get_obj("Cat").unwrap();
    // let ui = get_oui_across_r1!(obj, model);
    let ui = get_oui_across_r1!([(&obj.id, obj)], model)
        .iter()
        .next()
        .unwrap()
        .1;
    assert_eq!(ui.width, 226);
    assert_eq!(ui.height, 181);
}
