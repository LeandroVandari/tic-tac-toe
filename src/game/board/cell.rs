use crate::Player;

/// The trait that cells for [`Board`](super::Board) implementers must have.
///
/// It allows for the generic implementations of [`Board::get_state`](super::Board::get_state) and
/// [`BoardDisplay`](super::BoardDisplay).
pub trait Cell {
    /// Returns the [`Player`] who owns the cell. If the cell is empty or a drawn/in-progress
    /// game, return [`None`].
    fn owner(&self) -> Option<&Player>;
    /// Returns the [`Cell`]'s representation as a [`char`]. Required to be able to have more
    /// nuanced representations of cells by [`super::BoardDisplay`].
    fn as_char(&self) -> char;

    /// Returns whether the cell is available, that is, whether there is no owner.
    fn is_available(&self) -> bool {
        self.owner().is_none()
    }
}
