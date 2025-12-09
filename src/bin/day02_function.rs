// public help function 
pub fn is_invalid(value: u64, digits: u32, divided_to: u32) -> (bool, u64) {
    // divider to get the first {divided_to} digits of {value}
    let divider = 10_u64.pow(digits-divided_to);
    // Base Case -> number already has the amount of digits its supposed to be divided_to
    if digits == divided_to {
        return (true, value);
    } else {
        // Get recursion of value without the first split part
        let recursion = is_invalid(value % divider, digits-divided_to, divided_to);
        if recursion.0 == true // check if recursion is true up to this point
        // Check if recursion is the same as the first split part
        && (value / divider) == recursion.1 {
            // return the split part
            return (true, value/divider);
        } else {
            return (false, value)
        }
    }
}