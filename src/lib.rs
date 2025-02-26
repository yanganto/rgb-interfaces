// RGB interfaces by LNP/BP Standards Association
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2023-2024 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2023 LNP/BP Standards Association. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_types;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_crate as serde;

mod traits;

pub mod rgb20;
pub mod rgb21;
pub mod rgb25;

pub use rgb20::Rgb20;
pub use rgb21::Rgb21;
pub use rgb25::Rgb25;
pub use traits::{IfaceWrapper, IssuerWrapper, SchemaIssuer};

pub const LNPBP_IDENTITY: &str = "lnp-bp.org";
