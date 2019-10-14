extern crate heapless;

pub mod morse_map {

    use heapless::FnvIndexMap;
    use heapless::consts::*;
    
    pub fn get_morse_map() -> FnvIndexMap<char, &'static str, U32> {
        let mut morse_map = FnvIndexMap::<_, _, U32>::new();
        morse_map.insert(' ', " ").unwrap();
        morse_map.insert('a', ".-").unwrap();
        morse_map.insert('b', "-...").unwrap();
        morse_map.insert('c', "-.-.").unwrap();
        morse_map.insert('e', ".").unwrap();
        morse_map.insert('f', "..-.").unwrap();
        morse_map.insert('g', "--.").unwrap();
        morse_map.insert('h', "....").unwrap();
        morse_map.insert('i', "..").unwrap();
        morse_map.insert('j', ".---").unwrap();
        morse_map.insert('k', "-.-").unwrap();
        morse_map.insert('l', ".-..").unwrap();
        morse_map.insert('m', "--").unwrap();
        morse_map.insert('n', "-.").unwrap();
        morse_map.insert('o', "---").unwrap();
        morse_map.insert('p', ".--.").unwrap();
        morse_map.insert('q', "--.-").unwrap();
        morse_map.insert('r', ".-.").unwrap();
        morse_map.insert('s', "...").unwrap();
        morse_map.insert('t', "-").unwrap();
        morse_map.insert('u', "..-").unwrap();
        morse_map.insert('v', "...-").unwrap();
        morse_map.insert('w', ".--").unwrap();
        morse_map.insert('x', "-..-").unwrap();
        morse_map.insert('y', "-.--").unwrap();
        morse_map.insert('z', "--..").unwrap();
        morse_map
    }
}