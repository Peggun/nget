// This file is part of nget
//
// Copyright (C) 2024 Peggun
//
// nget is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// nget is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

use hickory_resolver::{
    name_server::{GenericConnector, TokioRuntimeProvider},
    AsyncResolver,
};

/// Creates a Async DNS Resolver for all platforms
pub fn build_resolver() -> AsyncResolver<GenericConnector<TokioRuntimeProvider>> {
    #[cfg(any(unix, windows))]
    {
        use hickory_resolver::{name_server::TokioConnectionProvider, TokioAsyncResolver};
        TokioAsyncResolver::from_system_conf(TokioConnectionProvider::default())
            .expect("failed to create resolver")
    }

    #[cfg(not(any(unix, windows)))]
    {
        use hickory_resolver::{
            config::{ResolverConfig, ResolverOpts},
            Resolver,
        };
        println!("Initializing Google resolver...");
        Resolver::tokio(ResolverConfig::google(), ResolverOpts::default())
    }
}
