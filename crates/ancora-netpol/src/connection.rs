use crate::rule::Protocol;

#[derive(Debug, Clone)]
pub struct ConnectionRequest {
    pub tenant_id: String,
    pub source: String,
    pub destination_host: String,
    pub destination_port: u16,
    pub protocol: Protocol,
}

impl ConnectionRequest {
    pub fn new(
        tenant_id: impl Into<String>,
        source: impl Into<String>,
        destination_host: impl Into<String>,
        destination_port: u16,
        protocol: Protocol,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            source: source.into(),
            destination_host: destination_host.into(),
            destination_port,
            protocol,
        }
    }

    pub fn tcp(tenant_id: impl Into<String>, source: impl Into<String>, host: impl Into<String>, port: u16) -> Self {
        Self::new(tenant_id, source, host, port, Protocol::Tcp)
    }

    pub fn udp(tenant_id: impl Into<String>, source: impl Into<String>, host: impl Into<String>, port: u16) -> Self {
        Self::new(tenant_id, source, host, port, Protocol::Udp)
    }
}
