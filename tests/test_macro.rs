use uuid::Uuid;

use nut::codegen::SarzakModel;
use nut::drawing::{
    get_a_across_r3, get_bui_across_r7, get_bui_across_r8, get_many_edg_across_r14,
    get_oui_across_r1, get_r_bin_across_r12, maybe_get_one_edg_across_r3, Right,
};
// use nut::sarzak::Binary;
use nut::sarzak::{get_obj_across_r17, get_r_from_across_r6, Binary};

#[test]
fn macro_test() {
    let model = SarzakModel::load_cuckoo_model("models/drawing_orig.json").unwrap();

    // We can get away with combining selections of different types because
    // they are all UUIDs. The problem is figuring out what type goes with
    // the UUID. We could get away with that if we had each type assigned to
    // a UUID, and a lookup table. Or, an enum of the types. Will I need to
    // resort to such drastic means?
    //
    // I need to do each of these separately so that I can maintain type information.
    //
    // Select all the Anchors with this edge (right)
    let anchors = get_a_across_r3!([(&Right, Right)], model);
    // println!("{:#?}", anchors);

    // Select all BUI across R7
    // For both R7 and R8 we are the Referrer, thus we have pointers. So it's
    // a simple case of comparing the input to the referential attribute we
    // store: `from` for R7 and `to` for R8.
    let mut r7 = get_bui_across_r7!(anchors, model);
    // println!("{:#?}", r7);

    // Select all BUI across R8
    let mut r8 = get_bui_across_r8!(anchors, model);
    // println!("{:#?}", r8);

    // Combine
    r7.append(&mut r8);
    let bui = r7;
    // println!("{:#?}", bui);

    // Select all R_BIN across R12
    let r_bin = get_r_bin_across_r12!(bui, model);
    // println!("{:#?}", r_bin);

    // Select all Referrer across R6
    // 💩, none of the code to traverse the below relationships is generated, and I haven't
    // written it yet. I need to think long and hard about what it's going to take to start
    // generating sarzak. I suppose I could do it into a separate domain and test things out
    // there. Maybe after I get the ObjectStore generating code working? I don't really have
    // to wait for that. I'm gonna try it now, for the fuck of it.
    let r6 = get_r_from_across_r6!(r_bin, model);
    // println!("{:#?}", r6);

    // Select all Object across R17 *
    let obj = get_obj_across_r17!(r6, model);
    // println!("{:#?}", obj);

    // Select All Referent across R5
    // Select all object across R16 *
    // Combine *
    // Select all ObjectUI across R1, with _this_ ID
    let _oui = get_oui_across_r1!(obj, model);
    // println!("{:#?}", oui);

    // Any anchors left over are connected to your edge
    //
    // Do the same for isa...ugh

    // let anchors = get_all_a_across_r3(edge, model);
    // let anchor = get_a_across_r3!(edge, model);

    // So far nothing is being tested!😵‍💫
    // Let th real testing begin!
    let oui = model.drawing.iter_object_ui().next().unwrap();
    let edges = get_many_edg_across_r14!(oui, model);
    assert_eq!(edges.len(), 4);

    let anch = model.drawing.iter_anchor().next().unwrap();
    let edg = maybe_get_one_edg_across_r3!(anch, model);
    // println!("{:?}", anch);
    // println!("{:?}", edg);
    assert!(edg.is_some());
}
