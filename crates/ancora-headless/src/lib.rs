pub mod boot;
pub mod cgroup;
pub mod config;
pub mod footprint;
pub mod init;
pub mod model;
pub mod network;
pub mod perf;
pub mod socket;
pub mod supervision;

#[cfg(test)]
mod tests {
    mod test_boot;
    mod test_cgroup;
    mod test_config;
    mod test_footprint;
    mod test_init;
    mod test_model;
    mod test_network;
    mod test_perf;
    mod test_socket;
    mod test_supervision;
}
