pub mod init;
pub mod footprint;
pub mod boot;
pub mod model;
pub mod cgroup;
pub mod supervision;
pub mod socket;
pub mod network;
pub mod perf;
pub mod config;

#[cfg(test)]
mod tests {
    mod test_boot;
    mod test_model;
    mod test_cgroup;
    mod test_supervision;
    mod test_socket;
    mod test_network;
    mod test_init;
    mod test_footprint;
    mod test_perf;
    mod test_config;
}
