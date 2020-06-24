const SMALL_INTERVAL: u32 = ('𝒶' as u32 -'a' as u32);
const CAPITAL_INTERVAL: u32 = ('𝒜' as u32 -'A' as u32);

fn to_normal_char(c: char)-> char
{
  let result = match c
  {
    '𝑜' => 'o',
    '𝑒' => 'e',
    '𝑔' => 'g',
    
    'ℬ' => 'B',
    'ℰ' => 'E',
    'ℱ' => 'F',
    'ℋ' => 'H',
    'ℐ' => 'I',
    'ℒ' => 'L',
    'ℳ' => 'M',
    'ℛ' => 'R',
    
    it @ '𝒶' ... '𝓏' =>
       std::char::from_u32(
         it as u32 - SMALL_INTERVAL
       ).unwrap(),
      
    it @ '𝒜' ... '𝒵' =>
       std::char::from_u32(
         it as u32 - CAPITAL_INTERVAL
       ).unwrap(),
       
    _ => c,
  } as u32;
  
  return std::char::from_u32(result).unwrap();
}

fn to_normal_string(s: &str)-> String
{
  s.chars().into_iter().map(|c|{
    to_normal_char(c)
  }).collect()
}
