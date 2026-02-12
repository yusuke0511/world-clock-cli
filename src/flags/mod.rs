// Flag mappings for all IANA timezone database cities
// Organized by geographic region for maintainability

mod africa;
mod america;
mod asia;
mod australia;
mod europe;
mod pacific;
mod other;

/// Get the country flag emoji for a given city name
/// 
/// # Arguments
/// * `city_name` - The city name as it appears in the timezone identifier (e.g., "New York", "Tokyo")
/// 
/// # Returns
/// The country flag emoji for the city, or 🌐 if not found
pub fn get_country_flag(city_name: &str) -> &'static str {
    // Try each region in order
    if let Some(flag) = africa::get_flag(city_name) {
        return flag;
    }
    if let Some(flag) = america::get_flag(city_name) {
        return flag;
    }
    if let Some(flag) = asia::get_flag(city_name) {
        return flag;
    }
    if let Some(flag) = australia::get_flag(city_name) {
        return flag;
    }
    if let Some(flag) = europe::get_flag(city_name) {
        return flag;
    }
    if let Some(flag) = pacific::get_flag(city_name) {
        return flag;
    }
    if let Some(flag) = other::get_flag(city_name) {
        return flag;
    }
    
    // Default: globe emoji
    "🌐"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_major_cities() {
        assert_eq!(get_country_flag("Tokyo"), "🇯🇵");
        assert_eq!(get_country_flag("New York"), "🇺🇸");
        assert_eq!(get_country_flag("London"), "🇬🇧");
        assert_eq!(get_country_flag("Paris"), "🇫🇷");
        assert_eq!(get_country_flag("Sydney"), "🇦🇺");
    }

    #[test]
    fn test_unknown_city() {
        assert_eq!(get_country_flag("UnknownCity"), "🌐");
    }
}
