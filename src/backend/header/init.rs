/// initiate header string for the first time when user is created
///
/// # Parameters
///
/// - `version_id` : Version Identifier of clog from which user is created

pub fn init(version_id: u32) -> String {
    format!("clog @{}", version_id)
}

#[cfg(test)]
mod tests {

    use super::init;

    #[test]
    fn test_init() {
        let version_id: u32 = 111;
        println!("{}", init(version_id));
    }
}
