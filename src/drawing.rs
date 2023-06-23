//! The Drawing Domain
//!
//! These are the bits that belong to Drawing, the domain of drawing boxen and lines!
use uuid::{uuid, Uuid};

mod drawing;
mod object_store;

#[macro_use]
mod macros;
#[macro_use]
mod macros_2;

// Macro re-exports
pub use get_a_across_r3;
pub use get_bui_across_r7;
pub use get_bui_across_r8;
pub use get_r_bin_across_r12;
// pub use get_e_t_across_r2;
// pub use get_obj_across_r1;
pub use get_many_edg_across_r14;
pub use get_oui_across_r1;
pub use get_p_across_r13;
pub use maybe_get_one_edg_across_r3;

pub use drawing::{
    Anchor, AssociativeUI, BinaryUI, Bottom, Edge, IsaUI, Left, ObjectEdge, ObjectUI, Point,
    RelationshipUI, Right, Top,
};
pub use object_store::ObjectStore;

// ISO OID for "drawing".
pub const UUID_NS: Uuid = uuid!("0d035b17-076b-5848-b20f-ee05ce7738a9");

/// Context for JSFormat Extrude
///
/// I'm having second thoughts about SarzakModel, over in sarzak. Are we running
/// code generation on each part separately? Should we mash them together? Giving
/// this just a little thought, I think it makes sens to treat them exactly as
/// separate domains. So that means we need to invent imported entities at the
/// data level, even if they don't exist in the tool. BTW, I'm stoned as fuck.
///                         🤟🤟😎😂🤟🤟
/// So separate domains. I thought about this a few days ago. Since we are pointing
/// into sarzak, we are the ones that need to store the pointer. It's funny, this
/// is _exactly_ the stuff I've been thinking about. This is over the top maybe,
/// but what if I create a model for how this works and do codegen? 🤣🤣🤣
///
/// So, anyway, the context contains the sarzak domain. We can do id lookups and
/// all that good shit. I didn't plan it, it just seems to work out this way. And
/// for every time it works this way, there's a dozen dead ends I've explored. 🤷‍♂️
///
/// Looking at Extrude, I wonder if we couldn't use a similar trait to hang our
/// compiler directives off of? Like pass a context around with the instances? I'd
/// have to think on that. Maybe on my walk.
///
/// I realized that each domain needs its own object store. Duh. Some part of me
/// was hoping that all the work I've done would somehow translate. It will
/// eventually. But how? I need to generate the object stores. I need to generate
/// all of this handwritten library code. Not all of it. But from this point
/// forward, unless there's a more general abstraction.
///
/// Think this out. I have generated code in this project. I used the sarzak domain
/// to do that. This is the drawing domain. So I have a head start on sarzak. The
/// implementation of this domain is in terms of sarzak. So this is actually
/// already using all my hard work. Once this domain is done, we'll be able to
/// generate code for both domains. How do those domains benefit? The answer is
/// that they will be built the same way, but with more code generated, or coming
/// from the user. Which is to say generated.
///
/// I need to reorganize model/drawing. drawing should contain all of the From and
/// Extrude implementations. That reminds me, the From impls need to be in the same
/// file as the generated code. So, I can either copy and paste all of the impls
/// into the code generation script. Or, what?
use crate::codegen::{Extrude, SarzakObjectStore};
use crate::model::{extract_ooa2, JSFormat};

struct Context<'a> {
    sarzak: &'a SarzakObjectStore,
    drawing: &'a mut ObjectStore,
    id: Option<Uuid>,
}

impl From<JSFormat> for ObjectStore {
    fn from(input: JSFormat) -> Self {
        let mut store = ObjectStore::new();
        let sarzak = extract_ooa2(&input);

        let mut context = Context {
            sarzak: &sarzak,
            drawing: &mut store,
            id: None,
        };

        input.paper.entities.into_iter().next().map(|(_id, paper)| {
            paper.objects.into_iter().for_each(|(id, obj)| {
                context.id = Some(id);
                let obj = ObjectUI::extrude(obj, &mut context);
                context.drawing.inter_object_ui(obj);
            });
            paper.relationships.into_iter().for_each(|(id, rel)| {
                context.id = Some(id);
                let rel = RelationshipUI::extrude(rel, &mut context);
                context.drawing.inter_relationship_ui(rel);
            });
        });

        store
    }
}
