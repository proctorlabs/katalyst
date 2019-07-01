use super::*;
use crate::*;

pub(crate) fn initialize() {
    bind_katalyst!(@
        handlers::FileServerModule,
        handlers::HostModule,
        authentication::AlwaysAuthenticator,
        authentication::NeverAuthenticator,
        authentication::HttpAuthenticatorBuilder,
        authentication::WhitelistBuilder,
        plugins::ContentPlugin,
        cache::DefaultCacheHandler,
        cache::MemoryCacheBuilder,
        balancer::LeastConnectionBalancerBuilder,
        balancer::RandomBalancerBuilder,
        balancer::RoundRobinBalancerBuilder
    )
}
