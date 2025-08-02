#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    //todo!("Convert {number:?} from base {from_base} to base {to_base}")
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    if number.len() == 0 {
        return Ok(vec![0]);
    }
    
    //converts to base 10 number
    let base10 = to_base_10(number, from_base)?;

    //if the target base is base 10, we store the digits in vec and return
    if to_base == 10u32 {
        return Ok(expand_base_10(base10));
    }

    //converts the base 10 number into target base number expanded 
    from_base_10(base10, to_base)
}

fn from_base_10(number: u32, base: u32)->Result<Vec<u32>, Error> {
    let mut result = vec![];
    let mut number = number;
    while number >= base {
        result.push(number % base);
        number /= base;
    }
    result.push(number);

    Ok(result.into_iter().rev().collect())
}

fn to_base_10(number: &[u32], base: u32)->Result<u32, Error> {
    let mut val = 0u32;
    for (i, &n) in number.iter().rev().enumerate() {
        if n >= base {
            return Err(Error::InvalidDigit(n));
        }
        val += n * base.pow(i as u32);
    }
    
    Ok(val)
}

fn expand_base_10(number: u32)->Vec<u32> {
    let mut result: Vec<u32> = vec![];
    let mut val = number;
    while val > 0 {
        let temp = val % 10;
        result.push(temp);
        val /= 10;
    }
    result.into_iter().rev().collect()
}
