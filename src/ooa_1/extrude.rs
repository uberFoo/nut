/// Extrude a new type
///
/// This is a pretty simple trait. It takes an input type, `T` and a reference to
/// a context, `C`, and outputs a new type, `Self`. Somehow. 😀
///
/// I have a feeling that this trait is going to be awesome.
///
/// I'm finding that this is most commonly used when interring a value into an
/// ObjectStore. The pattern is that the extrusion begins at the root of the
/// hierarchy, i.e., a Vec of stuff, T. Assuming that the T is not simple, it's
/// most likely that it's fields will need to be extruded as well.
///
/// So what you do, is extrude your children, and inter them in the store. Then
/// return yourself. Your children will do the same, and everything is happy.
///
/// In the Drawing Domain, I'm finding several classes that do not need to be
/// extruded -- only from'd.
pub trait Extrude<T, C> {
    fn extrude(input: T, context: &mut C) -> Self;
}
