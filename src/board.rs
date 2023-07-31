use thiserror::Error;

/// Defines the board on which the plants evolve
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    /// The multipliers for the fields
    pub multipliers: Multipliers,
    /// The fields of the map
    pub fields: Fields,
}

impl Board {
    /// Create a new board
    /// 
    /// # Parameters
    /// 
    /// multipliers: The multipliers for the new fields
    /// fields: The fields for the new board
    /// 
    /// # Examples
    /// 
    /// ```
    /// use evolution_plants::board;
    /// 
    /// let size = board::Size::new(2, 2);
    /// let light_field = [0.0, 0.5, 0.5, 1.0];
    /// let fields = board::Fields::new(size, &light_field).unwrap();
    /// let multipliers = board::Multipliers::new(1024);
    /// let board = board::Board::new(multipliers, fields);
    /// ```
    pub fn new(multipliers: Multipliers, fields: Fields) -> Self {
        Self { multipliers, fields }
    }
}

/// All the multipliers for the fields
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Multipliers {
    /// The multiplier for the light field
    pub light: u32,
}

impl Multipliers {
    /// Creates a new set of multipliers
    /// 
    /// # Parameters
    /// 
    /// light: The multiplier of the light field
    /// 
    /// # Examples
    /// 
    /// ```
    /// use evolution_plants::board;
    /// 
    /// let multipliers = board::Multipliers::new(1024);
    /// 
    /// assert_eq!(1024, multipliers.light);
    /// ```
    pub fn new(light: u32) -> Self {
        Self { light }
    }
}

/// All the fields
#[derive(Clone, Debug, PartialEq)]
pub struct Fields {
    /// The size of the field
    pub size: Size,
    /// The relative value of the light
    pub light: Vec<f32>,
}

impl Fields {
    /// Creates a new set of fields
    /// 
    /// Parameters 
    /// 
    /// size: The size of the board to put the fields
    /// light: The values of the light field
    /// 
    /// # Errors
    /// 
    /// BoardError::FieldSize: This will occur if any of the fields are not the correct size for the board
    /// 
    /// # Examples
    /// 
    /// ```
    /// use evolution_plants::board;
    /// 
    /// let light_field = [0.0, 1.5, 2.3, 3.9];
    /// let size = board::Size::new(2, 2);
    /// let fields = board::Fields::new(size, &light_field).unwrap();
    /// 
    /// assert_eq!([0.0, 1.5, 2.3, 3.9].to_vec(), fields.light);
    /// assert_eq!(size, fields.size);
    /// ```
    pub fn new(size: Size, light: &[f32]) -> Result<Self, FieldCreateError> {
        // Make sure the fields are the correct size
        let len = size.len();

        if light.len() != len {
            return Err(FieldCreateError::Size {name: "Light".to_string(), len: light.len(), size});
        }

        let light = light.to_vec();

        Ok(Self { size, light })
    }
}

/// The size of the map
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    /// The width of the map
    w: usize,
    /// The height of the map
    h: usize,
}

impl Size {
    /// Create a new size
    /// 
    /// # Parameters
    /// 
    /// w: The width of the size
    /// h: The height of the size
    /// 
    /// # Examples
    /// 
    /// ```
    /// use evolution_plants::board::Size;
    /// 
    /// let size = Size::new(512, 512);
    /// ```
    pub fn new(w: usize, h: usize) -> Self {
        Self { w, h }
    }

    /// Returns the size as a tuple of (w, h)
    /// 
    /// Examples
    /// 
    /// ```
    /// use evolution_plants::board::Size;
    /// 
    /// let size = Size::new(512, 256);
    /// assert_eq!((512, 256), size.size());
    /// ```
    pub fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    /// Returns the number of elements on the board
    /// 
    /// # Examples
    /// 
    /// ```
    /// use evolution_plants::board::Size;
    /// 
    /// let size = Size::new(512, 256);
    /// assert_eq!(512 * 256, size.len());
    /// ```
    pub fn len(&self) -> usize {
        self.w * self.h
    }

    /// Gets the stride of the fields for moving in the y direction
    pub(crate) fn stride(&self) -> usize {
        self.w
    }
}

#[derive(Clone, Error, Debug, PartialEq)]
pub enum FieldCreateError {
    #[error("{:?} field has wrong size ({:?}) should be ({:?}) on board with size {:?}", name, len, size.len(), size)]
    Size {
        name: String,
        len: usize,
        size: Size,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_new() {
        let size = Size::new(40, 55);
        assert_eq!((40, 55), (size.w, size.h));
    }

    #[test]
    fn size_size() {
        let size = Size::new(40, 55);
        assert_eq!((40, 55), size.size());      
    }

    #[test]
    fn size_len() {
        let size = Size::new(40, 55);
        assert_eq!(40 * 55, size.len());      
    }

    #[test]
    fn size_stride() {
        let size = Size::new(40, 55);
        assert_eq!(40, size.stride());   
    }

    #[test]
    fn fields_new() -> Result<(), FieldCreateError> {
        let size = Size::new(2, 2);
        let light_field = [1.0, 2.0, 3.0, 4.0];
        let fields = Fields::new(size, &light_field)?;

        assert_eq!(size, fields.size);
        assert_eq!(light_field.to_vec(), fields.light);

        Ok(())
    }

    #[test]
    fn fields_new_error_light() {
        let size = Size::new(2, 2);
        let light_field = [1.0, 2.0, 3.0];
        let fields = Fields::new(size, &light_field);

        assert!(fields.is_err());
        assert_eq!(FieldCreateError::Size {name: "Light".to_string(), len: 3, size}, fields.unwrap_err())
    }

    #[test]
    fn multipliers_new() {
        let multipliers = Multipliers::new(1024);

        assert_eq!(1024, multipliers.light);
    }

    #[test]
    fn board_new() {
        let size = Size::new(2, 2);
        let light_field = [1.0, 2.0, 3.0, 4.0];
        let fields = Fields::new(size, &light_field).unwrap();
        let multipliers = Multipliers::new(1024);
        let board = Board::new(multipliers, fields.clone());

        assert_eq!(multipliers, board.multipliers);
        assert_eq!(fields, board.fields);
    }
}