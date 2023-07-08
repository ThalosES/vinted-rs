/*!
 * The `model` module provides abstraction and serialization for the request and result
 * objects of the Vinted API queries.
 */

/// The `filter` module provides structures and enums for filtering items in the Vinted API.
///
/// It contains the following modules:
/// - `brand`: Provides functionality related to filtering by brand.
/// - `category`: Provides functionality related to filtering by category.
/// - `category_tree`: Provides functionality for retrieving the category tree.
/// - `colors`: Provides functionality related to filtering by color.
/// - `country`: Provides functionality related to filtering by country.
/// - `material`: Provides functionality related to filtering by material.
/// - `size`: Provides functionality related to filtering by size.
pub mod filter;

/// The `item` module provides the `Item` struct representing an item in the Vinted API.
///
/// It depends on the `photo` module for handling item photos.
///
/// # Structs
///
/// - `Item`: Represents an item in the Vinted API.
///    - `id`: The ID of the item.
///    - `title`: The title of the item.
///    - `size_title`: The size title of the item.
///    - `brand_title`: The brand title of the item.
///    - `currency`: The currency used for the item price.
///    - `price`: The price of the item.
///    - `photo`: The photo of the item.
///    - `url`: The URL of the item.
///    - `is_visible`: A flag indicating if the item is visible.
///    - `promoted`: A flag indicating if the item is promoted.
///    - `favourite_count`: The count of favorites for the item.
pub mod item;

/// The `items` module provides the `Items` struct representing a collection of items in the Vinted API.
///
/// It depends on the `item` module for representing individual items.
///
/// # Structs
///
/// - `Items`: Represents a collection of items in the Vinted API.
///    - `items`: The list of items.
///
/// # Methods
///
/// - `new(items: Vec<Item>) -> Self`: Creates a new instance of `Items` with the provided list of items.
pub mod items;

/// The `photo` module provides the `Photo` struct representing a photo in the Vinted API.
///
/// # Structs
///
/// - `Photo`: Represents a photo in the Vinted API.
///    - `id`: The ID of the photo.
///    - `url`: The URL of the photo.
///    - `dominant_color`: The dominant color of the photo.
///    - `dominant_color_opaque`: The opaque dominant color of the photo.
pub mod photo;

/// The `User` struct represents a user in the Vinted API.
///
/// It depends on the `photo` module for handling user photos.
///
/// # Struct Fields
///
/// - `id`: The ID of the user.
/// - `login`: The username of the user.
/// - `photo`: The photo of the user.
pub mod user;
pub use serde::{Deserialize, Serialize};
