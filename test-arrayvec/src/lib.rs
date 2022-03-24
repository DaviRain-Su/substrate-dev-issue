#![cfg_attr(not(feature = "std"), no_std)]

use sp_io;
use blake2_rfc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
