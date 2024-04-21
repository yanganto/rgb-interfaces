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

use rgbstd::interface::{ContractIface, Iface, IfaceClass, IfaceId};
use rgbstd::invoice::{Amount, Precision};
use rgbstd::stl::{rgb_contract_stl, ContractTerms, Details, Name};
use rgbstd::AssetTag;
use strict_encoding::InvalidRString;
use strict_types::TypeLib;

use super::{named_contract, Features, Issue, Rgb25Info};
use crate::rgb20::iface::*;
use crate::IssuerWrapper;

pub const RGB25_BASE_IFACE_ID: IfaceId = IfaceId::from_array([
    0x83, 0x58, 0x04, 0x0e, 0xf8, 0xc8, 0x0d, 0xe2, 0x73, 0x56, 0xdf, 0x47, 0x02, 0xa4, 0xdb, 0x03,
    0x1c, 0xc0, 0xf4, 0xe3, 0x4a, 0xc6, 0x54, 0x93, 0x0f, 0xc5, 0x32, 0xbf, 0x42, 0xee, 0x20, 0x96,
]);

pub const RGB25_IFACE_ID: IfaceId = IfaceId::from_array([
    0x03, 0x0f, 0xa4, 0x2e, 0xad, 0x12, 0x54, 0xf4, 0xc9, 0xe7, 0x8b, 0x39, 0x3a, 0xb3, 0xd0, 0x39,
    0x65, 0x89, 0x16, 0xaf, 0xc5, 0x08, 0x46, 0xf4, 0x31, 0x10, 0x44, 0x9f, 0xc9, 0xfa, 0xe3, 0xdd,
]);

#[derive(Wrapper, WrapperMut, Clone, Eq, PartialEq, Debug)]
#[wrapper(Deref)]
#[wrapper_mut(DerefMut)]
pub struct Rgb25(ContractIface);

impl From<ContractIface> for Rgb25 {
    fn from(iface: ContractIface) -> Self {
        if !Rgb25::IFACE_IDS.contains(&iface.iface.iface_id) {
            panic!("the provided interface is not RGB25 interface");
        }
        Self(iface)
    }
}

impl IfaceClass for Rgb25 {
    const IFACE_NAME: &'static str = "RGB25";
    const IFACE_IDS: &'static [IfaceId] = &[RGB25_BASE_IFACE_ID, RGB25_IFACE_ID];

    type Features = Features;
    type Info = Rgb25Info;

    fn iface(features: Features) -> Iface {
        let mut iface = named_contract().expect_extended(fungible(), "RGB25Base");
        if features.renaming {
            iface = iface.expect_extended(renameable(), "RGB25Renameable");
        }
        /*
        if features.reserves {
            iface = iface.expect_extended(reservable(), "RGB25Reservable");
        }
         */
        if features.burnable {
            iface = iface.expect_extended(burnable(), "RGB25Burnable");
        }
        iface
    }

    fn iface_id(features: Self::Features) -> IfaceId { todo!() }

    fn stl() -> TypeLib { rgb_contract_stl() }

    fn info(&self) -> Self::Info { todo!() }
}

impl Rgb25 {
    pub fn testnet<C: IssuerWrapper<IssuingIface = Self>>(
        name: &str,
        precision: Precision,
    ) -> Result<Issue, InvalidRString> {
        Issue::testnet::<C>(name, precision)
    }

    pub fn testnet_det<C: IssuerWrapper<IssuingIface = Self>>(
        name: &str,
        precision: Precision,
        asset_tag: AssetTag,
    ) -> Result<Issue, InvalidRString> {
        Issue::testnet_det::<C>(name, precision, asset_tag)
    }

    pub fn name(&self) -> Name {
        let strict_val = &self
            .0
            .global("name")
            .expect("RGB25 interface requires global `name`")[0];
        Name::from_strict_val_unchecked(strict_val)
    }

    pub fn details(&self) -> Option<Details> {
        let strict_val = &self
            .0
            .global("details")
            .expect("RGB25 interface requires global `details`");
        if strict_val.len() == 0 {
            None
        } else {
            Some(Details::from_strict_val_unchecked(&strict_val[0]))
        }
    }

    pub fn precision(&self) -> Precision {
        let strict_val = &self
            .0
            .global("precision")
            .expect("RGB25 interface requires global `precision`")[0];
        Precision::from_strict_val_unchecked(strict_val)
    }

    pub fn total_issued_supply(&self) -> Amount {
        self.0
            .global("issuedSupply")
            .expect("RGB25 interface requires global `issuedSupply`")
            .iter()
            .map(Amount::from_strict_val_unchecked)
            .sum()
    }

    pub fn total_burned_supply(&self) -> Amount {
        self.0
            .global("burnedSupply")
            .unwrap_or_default()
            .iter()
            .map(Amount::from_strict_val_unchecked)
            .sum()
    }

    pub fn contract_terms(&self) -> ContractTerms {
        let strict_val = &self
            .0
            .global("terms")
            .expect("RGB25 interface requires global `terms`")[0];
        ContractTerms::from_strict_val_unchecked(strict_val)
    }
}
