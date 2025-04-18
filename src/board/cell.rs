use crate::Player;

/// The trait that cells for [`super::Board`] implementers must have.
///
/// It allows for the generic implementations of [`super::Board::get_state`] and
/// [`super::BoardDisplay`].
pub trait Cell {
    /// Returns the [`Player`] who owns the cell. If the cell is empty or a drawn/in-progress
    /// game, return [`None`].
    fn owner(&self) -> Option<&Player>;
    /// Returns the [`Cell`]'s representation as a [`char`]. Required to be able to have more
    /// nuanced representations of cells by [`super::BoardDisplay`].
    fn as_char(&self) -> char;
}
