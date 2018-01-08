// Generated by rust-peg. Do not edit.
use self :: RuleResult :: { Matched , Failed } ; use std::collections::HashMap; use sgf_node::*; fn escape_default ( s : & str ) -> String {
s . chars (  ) . flat_map ( | c | c . escape_default (  ) ) . collect (  ) }
fn char_range_at ( s : & str , pos : usize ) -> ( char , usize ) {
let c = & s [ pos .. ] . chars (  ) . next (  ) . unwrap (  ) ; let next_pos =
pos + c . len_utf8 (  ) ; ( * c , next_pos ) } # [ derive ( Clone ) ] enum
RuleResult < T > { Matched ( usize , T ) , Failed , } # [
derive ( PartialEq , Eq , Debug , Clone ) ] pub struct ParseError {
pub line : usize , pub column : usize , pub offset : usize , pub expected : ::
std :: collections :: HashSet < & 'static str > , } pub type ParseResult < T >
= Result < T , ParseError > ; impl :: std :: fmt :: Display for ParseError {
fn fmt ( & self , fmt : & mut :: std :: fmt :: Formatter ) -> :: std :: result
:: Result < (  ) , :: std :: fmt :: Error > {
try ! (
write ! ( fmt , "error at {}:{}: expected " , self . line , self . column ) )
; if self . expected . len (  ) == 0 { try ! ( write ! ( fmt , "EOF" ) ) ; }
else if self . expected . len (  ) == 1 {
try ! (
write ! (
fmt , "`{}`" , escape_default (
self . expected . iter (  ) . next (  ) . unwrap (  ) ) ) ) ; } else {
let mut iter = self . expected . iter (  ) ; try ! (
write ! (
fmt , "one of `{}`" , escape_default ( iter . next (  ) . unwrap (  ) ) ) ) ;
for elem in iter {
try ! ( write ! ( fmt , ", `{}`" , escape_default ( elem ) ) ) ; } } Ok ( (  )
) } } impl :: std :: error :: Error for ParseError {
fn description ( & self ) -> & str { "parse error" } } fn slice_eq (
input : & str , state : & mut ParseState , pos : usize , m : & 'static str )
-> RuleResult < (  ) > {
# ! [ inline ] # ! [ allow ( dead_code ) ] let l = m . len (  ) ; if input .
len (  ) >= pos + l && & input . as_bytes (  ) [ pos .. pos + l ] == m .
as_bytes (  ) { Matched ( pos + l , (  ) ) } else {
state . mark_failure ( pos , m ) } } fn slice_eq_case_insensitive (
input : & str , state : & mut ParseState , pos : usize , m : & 'static str )
-> RuleResult < (  ) > {
# ! [ inline ] # ! [ allow ( dead_code ) ] let mut used = 0usize ; let mut
input_iter = input [ pos .. ] . chars (  ) . flat_map (
| x | x . to_uppercase (  ) ) ; for m_char_upper in m . chars (  ) . flat_map
( | x | x . to_uppercase (  ) ) {
used += m_char_upper . len_utf8 (  ) ; let input_char_result = input_iter .
next (  ) ; if input_char_result . is_none (  ) || input_char_result . unwrap
(  ) != m_char_upper { return state . mark_failure ( pos , m ) ; } } Matched (
pos + used , (  ) ) } fn any_char (
input : & str , state : & mut ParseState , pos : usize ) -> RuleResult < (  )
> {
# ! [ inline ] # ! [ allow ( dead_code ) ] if input . len (  ) > pos {
let ( _ , next ) = char_range_at ( input , pos ) ; Matched ( next , (  ) ) }
else { state . mark_failure ( pos , "<character>" ) } } fn pos_to_line (
input : & str , pos : usize ) -> ( usize , usize ) {
let before = & input [ .. pos ] ; let line = before . as_bytes (  ) . iter (
) . filter ( | && c | c == b'\n' ) . count (  ) + 1 ; let col = before . chars
(  ) . rev (  ) . take_while ( | & c | c != '\n' ) . count (  ) + 1 ; (
line , col ) } impl < 'input > ParseState < 'input > {
fn mark_failure ( & mut self , pos : usize , expected : & 'static str ) ->
RuleResult < (  ) > {
if self . suppress_fail == 0 {
if pos > self . max_err_pos {
self . max_err_pos = pos ; self . expected . clear (  ) ; } if pos == self .
max_err_pos { self . expected . insert ( expected ) ; } } Failed } } struct ParseState < 'input > { max_err_pos : usize , suppress_fail : usize , expected : :: std :: collections :: HashSet < & 'static str > , _phantom : :: std :: marker :: PhantomData < & 'input ( ) > , } impl < 'input > ParseState < 'input > { fn new ( ) -> ParseState < 'input > { ParseState { max_err_pos : 0 , suppress_fail : 0 , expected : :: std :: collections :: HashSet :: new ( ) , _phantom : :: std :: marker :: PhantomData , } } } 

 fn __parse_collection < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < SgfCollection > { # ! [ allow ( non_snake_case , unused ) ] { let __seq_res = { let mut __repeat_pos = __pos ; let mut __repeat_value = vec ! ( ) ; loop { let __pos = __repeat_pos ; let __step_res = __parse_game_tree ( __input , __state , __pos ) ; match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; __repeat_value . push ( __value ) ; } , Failed => { break ; } } } if __repeat_value . len ( ) >= 1 { Matched ( __repeat_pos , __repeat_value ) } else { Failed } } ; match __seq_res { Matched ( __pos , gs ) => { Matched ( __pos , { 
        SgfCollection::new(gs)
     } ) } Failed => Failed , } } } 

 fn __parse_game_tree < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < SgfNode > { # ! [ allow ( non_snake_case , unused ) ] { let __seq_res = { let mut __repeat_pos = __pos ; loop { let __pos = __repeat_pos ; let __step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { ' ' | '\t' | '\r' | '\n' | 'v' => Matched ( __next , ( ) ) , _ => __state . mark_failure ( __pos , "[ \t\r\nv]" ) , } } else { __state . mark_failure ( __pos , "[ \t\r\nv]" ) } ; match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; } , Failed => { break ; } } } Matched ( __repeat_pos , ( ) ) } ; match __seq_res { Matched ( __pos , _ ) => { { let __seq_res = slice_eq ( __input , __state , __pos , "(" ) ; match __seq_res { Matched ( __pos , _ ) => { { let __seq_res = __parse_sequence ( __input , __state , __pos ) ; match __seq_res { Matched ( __pos , s ) => { { let __seq_res = { let mut __repeat_pos = __pos ; let mut __repeat_value = vec ! ( ) ; loop { let __pos = __repeat_pos ; let __step_res = __parse_game_tree ( __input , __state , __pos ) ; match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; __repeat_value . push ( __value ) ; } , Failed => { break ; } } } Matched ( __repeat_pos , __repeat_value ) } ; match __seq_res { Matched ( __pos , gs ) => { { let __seq_res = slice_eq ( __input , __state , __pos , ")" ) ; match __seq_res { Matched ( __pos , _ ) => { { let __seq_res = { let mut __repeat_pos = __pos ; loop { let __pos = __repeat_pos ; let __step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { ' ' | '\t' | '\r' | '\n' | 'v' => Matched ( __next , ( ) ) , _ => __state . mark_failure ( __pos , "[ \t\r\nv]" ) , } } else { __state . mark_failure ( __pos , "[ \t\r\nv]" ) } ; match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; } , Failed => { break ; } } } Matched ( __repeat_pos , ( ) ) } ; match __seq_res { Matched ( __pos , _ ) => { Matched ( __pos , { 
        let mut s = s;
        let mut gs = gs;
            {
            let mut l = s.leaf_mut();
            l.children.append(&mut gs);
        }
        s
     } ) } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } 

 fn __parse_sequence < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < SgfNode > { # ! [ allow ( non_snake_case , unused ) ] { let __seq_res = { let mut __repeat_pos = __pos ; loop { let __pos = __repeat_pos ; let __step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { ' ' | '\t' | '\r' | '\n' | 'v' => Matched ( __next , ( ) ) , _ => __state . mark_failure ( __pos , "[ \t\r\nv]" ) , } } else { __state . mark_failure ( __pos , "[ \t\r\nv]" ) } ; match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; } , Failed => { break ; } } } Matched ( __repeat_pos , ( ) ) } ; match __seq_res { Matched ( __pos , _ ) => { { let __seq_res = { let mut __repeat_pos = __pos ; let mut __repeat_value = vec ! ( ) ; loop { let __pos = __repeat_pos ; let __step_res = __parse_node ( __input , __state , __pos ) ; match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; __repeat_value . push ( __value ) ; } , Failed => { break ; } } } if __repeat_value . len ( ) >= 1 { Matched ( __repeat_pos , __repeat_value ) } else { Failed } } ; match __seq_res { Matched ( __pos , ns ) => { { let __seq_res = { let mut __repeat_pos = __pos ; loop { let __pos = __repeat_pos ; let __step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { ' ' | '\t' | '\r' | '\n' | 'v' => Matched ( __next , ( ) ) , _ => __state . mark_failure ( __pos , "[ \t\r\nv]" ) , } } else { __state . mark_failure ( __pos , "[ \t\r\nv]" ) } ; match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; } , Failed => { break ; } } } Matched ( __repeat_pos , ( ) ) } ; match __seq_res { Matched ( __pos , _ ) => { Matched ( __pos , { 
        let mut ns = ns;
        ns.reverse();
        let mut i = ns.into_iter();
        let mut l = i.next().unwrap();
        for mut n in i {
            n.children.push(l);
            l = n;
        }
        l
     } ) } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } 

 fn __parse_node < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < SgfNode > { # ! [ allow ( non_snake_case , unused ) ] { let __seq_res = { let mut __repeat_pos = __pos ; loop { let __pos = __repeat_pos ; let __step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { ' ' | '\t' | '\r' | '\n' | 'v' => Matched ( __next , ( ) ) , _ => __state . mark_failure ( __pos , "[ \t\r\nv]" ) , } } else { __state . mark_failure ( __pos , "[ \t\r\nv]" ) } ; match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; } , Failed => { break ; } } } Matched ( __repeat_pos , ( ) ) } ; match __seq_res { Matched ( __pos , _ ) => { { let __seq_res = slice_eq ( __input , __state , __pos , ";" ) ; match __seq_res { Matched ( __pos , _ ) => { { let __seq_res = { let mut __repeat_pos = __pos ; let mut __repeat_value = vec ! ( ) ; loop { let __pos = __repeat_pos ; let __step_res = __parse_property ( __input , __state , __pos ) ; match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; __repeat_value . push ( __value ) ; } , Failed => { break ; } } } Matched ( __repeat_pos , __repeat_value ) } ; match __seq_res { Matched ( __pos , props ) => { { let __seq_res = { let mut __repeat_pos = __pos ; loop { let __pos = __repeat_pos ; let __step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { ' ' | '\t' | '\r' | '\n' | 'v' => Matched ( __next , ( ) ) , _ => __state . mark_failure ( __pos , "[ \t\r\nv]" ) , } } else { __state . mark_failure ( __pos , "[ \t\r\nv]" ) } ; match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; } , Failed => { break ; } } } Matched ( __repeat_pos , ( ) ) } ; match __seq_res { Matched ( __pos , _ ) => { match { 
        let mut h = HashMap::new();
        let mut duplicated = false;
        for e in props {
            if h.contains_key(&e.0) {
                duplicated = true;
                break
            }
            h.insert(e.0, e.1);
        }
        if duplicated {
            Err("duplicated properties")
        } else {
            Ok(SgfNode::new(h))
        }
     } { Ok ( res ) => Matched ( __pos , res ) , Err ( expected ) => { __state . mark_failure ( __pos , expected ) ; Failed } , } } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } 

 fn __parse_property < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < (String, Vec<String>) > { # ! [ allow ( non_snake_case , unused ) ] { let __seq_res = { let mut __repeat_pos = __pos ; loop { let __pos = __repeat_pos ; let __step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { ' ' | '\t' | '\r' | '\n' | 'v' => Matched ( __next , ( ) ) , _ => __state . mark_failure ( __pos , "[ \t\r\nv]" ) , } } else { __state . mark_failure ( __pos , "[ \t\r\nv]" ) } ; match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; } , Failed => { break ; } } } Matched ( __repeat_pos , ( ) ) } ; match __seq_res { Matched ( __pos , _ ) => { { let __seq_res = __parse_prop_ident ( __input , __state , __pos ) ; match __seq_res { Matched ( __pos , i ) => { { let __seq_res = { let mut __repeat_pos = __pos ; let mut __repeat_value = vec ! ( ) ; loop { let __pos = __repeat_pos ; let __step_res = __parse_prop_value ( __input , __state , __pos ) ; match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; __repeat_value . push ( __value ) ; } , Failed => { break ; } } } if __repeat_value . len ( ) >= 1 { Matched ( __repeat_pos , __repeat_value ) } else { Failed } } ; match __seq_res { Matched ( __pos , vs ) => { { let __seq_res = { let mut __repeat_pos = __pos ; loop { let __pos = __repeat_pos ; let __step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { ' ' | '\t' | '\r' | '\n' | 'v' => Matched ( __next , ( ) ) , _ => __state . mark_failure ( __pos , "[ \t\r\nv]" ) , } } else { __state . mark_failure ( __pos , "[ \t\r\nv]" ) } ; match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; } , Failed => { break ; } } } Matched ( __repeat_pos , ( ) ) } ; match __seq_res { Matched ( __pos , _ ) => { Matched ( __pos , { 
        (i, vs)
     } ) } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } 

 fn __parse_prop_ident < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < String > { # ! [ allow ( non_snake_case , unused ) ] { let __seq_res = { let str_start = __pos ; match { let mut __repeat_pos = __pos ; let mut __repeat_value = vec ! ( ) ; loop { let __pos = __repeat_pos ; let __step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { 'A' ... 'Z' => Matched ( __next , ( ) ) , _ => __state . mark_failure ( __pos , "[A-Z]" ) , } } else { __state . mark_failure ( __pos , "[A-Z]" ) } ; match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; __repeat_value . push ( __value ) ; } , Failed => { break ; } } } if __repeat_value . len ( ) >= 1 { Matched ( __repeat_pos , ( ) ) } else { Failed } } { Matched ( __newpos , _ ) => { Matched ( __newpos , & __input [ str_start .. __newpos ] ) } , Failed => Failed , } } ; match __seq_res { Matched ( __pos , match_str ) => { Matched ( __pos , { 
        match_str.to_string()
     } ) } Failed => Failed , } } } 

 fn __parse_prop_value < 'input > ( __input : & 'input str , __state : & mut ParseState < 'input > , __pos : usize ) -> RuleResult < String > { # ! [ allow ( non_snake_case , unused ) ] { let __seq_res = { let mut __repeat_pos = __pos ; loop { let __pos = __repeat_pos ; let __step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { ' ' | '\t' | '\r' | '\n' | 'v' => Matched ( __next , ( ) ) , _ => __state . mark_failure ( __pos , "[ \t\r\nv]" ) , } } else { __state . mark_failure ( __pos , "[ \t\r\nv]" ) } ; match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; } , Failed => { break ; } } } Matched ( __repeat_pos , ( ) ) } ; match __seq_res { Matched ( __pos , _ ) => { { let __seq_res = slice_eq ( __input , __state , __pos , "[" ) ; match __seq_res { Matched ( __pos , _ ) => { { let __seq_res = { let str_start = __pos ; match { let mut __repeat_pos = __pos ; loop { let __pos = __repeat_pos ; let __step_res = { let __choice_res = slice_eq ( __input , __state , __pos , "\\]" ) ; match __choice_res { Matched ( __pos , __value ) => Matched ( __pos , __value ) , Failed => if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { ']' => __state . mark_failure ( __pos , "[^]]" ) , _ => Matched ( __next , ( ) ) , } } else { __state . mark_failure ( __pos , "[^]]" ) } } } ; match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; } , Failed => { break ; } } } Matched ( __repeat_pos , ( ) ) } { Matched ( __newpos , _ ) => { Matched ( __newpos , & __input [ str_start .. __newpos ] ) } , Failed => Failed , } } ; match __seq_res { Matched ( __pos , match_str ) => { { let __seq_res = slice_eq ( __input , __state , __pos , "]" ) ; match __seq_res { Matched ( __pos , _ ) => { { let __seq_res = { let mut __repeat_pos = __pos ; loop { let __pos = __repeat_pos ; let __step_res = if __input . len ( ) > __pos { let ( __ch , __next ) = char_range_at ( __input , __pos ) ; match __ch { ' ' | '\t' | '\r' | '\n' | 'v' => Matched ( __next , ( ) ) , _ => __state . mark_failure ( __pos , "[ \t\r\nv]" ) , } } else { __state . mark_failure ( __pos , "[ \t\r\nv]" ) } ; match __step_res { Matched ( __newpos , __value ) => { __repeat_pos = __newpos ; } , Failed => { break ; } } } Matched ( __repeat_pos , ( ) ) } ; match __seq_res { Matched ( __pos , _ ) => { Matched ( __pos , { 
        match_str.to_string()
     } ) } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } Failed => Failed , } } } 

 pub fn collection < 'input > ( __input : & 'input str ) -> ParseResult < SgfCollection > { # ! [ allow ( non_snake_case , unused ) ] let mut __state = ParseState :: new ( ) ; match __parse_collection ( __input , & mut __state , 0 ) { Matched ( __pos , __value ) => { if __pos == __input . len ( ) { return Ok ( __value ) } } _ => { } } let ( __line , __col ) = pos_to_line ( __input , __state . max_err_pos ) ; Err ( ParseError { line : __line , column : __col , offset : __state . max_err_pos , expected : __state . expected , } ) }