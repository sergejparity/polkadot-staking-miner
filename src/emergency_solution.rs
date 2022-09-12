// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! The emergency-solution command.

use crate::{opt::EmergencySolutionConfig, prelude::*};

macro_rules! emergency_cmd_for {
	($runtime:tt) => {
		paste::paste! {
			pub async fn [<run_$runtime>](
				_api: SubxtClient,
				_config: EmergencySolutionConfig
			) -> Result<(), Error> {
				todo!("not possible to implement yet");
			}
		}
	};
}

/*emergency_cmd_for!(polkadot);
emergency_cmd_for!(kusama);
emergency_cmd_for!(westend);*/
emergency_cmd_for!(kitchensink);
