pub mod library;

mod pb {
    tonic::include_proto!("grit");
}

pub use self::library::Library;