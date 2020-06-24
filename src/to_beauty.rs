const SMALL_INTERVAL: u32 = ('𝒶' as u32 -'a' as u32);
const CAPITAL_INTERVAL: u32 = ('𝒜' as u32 -'A' as u32);

fn to_beauty_char(c: char)-> char
{
  let result = match c
  {
    'o' => '𝑜',
    'e' => '𝑒',
    'g' => '𝑔',
    
    'B' => 'ℬ',
    'E' => 'ℰ',
    'F' => 'ℱ',
    'H' => 'ℋ',
    'I' => 'ℐ',
    'L' => 'ℒ',
    'M' => 'ℳ',
    'R' => 'ℛ',
    
    it @ 'a' ... 'z' => 
       std::char::from_u32(
         it as u32 + SMALL_INTERVAL
       ).unwrap(),
       
    it @ 'A' ... 'Z' =>
       std::char::from_u32(
         it as u32 + CAPITAL_INTERVAL
       ).unwrap(),
    
    _ => c,
  } as u32;
  
  return std::char::from_u32(result).unwrap();
}

fn to_beauty_string(s: &str)-> String
{
  s.chars().into_iter().map(|c|{
    to_beauty_char(c)
  }).collect()
}
