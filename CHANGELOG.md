# Changelog

# 0.8.3 (2023-08-29) [#67](https://github.com/TuTarea/vinted-rs/pull/67/)

## Improved
- Added support for setting the user agent

# 0.8.2 (2023-08-29) [#65](https://github.com/TuTarea/vinted-rs/pull/65/)

## Improved

- Add a default `VintedWrappers` that is `all_the_wrappers`.
 

# 0.8.1 (2023-08-23) [#62](https://github.com/TuTarea/vinted-rs/pull/62/)

## Improved
- Added support for multiple vinted wrappers.

# 0.7.0 (2023-08-20) [#59](https://github.com/TuTarea/vinted-rs/pull/59/)

## Improved
- Added support for Redis for all objects.
- Auto request cookies in get_advanced_item.

## 0.6.0 (2023-08-16) [#57](https://github.com/TuTarea/vinted-rs/pull/57/)

## Improved 
- Added support for redis bindings
- Added support for Advanced Item query thanks to @alvarocabo.

## 0.5.1 (2023-08-04) [#54](https://github.com/TuTarea/vinted-rs/pull/54/)

## Fixed
- Fix type of `price_from` and `price_to` atribute.
- Fix a test related to sizes.

## 0.5.0 (2023-08-04) [#52](https://github.com/TuTarea/vinted-rs/pull/52/)

## Improved

### Size struct

Added more fields:

- title name: Actual size, as `XL` 🇪🇸 🇫🇷 🇬🇧
- size_type: More information about the size,as `Jean's sizes` 🇪🇸 🇫🇷 🇬🇧
- category_id: Father category related

## 0.4.0 (2023-07-27) [#47](https://github.com/TuTarea/vinted-rs/pull/47/)

## Improved

- Remove `strip_option` in `Filter` struct for compatilbility.

## 0.3.3 (2023-07-26) [#45](https://github.com/TuTarea/vinted-rs/pull/45/)

## Added

- Convert VintedWrapperError to FangError support.

## 0.3.2 (2023-07-17) [#44](https://github.com/TuTarea/vinted-rs/pull/44/)

## Added

- Models now implement serde::{Serialize, Deserialize} [#43](https://github.com/TuTarea/vinted-rs/pull/43/)

## Improved

- Example project, python benchmark added [#41](https://github.com/TuTarea/vinted-rs/pull/41/)

## 0.3.1 (2023-07-15) [#42](https://github.com/TuTarea/vinted-rs/pull/42/)

### Fixed

- UK host had wrong domain [#38](https://github.com/TuTarea/vinted-rs/pull/38/)
- Not using user-agent resulted in some domains returning 403 [#38](https://github.com/TuTarea/vinted-rs/pull/38/)

### Improved

- CookieError now returns the Status Code of the requests [#38](https://github.com/TuTarea/vinted-rs/pull/38/)

## 0.3.0 (2023-07-15) [#34](<(https://github.com/TuTarea/vinted-rs/pull/34/)>)

### Added

- Filter by Currency implemented - [#32](https://github.com/TuTarea/vinted-rs/pull/32/)
- Example project using advanced filters feature - [#33](<(https://github.com/TuTarea/vinted-rs/pull/33/)>)
- CHANGELOG file

### Improved

- Documentation for `filter` module - [#35](<(https://github.com/TuTarea/vinted-rs/pull/35/)>)
