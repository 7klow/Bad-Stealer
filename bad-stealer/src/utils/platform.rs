pub fn get_platform() -> String {
    #[cfg(target_os = "linux")]
    {
        return "Linux".to_string();
    }

    #[cfg(target_os = "windows")]
    {
        return "Windows".to_string();
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        return "Unknown".to_string();
    }
}
